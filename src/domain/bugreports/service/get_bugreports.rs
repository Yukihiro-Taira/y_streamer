use app_macros::secured_server;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::domain::auth::_users::data::user::User;
use crate::domain::auth::_users::permission::Permission;
use crate::domain::auth::_users::role::Role;
use crate::domain::auth::access_controller::AccessController;
use crate::domain::auth::app_error::AppError;
use crate::domain::bugreports::data::bug_report::BugReport;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BugReportPermission {
    View,
}

impl Permission for BugReportPermission {
    fn roles_required(&self) -> Vec<Role> {
        match self {
            Self::View => vec![Role::Root, Role::Admin],
        }
    }
}

#[secured_server]
pub async fn get_bug_reports() -> Result<Vec<BugReport>, ServerFnError> {
    use dioxus::server::axum::Extension;
    use dioxus_fullstack::FullstackContext;
    use sqlx::PgPool;

    use crate::domain::bugreports::data::bugreports_db::BugReportsDb;

    let Extension(pool) = FullstackContext::extract::<Extension<PgPool>, _>().await?;
    BugReportsDb::get_all(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

impl AccessController for GetBugReportsController {
    type StateParam = ();

    fn check_permission(user: &User) -> Result<(), AppError> {
        user.check_permission(&BugReportPermission::View)
    }
}
