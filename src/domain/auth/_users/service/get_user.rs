use dioxus::prelude::*;

use crate::domain::auth::_users::data::user::User;

/// Returns the currently authenticated user, or `None` if unauthenticated.
#[server]
pub async fn get_user() -> Result<Option<User>, ServerFnError> {
    use crate::domain::auth::auth_context::auth;

    let auth = auth().await?;
    Ok(auth.current_user)
}
