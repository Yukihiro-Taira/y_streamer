/// Extract the auth session inside a `#[server]` function.
///
/// Dioxus uses `FullstackContext::extract` instead of Leptos's `use_context`.
#[cfg(feature = "server")]
pub async fn auth()
-> Result<crate::domain::auth::auth_session::AuthSession, dioxus::prelude::ServerFnError> {
    use dioxus_fullstack::FullstackContext;

    FullstackContext::extract::<crate::domain::auth::auth_session::AuthSession, _>()
        .await
        .map_err(|e| dioxus::prelude::ServerFnError::new(e.to_string()))
}

// ── axum_session_auth trait impls ─────────────────────────────────────────────
//
// These must live in this crate (orphan rules: trait + type both foreign).

#[cfg(feature = "server")]
mod impls {
    use async_trait::async_trait;
    use axum_session_auth::{Authentication, HasPermission};
    use sqlx::PgPool;
    use uuid::Uuid;

    use crate::domain::auth::_users::data::user::User;
    use crate::domain::auth::_users::data::users_db::UsersDb;

    #[async_trait]
    impl Authentication<User, Uuid, PgPool> for User {
        async fn load_user(user_id: Uuid, pool: Option<&PgPool>) -> Result<User, anyhow::Error> {
            let pool = pool.expect("Pool is None");
            UsersDb::get_by_unid(pool, user_id)
                .await
                .map_err(|e| anyhow::anyhow!("Cannot load user: {}", e))?
                .ok_or_else(|| anyhow::anyhow!("User not found"))
        }

        fn is_authenticated(&self) -> bool {
            true
        }

        fn is_active(&self) -> bool {
            true
        }

        fn is_anonymous(&self) -> bool {
            false
        }
    }

    #[async_trait]
    impl HasPermission<PgPool> for User {
        async fn has(&self, _role: &str, _pool: &Option<&PgPool>) -> bool {
            false
        }
    }
}
