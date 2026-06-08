pub struct BugReportsDb;

#[cfg(feature = "server")]
mod db {
    use sqlx::{PgPool, query_as};
    use time::OffsetDateTime;
    use uuid::Uuid;

    use super::*;
    use crate::domain::bugreports::data::bug_report::BugReport;

    #[derive(Debug, sqlx::FromRow)]
    struct SqlBugReport {
        unid: Uuid,
        bug_type: String,
        message: String,
        user_login: Option<String>,
        created_at: OffsetDateTime,
    }

    impl BugReportsDb {
        pub async fn get_all(pool: &PgPool) -> Result<Vec<BugReport>, sqlx::Error> {
            let rows = query_as!(
                SqlBugReport,
                r#"
                    SELECT unid, bug_type, message, user_login, created_at
                    FROM app_schema.bug_reports
                    ORDER BY created_at DESC
                    LIMIT 100
                "#
            )
            .fetch_all(pool)
            .await?;

            Ok(rows
                .into_iter()
                .map(|r| BugReport {
                    unid: r.unid,
                    bug_type: r.bug_type,
                    message: r.message,
                    user_login: r.user_login,
                    created_at: r.created_at,
                })
                .collect())
        }
    }
}
