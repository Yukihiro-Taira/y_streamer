use dioxus::prelude::*;
use icons::{
    Bug, ChevronsUpDown, FileVideo, FlaskConical, LayoutDashboard, LogOut, ShieldCheck, UserRound,
    Users,
};

use crate::app::Route;
use crate::components::ui::sidenav::{
    Sidenav, SidenavContent, SidenavCtx, SidenavFooter, SidenavGroupLabel, SidenavHeader,
    SidenavMenu, SidenavMenuItem, SidenavResizeHandle,
};
use crate::domain::auth::_users::data::user::User;
use crate::domain::auth::_users::role::Role;
use crate::domain::auth::_users::service::logout::logout;

#[component]
pub fn AppSidenav(user: User) -> Element {
    let ctx = use_context::<SidenavCtx>();
    let open = (ctx.open)();

    let show_admin = user.has_any_role(&[Role::Root, Role::Admin]);

    let initials = user
        .email
        .chars()
        .next()
        .unwrap_or('?')
        .to_uppercase()
        .to_string();

    let role_label = user
        .roles
        .iter()
        .next()
        .map(|r| format!("{:?}", r.role))
        .unwrap_or_default();

    rsx! {
        Sidenav {
            // ── Header ────────────────────────────────────────────────────
            SidenavHeader {
                if open {
                    Link {
                        to: Route::DashboardOverview {},
                        class: "flex items-center gap-2.5 hover:opacity-80 transition-opacity flex-1 min-w-0",
                        div { class: "flex aspect-square size-8 items-center justify-center rounded-lg bg-primary text-primary-foreground text-xs font-bold shrink-0",
                            LayoutDashboard { class: "size-4" }
                        }
                        div { class: "flex flex-col gap-0.5 leading-none min-w-0",
                            span { class: "font-semibold truncate", "Auth App" }
                            span { class: "text-xs text-muted-foreground", "Dashboard" }
                        }
                    }
                } else {
                    div { class: "flex aspect-square size-8 items-center justify-center rounded-lg bg-primary text-primary-foreground text-xs font-bold mx-auto",
                        LayoutDashboard { class: "size-4" }
                    }
                }
            }

            // ── Content ───────────────────────────────────────────────────
            SidenavContent {
                SidenavMenu {
                    SidenavMenuItem {
                        NavItem { to: Route::DashboardOverview {}, label: "Overview", open,
                            LayoutDashboard { class: "size-4 shrink-0" }
                        }
                    }
                    if show_admin {
                        SidenavMenuItem {
                            NavItem { to: Route::DashboardUsers {}, label: "Users", open,
                                Users { class: "size-4 shrink-0" }
                            }
                        }
                        SidenavMenuItem {
                            NavItem { to: Route::DashboardBugReports {}, label: "Bug Reports", open,
                                Bug { class: "size-4 shrink-0" }
                            }
                        }
                    }
                    SidenavMenuItem {
                        NavItem { to: Route::DashboardAccessDemo {}, label: "Access Demo", open,
                            ShieldCheck { class: "size-4 shrink-0" }
                        }
                    }
                    SidenavMenuItem {
                        NavItem { to: Route::DashboardProfile {}, label: "Profile", open,
                            UserRound { class: "size-4 shrink-0" }
                        }
                    }
                }

                SidenavGroupLabel { "Tests" }

                SidenavMenu {
                    SidenavMenuItem {
                        NavItem { to: Route::TestPageRoute {}, label: "File Drop Test", open,
                            FlaskConical { class: "size-4 shrink-0" }
                        }
                    }
                    SidenavMenuItem {
                        NavItem { to: Route::TestWorkflowsRoute {}, label: "Workflows", open,
                            FlaskConical { class: "size-4 shrink-0" }
                        }
                    }
                    SidenavMenuItem {
                        NavItem { to: Route::TestVideoWorkflowRoute {}, label: "Video Workflow", open,
                            FileVideo { class: "size-4 shrink-0" }
                        }
                    }
                    SidenavMenuItem {
                        NavItem { to: Route::DiagnosticRoute {}, label: "Diagnostic", open,
                            FileVideo { class: "size-4 shrink-0" }
                        }
                    }
                    SidenavMenuItem {
                        NavItem { to: Route::MediaWriteRoute {}, label: "Media Write", open,
                            FileVideo { class: "size-4 shrink-0" }
                        }
                    }
                }
            }

            // ── Footer ─────────────────────────────────────────────────────
            SidenavFooter {
                NavUser { user, open, initials, role_label }
            }

            SidenavResizeHandle {}
        }
    }
}

// ── NavItem ───────────────────────────────────────────────────────────────────

#[component]
fn NavItem(to: Route, label: String, open: bool, children: Element) -> Element {
    rsx! {
        Link {
            class: "flex items-center gap-2 px-2 py-2 rounded-md text-sm text-muted-foreground hover:text-foreground hover:bg-accent transition-colors",
            active_class: "text-foreground bg-accent font-medium",
            to,
            {children}
            if open {
                span { class: "truncate", "{label}" }
            }
        }
    }
}

// ── NavUser ───────────────────────────────────────────────────────────────────

#[component]
fn NavUser(user: User, open: bool, initials: String, role_label: String) -> Element {
    let mut dropdown_open = use_signal(|| false);
    let nav = use_navigator();

    rsx! {
        div { class: "p-2 relative",
            button {
                class: "flex items-center gap-2 w-full px-2 py-2 rounded-md hover:bg-accent transition-colors",
                onclick: move |_| dropdown_open.set(!dropdown_open()),
                div { class: "size-8 rounded-lg bg-muted flex items-center justify-center text-xs font-semibold shrink-0",
                    "{initials}"
                }
                if open {
                    div { class: "grid flex-1 text-left text-sm leading-tight min-w-0",
                        span { class: "truncate text-xs font-medium", "{user.email}" }
                        span { class: "truncate text-[10px] text-muted-foreground", "{role_label}" }
                    }
                    ChevronsUpDown { class: "ml-auto size-4 text-muted-foreground shrink-0" }
                }
            }

            if dropdown_open() {
                div { class: "absolute bottom-full left-0 mb-1 min-w-56 w-max bg-card border rounded-lg shadow-md py-1 z-50",
                    div { class: "flex items-center gap-2 px-2 py-1.5 border-b mb-1",
                        div { class: "size-8 rounded-lg bg-muted flex items-center justify-center text-xs font-semibold shrink-0",
                            "{initials}"
                        }
                        div { class: "grid flex-1 text-left text-sm leading-tight min-w-0",
                            span { class: "truncate text-xs font-medium", "{user.email}" }
                            span { class: "truncate text-[10px] text-muted-foreground", "{role_label}" }
                        }
                    }
                    Link {
                        to: Route::DashboardProfile {},
                        class: "flex items-center gap-2 px-3 py-1.5 text-sm hover:bg-accent transition-colors w-full",
                        onclick: move |_| dropdown_open.set(false),
                        UserRound { class: "size-4 shrink-0" }
                        "Account"
                    }
                    div { class: "h-px bg-border my-1" }
                    button {
                        class: "flex items-center gap-2 px-3 py-1.5 text-sm hover:bg-accent transition-colors w-full text-destructive",
                        onclick: move |_| {
                            let nav = nav.clone();
                            spawn(async move {
                                let _ = logout().await;
                                nav.push(Route::Login {});
                            });
                        },
                        LogOut { class: "size-4 shrink-0" }
                        "Sign out"
                    }
                }
            }
        }
    }
}
