#[cfg(feature = "server")]
pub type AuthSession = axum_session_auth::AuthSession<
    crate::domain::auth::_users::data::user::User,
    uuid::Uuid,
    axum_session_sqlx::SessionPgPool,
    sqlx::PgPool,
>;
