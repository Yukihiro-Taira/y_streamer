use dioxus::prelude::*;
use icons::{LayoutDashboard, Users};

use crate::components::ui::card::{Card, CardContent, CardHeader, CardTitle};
use crate::domain::auth::_users::service::get_user::get_user;

#[component]
pub fn DashboardOverview() -> Element {
    let user = use_resource(get_user);

    rsx! {
        div { class: "space-y-6 pt-4",
            // ── Page title ────────────────────────────────────────────────
            div { class: "flex items-center gap-2",
                LayoutDashboard { class: "size-5 text-primary" }
                h1 { class: "text-xl font-semibold", "Overview" }
            }

            // ── Stats grid ────────────────────────────────────────────────
            {match user.read().as_ref() {
                Some(Ok(Some(u))) => rsx! {
                    div { class: "grid gap-4 sm:grid-cols-2 lg:grid-cols-3",
                        // Welcome card
                        Card {
                            CardHeader {
                                div { class: "flex flex-row items-center justify-between pb-2",
                                    CardTitle { class: "text-sm font-medium text-muted-foreground",
                                        "Signed in as"
                                    }
                                    Users { class: "size-4 text-muted-foreground" }
                                }
                            }
                            CardContent {
                                p { class: "text-lg font-semibold truncate", "{u.email}" }
                                p { class: "mt-1 text-xs text-muted-foreground",
                                    {
                                        u.roles.iter()
                                            .map(|r| format!("{:?}", r.role))
                                            .collect::<Vec<_>>()
                                            .join(", ")
                                    }
                                }
                            }
                        }
                    }
                },
                None => rsx! {
                    div { class: "grid gap-4 sm:grid-cols-2 lg:grid-cols-3",
                        // Loading skeleton card
                        Card {
                            CardContent {
                                div { class: "h-4 w-32 rounded bg-muted animate-pulse mt-4" }
                                div { class: "h-3 w-24 rounded bg-muted animate-pulse mt-2" }
                            }
                        }
                    }
                },
                _ => rsx! {
                    p { class: "text-sm text-destructive", "Failed to load." }
                },
            }}
        }
    }
}
