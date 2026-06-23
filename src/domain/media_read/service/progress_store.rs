#[cfg(feature = "server")]
use std::collections::HashMap;
#[cfg(feature = "server")]
use std::sync::Arc;
#[cfg(feature = "server")]
use tokio::sync::RwLock;

#[cfg(feature = "server")]
use crate::domain::media_read::data::diagnostic_progress::DiagnosticProgress;

#[cfg(feature = "server")]
pub type ProgressStore = Arc<RwLock<HashMap<String, DiagnosticProgress>>>;

#[cfg(feature = "server")]
pub fn new_progress_store() -> ProgressStore {
    Arc::new(RwLock::new(HashMap::new()))
}

#[cfg(feature = "server")]
pub async fn progress_get_handler(
    axum::extract::Path(trace_id): axum::extract::Path<String>,
    dioxus::server::axum::Extension(store): dioxus::server::axum::Extension<ProgressStore>,
) -> Result<axum::Json<DiagnosticProgress>, axum::http::StatusCode> {
    let guard = store.read().await;
    match guard.get(&trace_id) {
        Some(progress) => Ok(axum::Json(progress.clone())),
        None => Err(axum::http::StatusCode::NOT_FOUND),
    }
}
