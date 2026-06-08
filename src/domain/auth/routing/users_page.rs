use dioxus::prelude::*;

use crate::domain::auth::_users::service::list_users::get_all_users;

#[component]
pub fn UsersPage() -> Element {
    let users = use_resource(get_all_users);

    rsx! {
        div { class: "flex flex-col gap-6",
            h1 { class: "text-2xl font-bold", "Users" }

            match users.read().as_ref() {
                None => rsx! {
                    p { class: "text-muted-foreground text-sm", "Loading..." }
                },
                Some(Err(e)) => rsx! {
                    p { class: "text-destructive text-sm", "Error: {e}" }
                },
                Some(Ok(list)) if list.is_empty() => rsx! {
                    p { class: "text-muted-foreground text-sm", "No users found." }
                },
                Some(Ok(list)) => rsx! {
                    div { class: "rounded-md border",
                        table { class: "w-full text-sm",
                            thead {
                                tr { class: "border-b bg-muted/40",
                                    th { class: "px-4 py-3 text-left font-medium text-muted-foreground", "Name" }
                                    th { class: "px-4 py-3 text-left font-medium text-muted-foreground", "Email" }
                                    th { class: "px-4 py-3 text-left font-medium text-muted-foreground", "Roles" }
                                }
                            }
                            tbody {
                                for u in list {
                                    tr { class: "border-b last:border-0 hover:bg-muted/20",
                                        key: "{u.unid}",
                                        td { class: "px-4 py-3 font-medium", "{u.full_name()}" }
                                        td { class: "px-4 py-3 text-muted-foreground", "{u.email}" }
                                        td { class: "px-4 py-3 text-muted-foreground text-xs",
                                            {
                                                if u.roles.is_empty() {
                                                    "—".to_string()
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
                        }
                    }
                },
            }
        }
    }
}
