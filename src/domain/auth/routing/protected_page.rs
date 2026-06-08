use dioxus::prelude::*;

use crate::app::Route;
use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::card::{Card, CardContent, CardFooter, CardHeader, CardTitle};
use crate::domain::auth::_users::service::get_user::get_user;
use crate::domain::auth::_users::service::logout::logout;

/// Route: /protected — only reachable when authenticated (guarded by ProtectedLayout).
#[component]
pub fn Protected() -> Element {
    let nav = use_navigator();
    let auth = use_resource(get_user);

    rsx! {
        div { class: "flex justify-center items-center min-h-screen",
            Card { class: "w-full max-w-md",
                CardHeader {
                    CardTitle { "Protected Page" }
                }
                CardContent {
                    match auth.read().as_ref() {
                        Some(Ok(Some(user))) => rsx! {
                            div { class: "flex flex-col gap-2",
                                p { "Welcome, {user.full_name()}!" }
                                p { class: "text-muted-foreground text-sm", "Email: {user.email}" }
                            }
                        },
                        _ => rsx! { p { class: "text-muted-foreground text-sm", "Loading..." } },
                    }
                }
                CardFooter {
                    Button {
                        variant: ButtonVariant::Destructive,
                        onclick: move |_| {
                            let nav = nav.clone();
                            spawn(async move {
                                let _ = logout().await;
                                nav.push(Route::Login {});
                            });
                        },
                        "Logout"
                    }
                }
            }
        }
    }
}
