use dioxus::prelude::*;

use crate::domain::todos::data::todo::Todo;

#[server]
pub async fn get_todos() -> Result<Vec<Todo>, ServerFnError> {
    use dioxus::server::axum::Extension;
    use dioxus_fullstack::FullstackContext;
    use sqlx::PgPool;

    use crate::domain::todos::data::todos_db::TodosDb;

    let Extension(pool) = FullstackContext::extract::<Extension<PgPool>, _>().await?;
    TodosDb::get_all(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}
