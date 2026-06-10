#[cfg(feature = "server")]
use sqlx::PgPool;
#[cfg(feature = "server")]
use tracing::error;

/// Fire-and-forget: log error via tracing AND persist to bug_reports table.
/// Never panics — DB failure is logged but not propagated.
#[cfg(feature = "server")]
pub fn report_server_error(pool: PgPool, bug_type: impl Into<String>, message: impl Into<String>) {
    let bug_type = bug_type.into();
    let message = message.into();
    error!(bug_type = %bug_type, message = %message, "server error reported");
    tokio::spawn(async move {
        use crate::domain::bugreports::data::bugreports_db::BugReportsDb;
        if let Err(db_err) = BugReportsDb::insert(&pool, &bug_type, &message, None).await {
            error!(db_err = %db_err, "failed to persist error report to DB");
        }
    });
}
