use std::collections::HashSet;

use app_macros::secured_server;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::components::ui::card::{Card, CardContent, CardHeader, CardTitle};
use crate::domain::auth::_users::data::user::User;
use crate::domain::auth::_users::permission::Permission;
use crate::domain::auth::_users::permission_user::UserPermission;
use crate::domain::auth::_users::role::{Role, RoleAccess};
use crate::domain::auth::access_controller::AccessController;
use crate::domain::auth::app_error::AppError;

// ── Permission type ────────────────────────────────────────────────────────────

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OnlyRootPermission {
    Access,
}

impl crate::domain::auth::_users::permission::Permission for OnlyRootPermission {
    fn roles_required(&self) -> Vec<Role> {
        match self {
            Self::Access => vec![Role::Root],
        }
    }
}

// ── Server fns ────────────────────────────────────────────────────────────────

#[secured_server]
pub async fn get_root_access_data() -> Result<String, ServerFnError> {
    Ok("You have root access!".to_string())
}

impl AccessController for GetRootAccessDataController {
    type StateParam = ();

    fn check_permission(user: &User) -> Result<(), AppError> {
        user.check_permission(&OnlyRootPermission::Access)
    }
}

#[secured_server]
pub async fn get_current_user_roles() -> Result<HashSet<RoleAccess>, ServerFnError> {
    // `user` is injected by secured_server macro above this body
    Ok(user.roles)
}

impl AccessController for GetCurrentUserRolesController {
    type StateParam = ();

    fn check_permission(_user: &User) -> Result<(), AppError> {
        Ok(())
    }
}

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

// ── Page ──────────────────────────────────────────────────────────────────────

#[component]
pub fn OnlyRootPage() -> Element {
    let access_data = use_resource(get_root_access_data);
    let user_roles = use_resource(get_current_user_roles);
    let all_users = use_resource(get_all_users);

    rsx! {
        div { class: "flex flex-col gap-8 p-6",
            h1 { class: "text-3xl font-bold", "Only Root Page" }

            // Root access check
            Card {
                CardHeader { CardTitle { "Root Access Check" } }
                CardContent {
                    match access_data.read().as_ref() {
                        None => rsx! { p { class: "text-muted-foreground text-sm", "Loading..." } },
                        Some(Ok(msg)) => rsx! {
                            p { class: "text-green-700 bg-green-100 rounded border border-green-400 px-4 py-3",
                                "{msg}"
                            }
                        },
                        Some(Err(e)) => rsx! {
                            p { class: "text-red-700 bg-red-100 rounded border border-red-400 px-4 py-3",
                                "Access denied: {e}"
                            }
                        },
                    }
                }
            }

            // Current user roles
            Card {
                CardHeader { CardTitle { "Your Current Roles" } }
                CardContent {
                    match user_roles.read().as_ref() {
                        None => rsx! { p { class: "text-muted-foreground text-sm", "Loading..." } },
                        Some(Err(e)) => rsx! {
                            p { class: "text-red-700 text-sm", "Error: {e}" }
                        },
                        Some(Ok(roles)) if roles.is_empty() => rsx! {
                            p { class: "text-yellow-700 bg-yellow-100 rounded border border-yellow-400 px-4 py-3",
                                "No roles assigned"
                            }
                        },
                        Some(Ok(roles)) => rsx! {
                            div { class: "flex flex-wrap gap-2",
                                for role_access in roles {
                                    span { class: "px-3 py-1 text-sm rounded-full bg-blue-100 text-blue-800 border border-blue-200",
                                        "{role_access.role:?}"
                                    }
                                }
                            }
                        },
                    }
                }
            }

            // All users table
            Card {
                CardHeader { CardTitle { "All Users" } }
                CardContent {
                    match all_users.read().as_ref() {
                        None => rsx! { p { class: "text-muted-foreground text-sm", "Loading..." } },
                        Some(Err(e)) => rsx! {
                            p { class: "text-red-700 text-sm", "Error: {e}" }
                        },
                        Some(Ok(users)) if users.is_empty() => rsx! {
                            p { class: "text-yellow-700 text-sm", "No users found" }
                        },
                        Some(Ok(users)) => rsx! {
                            table { class: "w-full text-sm",
                                thead {
                                    tr { class: "border-b text-left",
                                        th { class: "pb-2 pr-4", "Name" }
                                        th { class: "pb-2 pr-4", "Email" }
                                        th { class: "pb-2", "Roles" }
                                    }
                                }
                                tbody {
                                    for user in users {
                                        tr { class: "border-b last:border-0",
                                            td { class: "py-2 pr-4", "{user.full_name()}" }
                                            td { class: "py-2 pr-4 text-muted-foreground", "{user.email}" }
                                            td { class: "py-2 text-muted-foreground",
                                                {
                                                    if user.roles.is_empty() {
                                                        "—".to_string()
                                                    } else {
                                                        user.roles.iter().map(|r| format!("{:?}", r.role)).collect::<Vec<_>>().join(", ")
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        },
                    }
                }
            }
        }
    }
}
