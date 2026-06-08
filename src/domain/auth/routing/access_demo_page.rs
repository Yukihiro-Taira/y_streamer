use std::collections::HashSet;

use app_macros::secured_server;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::components::ui::card::{Card, CardContent, CardHeader, CardTitle};
use crate::domain::auth::_users::data::user::User;
use crate::domain::auth::_users::permission::Permission;
use crate::domain::auth::_users::role::{Role, RoleAccess};
use crate::domain::auth::access_controller::AccessController;
use crate::domain::auth::app_error::AppError;

// ── Permissions ───────────────────────────────────────────────────────────────

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccessDemoPermission {
    RootOnly,
}

impl Permission for AccessDemoPermission {
    fn roles_required(&self) -> Vec<Role> {
        match self {
            Self::RootOnly => vec![Role::Root],
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
        user.check_permission(&AccessDemoPermission::RootOnly)
    }
}

#[secured_server]
pub async fn get_current_user_roles() -> Result<HashSet<RoleAccess>, ServerFnError> {
    Ok(user.roles)
}

impl AccessController for GetCurrentUserRolesController {
    type StateParam = ();

    fn check_permission(_user: &User) -> Result<(), AppError> {
        Ok(())
    }
}

// ── Page ──────────────────────────────────────────────────────────────────────

#[component]
pub fn AccessDemoPage() -> Element {
    let access_data = use_resource(get_root_access_data);
    let user_roles = use_resource(get_current_user_roles);

    rsx! {
        div { class: "flex flex-col gap-6",
            div {
                h1 { class: "text-2xl font-bold", "Access Demo" }
                p { class: "text-sm text-muted-foreground mt-1",
                    "Permission-based access control demonstration."
                }
            }

            Card {
                CardHeader { CardTitle { "Root-Only Endpoint" } }
                CardContent {
                    match access_data.read().as_ref() {
                        None => rsx! { p { class: "text-muted-foreground text-sm", "Loading..." } },
                        Some(Ok(msg)) => rsx! {
                            p { class: "text-sm px-4 py-3 rounded-md bg-green-100 text-green-800 border border-green-300",
                                "{msg}"
                            }
                        },
                        Some(Err(e)) => rsx! {
                            p { class: "text-sm px-4 py-3 rounded-md bg-red-100 text-red-800 border border-red-300",
                                "Access denied: {e}"
                            }
                        },
                    }
                }
            }

            Card {
                CardHeader { CardTitle { "Your Roles" } }
                CardContent {
                    match user_roles.read().as_ref() {
                        None => rsx! { p { class: "text-muted-foreground text-sm", "Loading..." } },
                        Some(Err(e)) => rsx! {
                            p { class: "text-destructive text-sm", "Error: {e}" }
                        },
                        Some(Ok(roles)) if roles.is_empty() => rsx! {
                            p { class: "text-muted-foreground text-sm", "No roles assigned." }
                        },
                        Some(Ok(roles)) => rsx! {
                            div { class: "flex flex-wrap gap-2",
                                for role_access in roles {
                                    span { class: "px-3 py-1 text-sm rounded-md bg-secondary text-secondary-foreground border",
                                        "{role_access.role:?}"
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
