use dioxus::prelude::*;

use crate::components::ui::card::{Card, CardContent, CardHeader, CardTitle};
use crate::domain::todos::service::get_todos::get_todos;

/// Route: /todos — protected, shows all todos fetched from DB.
#[component]
pub fn Todos() -> Element {
    let todos = use_resource(get_todos);

    rsx! {
        div { class: "flex flex-col gap-6",
            h1 { class: "text-2xl font-bold", "Todos" }

            match todos.read().as_ref() {
                None => rsx! {
                    p { class: "text-muted-foreground text-sm", "Loading..." }
                },
                Some(Err(e)) => rsx! {
                    p { class: "text-destructive text-sm", "Error: {e}" }
                },
                Some(Ok(items)) => rsx! {
                    div { class: "flex flex-col gap-3",
                        for todo in items {
                            Card { key: "{todo.unid}",
                                CardHeader {
                                    CardTitle { "{todo.name}" }
                                }
                                CardContent {
                                    p { class: "text-muted-foreground text-xs",
                                        "Created: {todo.created_at}"
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
