use dioxus::prelude::*;

use crate::domain::bugreports::service::get_bugreports::get_bug_reports;

#[component]
pub fn BugReportsPage() -> Element {
    let reports = use_resource(get_bug_reports);

    rsx! {
        div { class: "flex flex-col gap-6",
            h1 { class: "text-2xl font-bold", "Bug Reports" }

            match reports.read().as_ref() {
                None => rsx! {
                    p { class: "text-muted-foreground text-sm", "Loading..." }
                },
                Some(Err(e)) => rsx! {
                    p { class: "text-destructive text-sm", "Error: {e}" }
                },
                Some(Ok(items)) if items.is_empty() => rsx! {
                    p { class: "text-muted-foreground text-sm", "No bug reports found." }
                },
                Some(Ok(items)) => rsx! {
                    div { class: "rounded-md border",
                        table { class: "w-full text-sm",
                            thead {
                                tr { class: "border-b bg-muted/40",
                                    th { class: "px-4 py-3 text-left font-medium text-muted-foreground", "Type" }
                                    th { class: "px-4 py-3 text-left font-medium text-muted-foreground", "Message" }
                                    th { class: "px-4 py-3 text-left font-medium text-muted-foreground", "User" }
                                    th { class: "px-4 py-3 text-left font-medium text-muted-foreground", "Date" }
                                }
                            }
                            tbody {
                                for report in items {
                                    tr { class: "border-b last:border-0 hover:bg-muted/20",
                                        key: "{report.unid}",
                                        td { class: "px-4 py-3 font-mono text-xs", "{report.bug_type}" }
                                        td { class: "px-4 py-3 max-w-xs truncate text-muted-foreground", "{report.message}" }
                                        td { class: "px-4 py-3 text-muted-foreground text-xs",
                                            { report.user_login.clone().unwrap_or_else(|| "—".to_string()) }
                                        }
                                        td { class: "px-4 py-3 text-muted-foreground text-xs whitespace-nowrap",
                                            { report.created_at.date().to_string() }
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
