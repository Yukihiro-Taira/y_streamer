use app_macros::secured_server;
use dioxus::prelude::*;

use crate::domain::auth::_users::data::user::User;
use crate::domain::auth::_users::permission_user::UserPermission;
use crate::domain::auth::access_controller::AccessController;
use crate::domain::auth::app_error::AppError;

#[secured_server]
pub async fn get_all_users() -> Result<Vec<User>, ServerFnError> {
    use dioxus::server::axum::Extension;
    use dioxus_fullstack::FullstackContext;
    use sqlx::PgPool;

    use crate::domain::auth::_users::data::users_db::UsersDb;

    let Extension(pool) = FullstackContext::extract::<Extension<PgPool>, _>().await?;
    UsersDb::get_all(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

impl AccessController for GetAllUsersController {
    type StateParam = ();

    fn check_permission(user: &User) -> Result<(), AppError> {
        user.check_permission(&UserPermission::ListAll)
    }
}
