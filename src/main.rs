mod app;
mod components;
mod domain;

use app::App;

fn main() {
    #[cfg(not(feature = "server"))]
    dioxus::launch(App);

    #[cfg(feature = "server")]
    dioxus::serve(|| async {
        use axum::extract::DefaultBodyLimit;
        use axum_session::{SessionConfig, SessionLayer, SessionStore};
        use axum_session_auth::{AuthConfig, AuthSessionLayer};
        use axum_session_sqlx::SessionPgPool;
        use dioxus::server::axum::Extension;
        use sqlx::PgPool;
        use sqlx::postgres::PgPoolOptions;
        use uuid::Uuid;

        use crate::domain::auth::_users::data::user::User;

        dotenv::dotenv().ok();

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
            // Media inspection uploads send raw file bytes through a server function.
            // Raise the default body limit so moderately large local files can reach ffprobe.
            .layer(DefaultBodyLimit::max(128 * 1024 * 1024))
            .layer(Extension(pool.clone()))
            .layer(
                AuthSessionLayer::<User, Uuid, SessionPgPool, PgPool>::new(Some(pool))
                    .with_config(auth_config),
            )
            .layer(SessionLayer::new(session_store)))
    });
}
