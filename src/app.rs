use dioxus::prelude::*;
use icons::PanelLeft;

use crate::components::sidenav::Sidenav;
use crate::domain::auth::_users::service::get_user::get_user;
use crate::domain::auth::routing::access_demo_page::AccessDemoPage;
use crate::domain::auth::routing::dashboard_page::DashboardOverview;
use crate::domain::auth::routing::login_page::Login;
use crate::domain::auth::routing::profile_page::ProfilePage;
use crate::domain::auth::routing::users_page::UsersPage;
use crate::domain::bugreports::routing::bugreports_page::BugReportsPage;
use crate::domain::test::routing::test_page::TestPage;

const TAILWIND_CSS: Asset = asset!("/public/tailwind.css");

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/login")]
    Login {},

    #[layout(DashboardShell)]
        #[route("/")]
        Home {},

        #[route("/dashboard")]
        DashboardOverview {},

        #[route("/dashboard/users")]
        DashboardUsers {},

        #[route("/dashboard/bugreports")]
        DashboardBugReports {},

        #[route("/dashboard/profile")]
        DashboardProfile {},

        #[route("/dashboard/access-demo")]
        DashboardAccessDemo {},

    #[end_layout]

    #[route("/test-page")]
    TestPageRoute {},

    #[route("/:..route")]
    NotFound { route: Vec<String> },
}

#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

// ── Layouts ───────────────────────────────────────────────────────────────────

#[component]
fn DashboardShell() -> Element {
    let nav = use_navigator();
    let auth = use_resource(get_user);
    let mut sidebar_open = use_signal(|| true);
    let route = use_route::<Route>();

    let page_title = match route {
        Route::DashboardOverview {} => "Overview",
        Route::DashboardUsers {} => "Users",
        Route::DashboardBugReports {} => "Bug Reports",
        Route::DashboardProfile {} => "Profile",
        Route::DashboardAccessDemo {} => "Access Demo",
        _ => "Dashboard",
    };

    match auth.read().as_ref() {
        Some(Ok(Some(user))) => {
            let user = user.clone();
            rsx! {
                div { class: "flex h-screen overflow-hidden",
                    Sidenav { user: user.clone(), open: sidebar_open() }
                    div { class: "flex-1 overflow-auto flex flex-col min-w-0",
                        // ── Header ────────────────────────────────────────
                        div { class: "h-16 flex shrink-0 items-center gap-2 border-b px-4 transition-[width,height] ease-linear",
                            button {
                                class: "-ml-1 size-7 inline-flex items-center justify-center rounded-md hover:bg-accent transition-colors",
                                onclick: move |_| sidebar_open.set(!sidebar_open()),
                                PanelLeft { class: "size-4" }
                            }
                            div { class: "h-4 w-px bg-border" }
                            span { class: "text-sm font-medium", "{page_title}" }
                        }
                        // ── Page content ──────────────────────────────────
                        div { class: "flex flex-col flex-1 gap-4 p-4 pt-0 overflow-auto",
                            Outlet::<Route> {}
                        }
                    }
                }
            }
        }
        Some(Ok(None)) | Some(Err(_)) => {
            nav.push(Route::Login {});
            rsx! { div {} }
        }
        None => rsx! {
            div { class: "flex items-center justify-center h-screen",
                p { class: "text-muted-foreground text-sm", "Loading..." }
            }
        },
    }
}

// ── Pages ─────────────────────────────────────────────────────────────────────

#[component]
fn Home() -> Element {
    let nav = use_navigator();
    use_effect(move || {
        nav.push(Route::DashboardOverview {});
    });
    rsx! { div {} }
}

#[component]
fn DashboardUsers() -> Element {
    rsx! { UsersPage {} }
}

#[component]
fn DashboardBugReports() -> Element {
    rsx! { BugReportsPage {} }
}

#[component]
fn DashboardProfile() -> Element {
    rsx! { ProfilePage {} }
}

#[component]
fn DashboardAccessDemo() -> Element {
    rsx! { AccessDemoPage {} }
}

#[component]
fn TestPageRoute() -> Element {
    rsx! { TestPage {} }
}

#[component]
fn NotFound(route: Vec<String>) -> Element {
    rsx! {
        div { class: "flex items-center justify-center h-screen",
            div { class: "text-center",
                p { class: "text-4xl font-bold text-muted-foreground", "404" }
                p { class: "text-muted-foreground mt-2", "Page not found: /{route.join(\"/\")}" }
            }
        }
    }
}
