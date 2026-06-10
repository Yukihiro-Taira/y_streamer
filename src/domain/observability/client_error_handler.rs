#[cfg(feature = "server")]
use axum::Json;
#[cfg(feature = "server")]
use axum::http::StatusCode;
#[cfg(feature = "server")]
use dioxus::server::axum::Extension;
#[cfg(feature = "server")]
use serde::Deserialize;
#[cfg(feature = "server")]
use sqlx::PgPool;
#[cfg(feature = "server")]
use tracing::warn;

#[derive(Debug, Deserialize)]
#[cfg_attr(not(feature = "server"), allow(dead_code))]
pub struct ClientErrorReport {
    pub bug_type: String,
    pub message: String,
    pub trace_id: Option<String>,
}

#[cfg(feature = "server")]
pub async fn client_error_handler(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<ClientErrorReport>,
) -> StatusCode {
    use crate::domain::bugreports::data::bugreports_db::BugReportsDb;
    use crate::domain::observability::error_reporter::report_server_error;

    let full_message = match &payload.trace_id {
        Some(tid) => format!("[trace_id={}] {}", tid, payload.message),
        None => payload.message.clone(),
    };

    warn!(
        bug_type = %payload.bug_type,
        message = %full_message,
        "client error reported"
    );

    if let Err(db_err) = BugReportsDb::insert(&pool, &payload.bug_type, &full_message, None).await
    {
        report_server_error(pool, "db_insert_failed", db_err.to_string());
    }

    StatusCode::NO_CONTENT
}
