pub struct TodosDb;

#[cfg(feature = "server")]
mod db {
    use sqlx::{PgPool, query_as};
    use time::OffsetDateTime;
    use uuid::Uuid;

    use super::*;
    use crate::domain::todos::data::todo::Todo;

    #[derive(Debug, sqlx::FromRow)]
    struct SqlTodo {
        unid: Uuid,
        name: String,
        created_at: OffsetDateTime,
        updated_at: OffsetDateTime,
    }

    impl TodosDb {
        pub async fn get_all(pool: &PgPool) -> Result<Vec<Todo>, sqlx::Error> {
            let rows = query_as!(
                SqlTodo,
                r#"
                    SELECT unid, name, created_at, updated_at
                    FROM app_schema.todos
                    ORDER BY created_at DESC
                "#
            )
            .fetch_all(pool)
            .await?;

            Ok(rows
                .into_iter()
                .map(|r| Todo {
                    unid: r.unid,
                    name: r.name,
                    created_at: r.created_at,
                    updated_at: r.updated_at,
                })
                .collect())
        }
    }
}
