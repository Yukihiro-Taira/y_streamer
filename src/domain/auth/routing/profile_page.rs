use dioxus::prelude::*;

use crate::components::ui::card::{Card, CardContent};
use crate::domain::auth::_users::service::get_user::get_user;

#[component]
pub fn ProfilePage() -> Element {
    let user = use_resource(get_user);

    rsx! {
        div { class: "flex flex-col gap-6",
            div {
                h1 { class: "text-2xl font-bold", "Profile" }
                p { class: "text-sm text-muted-foreground mt-1", "Your account information." }
            }

            match user.read().as_ref() {
                Some(Ok(Some(u))) => rsx! {
                    Card { class: "max-w-md",
                        CardContent { class: "divide-y px-0",
                            div { class: "flex items-center gap-4 px-6 py-4",
                                span { class: "text-muted-foreground text-sm w-20 shrink-0", "Name" }
                                span { class: "font-medium", "{u.full_name()}" }
                            }
                            div { class: "flex items-center gap-4 px-6 py-4",
                                span { class: "text-muted-foreground text-sm w-20 shrink-0", "Email" }
                                span { class: "font-medium", "{u.email}" }
                            }
                            div { class: "flex items-center gap-4 px-6 py-4",
                                span { class: "text-muted-foreground text-sm w-20 shrink-0", "Role" }
                                span { class: "font-medium",
                                    {
                                        if u.roles.is_empty() {
                                            "No roles".to_string()
                                        } else {
                                            u.roles
                                                .iter()
                                                .map(|r| format!("{:?}", r.role))
                                                .collect::<Vec<_>>()
                                                .join(", ")
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                None => rsx! {
                    p { class: "text-muted-foreground text-sm", "Loading..." }
                },
                _ => rsx! {
                    p { class: "text-destructive text-sm", "Failed to load profile." }
                },
            }
        }
    }
}
