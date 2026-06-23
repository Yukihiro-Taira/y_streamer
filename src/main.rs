mod app;
mod components;
mod domain;

use app::App;

fn main() {
    #[cfg(not(feature = "server"))]
    {
        init_client_debugging();
        dioxus::launch(App);
    }

    #[cfg(feature = "server")]
    dioxus::serve(|| async {
        use axum::routing::{get, post};
        use axum_session::{SessionConfig, SessionLayer, SessionStore};
        use axum_session_auth::{AuthConfig, AuthSessionLayer};
        use axum_session_sqlx::SessionPgPool;
        use dioxus::server::axum::Extension;
        use sqlx::PgPool;
        use sqlx::postgres::PgPoolOptions;
        use uuid::Uuid;

        use crate::domain::auth::_users::data::user::User;
        use crate::domain::media_read::service::{
            media_read_start_handler, media_read_upload_handler, media_read_upload_limit_bytes,
            new_progress_store, progress_get_handler,
        };
        use crate::domain::media_write::service::{
            media_write_artifact_download_handler, media_write_compress_handler,
            media_write_transcode_handler, media_write_upload_limit_bytes,
        };
        use crate::domain::observability::client_error_handler::client_error_handler;

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

        let progress_store = new_progress_store();

        Ok(dioxus::server::router(App)
            .route(
                "/api/media-read/upload",
                post(media_read_upload_handler).layer(axum::extract::DefaultBodyLimit::max(
                    media_read_upload_limit_bytes(),
                )),
            )
            .route(
                "/api/media-read/start",
                post(media_read_start_handler).layer(axum::extract::DefaultBodyLimit::max(
                    media_read_upload_limit_bytes(),
                )),
            )
            .route(
                "/api/media-read/progress/{trace_id}",
                get(progress_get_handler),
            )
            .route(
                "/api/media-write/compress",
                post(media_write_compress_handler).layer(axum::extract::DefaultBodyLimit::max(
                    media_write_upload_limit_bytes(),
                )),
            )
            .route(
                "/api/media-write/transcode",
                post(media_write_transcode_handler).layer(axum::extract::DefaultBodyLimit::max(
                    media_write_upload_limit_bytes(),
                )),
            )
            .route(
                "/api/media-write/artifacts/{artifact_id}",
                get(media_write_artifact_download_handler),
            )
            .route("/api/errors/client", post(client_error_handler))
            .layer(Extension(progress_store))
            .layer(Extension(pool.clone()))
            .layer(
                AuthSessionLayer::<User, Uuid, SessionPgPool, PgPool>::new(Some(pool))
                    .with_config(auth_config),
            )
            .layer(SessionLayer::new(session_store)))
    });
}

#[cfg(not(feature = "server"))]
fn init_client_debugging() {
    use std::sync::Once;

    static INIT: Once = Once::new();
    INIT.call_once(|| {
        std::panic::set_hook(Box::new(|panic_info| {
            #[cfg(target_arch = "wasm32")]
            web_sys::console::error_1(&format!("panic on web client: {panic_info}").into());

            #[cfg(not(target_arch = "wasm32"))]
            eprintln!("panic on client: {panic_info}");
        }));

        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&"web client debug hook installed".into());
    });
}
