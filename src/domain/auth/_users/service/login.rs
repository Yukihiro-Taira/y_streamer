use dioxus::prelude::*;

/// Verify credentials and create a session. Navigate to "/" on the client after `Ok(())`.
#[server]
pub async fn login(email: String, password: String) -> Result<(), ServerFnError> {
    use dioxus::server::axum::Extension;
    use dioxus_fullstack::FullstackContext;

    use crate::domain::auth::_users::data::users_db::UsersDb;
    use crate::domain::auth::auth_context::auth;

    let Extension(pool) = FullstackContext::extract::<Extension<sqlx::PgPool>, _>().await?;

    let user = UsersDb::get_from_email(email, &pool)
        .await
        .ok_or_else(|| ServerFnError::new("User does not exist."))?;

    if pwhash::unix::verify(&password, &user.password) {
        let auth = auth().await?;
        auth.login_user(user.unid);
        Ok(())
    } else {
        Err(ServerFnError::new("Password does not match."))
    }
}
