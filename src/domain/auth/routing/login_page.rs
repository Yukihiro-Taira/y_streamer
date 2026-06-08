use dioxus::prelude::*;

use crate::app::Route;
use crate::components::ui::alert::{Alert, AlertDescription, AlertVariant};
use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::card::{Card, CardContent, CardHeader, CardTitle};
use crate::components::ui::input::{Input, InputType};
use crate::components::ui::label::Label;
use crate::domain::auth::_users::service::get_user::get_user;
use crate::domain::auth::_users::service::login::login;
use crate::domain::auth::_users::service::logout::logout;

/// Route: /login
#[component]
pub fn Login() -> Element {
    let nav = use_navigator();
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut error = use_signal(|| None::<String>);
    let auth = use_resource(get_user);

    let on_login = move |evt: Event<FormData>| {
        evt.prevent_default();
        let email_val = email.read().clone();
        let pass_val = password.read().clone();
        async move {
            match login(email_val, pass_val).await {
                Ok(()) => {
                    nav.push(Route::Home {});
                }
                Err(e) => error.set(Some(e.to_string())),
            }
        }
    };

    rsx! {
        div { class: "flex justify-center items-center min-h-screen",
            Card { class: "w-full max-w-md",
                CardHeader {
                    CardTitle { "Login" }
                }
                CardContent {
                    match auth.read().as_ref() {
                        Some(Ok(Some(user))) => rsx! {
                            div { class: "flex flex-col gap-4",
                                p { class: "text-sm text-muted-foreground text-center",
                                    "Logged in as {user.email}"
                                }
                                Button {
                                    variant: ButtonVariant::Destructive,
                                    class: "w-full",
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
                        },
                        _ => rsx! {
                            form { onsubmit: on_login, class: "flex flex-col gap-4",
                                if let Some(err) = error.read().as_ref() {
                                    Alert { variant: AlertVariant::Destructive,
                                        AlertDescription { "{err}" }
                                    }
                                }

                                div { class: "flex flex-col gap-1.5",
                                    Label { html_for: "email", "Email" }
                                    Input {
                                        id: "email",
                                        r#type: InputType::Email,
                                        placeholder: "you@example.com",
                                        value: email.read().clone(),
                                        required: true,
                                        oninput: move |e: FormEvent| email.set(e.value()),
                                    }
                                }

                                div { class: "flex flex-col gap-1.5",
                                    Label { html_for: "password", "Password" }
                                    Input {
                                        id: "password",
                                        r#type: InputType::Password,
                                        placeholder: "••••••••",
                                        value: password.read().clone(),
                                        required: true,
                                        oninput: move |e: FormEvent| password.set(e.value()),
                                    }
                                }

                                Button {
                                    button_type: "submit",
                                    class: "w-full",
                                    "Sign in"
                                }
                            }
                        },
                    }
                }
            }
        }
    }
}
