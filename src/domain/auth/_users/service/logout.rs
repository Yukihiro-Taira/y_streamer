use dioxus::prelude::*;

/// Destroy the current session. Navigate to "/" on the client after `Ok(())`.
#[server]
pub async fn logout() -> Result<(), ServerFnError> {
    use crate::domain::auth::auth_context::auth;

    let auth = auth().await?;
    auth.logout_user();
    Ok(())
}
