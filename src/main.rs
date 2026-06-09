mod app;
mod components;
mod domain;

use app::App;

fn main() {
    #[cfg(not(feature = "server"))]
    dioxus::launch(App);

    #[cfg(feature = "server")]
    dioxus::serve(|| async {
        use axum::routing::post;
        use axum_session::{SessionConfig, SessionLayer, SessionStore};
        use axum_session_auth::{AuthConfig, AuthSessionLayer};
        use axum_session_sqlx::SessionPgPool;
        use dioxus::server::axum::Extension;
        use sqlx::PgPool;
        use sqlx::postgres::PgPoolOptions;
        use uuid::Uuid;

        use crate::domain::auth::_users::data::user::User;
        use crate::domain::media_inspector::service::{
            media_inspector_upload_handler, media_inspector_upload_limit_bytes,
        };

        dotenv::dotenv().ok();
        let _ = tracing_subscriber::fmt()
            .with_env_filter(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "video_platform_app=info,axum=info".into()),
            )
            .with_target(true)
            .with_thread_ids(true)
            .with_level(true)
            .try_init();

        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .expect("Failed to connect to database");

        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Failed to run migrations");

        let session_pool = PgPool::connect(&database_url)
            .await
            .expect("Failed to connect session pool");
        let session_table =
            std::env::var("SESSION_TABLE_NAME").unwrap_or_else(|_| "async_sessions".to_string());
        let session_config = SessionConfig::default().with_table_name(session_table);
        let auth_config = AuthConfig::<Uuid>::default();
        let session_store =
            SessionStore::<SessionPgPool>::new(Some(session_pool.into()), session_config)
                .await
                .expect("Failed to create session store");

        Ok(dioxus::server::router(App)
            .route(
                "/api/media-inspector/upload",
                post(media_inspector_upload_handler).layer(axum::extract::DefaultBodyLimit::max(
                    media_inspector_upload_limit_bytes(),
                )),
            )
            .layer(Extension(pool.clone()))
            .layer(
                AuthSessionLayer::<User, Uuid, SessionPgPool, PgPool>::new(Some(pool))
                    .with_config(auth_config),
            )
            .layer(SessionLayer::new(session_store)))
    });
}
