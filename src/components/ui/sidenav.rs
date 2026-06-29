use dioxus::document::eval;
use dioxus::prelude::*;
use icons::PanelLeft;

// ── Context ───────────────────────────────────────────────────────────────────

#[derive(Clone, Copy)]
pub struct SidenavCtx {
    pub open: Signal<bool>,
}

impl SidenavCtx {
    pub fn toggle(&mut self) {
        self.open.set(!(self.open)());
    }
}

// ── Provider ──────────────────────────────────────────────────────────────────

#[component]
pub fn SidenavProvider(
    #[props(default = true)] default_open: bool,
    children: Element,
) -> Element {
    let open = use_signal(|| default_open);
    use_context_provider(|| SidenavCtx { open });

    rsx! { {children} }
}

// ── Sidenav ───────────────────────────────────────────────────────────────────

#[component]
pub fn Sidenav(
    #[props(default = 160)] min_width: u32,
    #[props(default = 480)] max_width: u32,
    children: Element,
) -> Element {
    let ctx = use_context::<SidenavCtx>();
    let open = (ctx.open)();

    let aside_style = if open {
        format!("min-width:{min_width}px; max-width:{max_width}px; width:256px")
    } else {
        "width:64px".to_string()
    };

    use_effect(move || {
        if open {
            eval(&format!(
                r#"(function(){{
                    const el = document.querySelector('[data-sidenav]');
                    if (!el) return;
                    const h = el.querySelector('[data-sidenav-handle]');
                    if (!h || h.dataset.initialized) return;
                    h.dataset.initialized = '1';

                    const saved = localStorage.getItem('sidenav-width');
                    if (saved) el.style.width = saved + 'px';

                    h.addEventListener('mousedown', function(e) {{
                        e.preventDefault();
                        document.body.style.userSelect = 'none';
                        const x0 = e.clientX;
                        const w0 = el.getBoundingClientRect().width;
                        const mn = parseInt(el.dataset.minWidth);
                        const mx = parseInt(el.dataset.maxWidth);
                        function move(e) {{
                            el.style.width = Math.max(mn, Math.min(mx, w0 + e.clientX - x0)) + 'px';
                        }}
                        function up() {{
                            document.body.style.userSelect = '';
                            localStorage.setItem('sidenav-width', el.getBoundingClientRect().width);
                            document.removeEventListener('mousemove', move);
                            document.removeEventListener('mouseup', up);
                        }}
                        document.addEventListener('mousemove', move);
                        document.addEventListener('mouseup', up);
                    }});
                }})();"#
            ));
        }
    });

    rsx! {
        aside {
            class: "relative flex flex-col h-screen border-r bg-sidenav text-sidenav-foreground shrink-0 sticky top-0 overflow-hidden transition-[width] duration-200",
            style: "{aside_style}",
            "data-sidenav": true,
            "data-min-width": "{min_width}",
            "data-max-width": "{max_width}",
            {children}
            if open {
                div {
                    class: "absolute top-0 right-0 w-1 h-full z-50 cursor-col-resize hover:bg-border transition-colors",
                    "data-sidenav-handle": true,
                }
            }
        }
    }
}

// ── Trigger ───────────────────────────────────────────────────────────────────

#[component]
pub fn SidenavTrigger(#[props(default = "".to_string())] class: String) -> Element {
    let mut ctx = use_context::<SidenavCtx>();
    rsx! {
        button {
            r#type: "button",
            class: "inline-flex items-center justify-center size-7 rounded-md hover:bg-accent transition-colors {class}",
            onclick: move |_| ctx.toggle(),
            PanelLeft { class: "size-4" }
        }
    }
}

// ── Header / Content / Footer ─────────────────────────────────────────────────

#[component]
pub fn SidenavHeader(
    #[props(default = "".to_string())] class: String,
    children: Element,
) -> Element {
    rsx! {
        div { class: "h-16 flex items-center px-3 border-b shrink-0 {class}", {children} }
    }
}

#[component]
pub fn SidenavContent(
    #[props(default = "".to_string())] class: String,
    children: Element,
) -> Element {
    rsx! {
        div { class: "flex-1 overflow-y-auto {class}", {children} }
    }
}

#[component]
pub fn SidenavFooter(
    #[props(default = "".to_string())] class: String,
    children: Element,
) -> Element {
    rsx! {
        div { class: "border-t {class}", {children} }
    }
}

// ── Menu ──────────────────────────────────────────────────────────────────────

#[component]
pub fn SidenavMenu(
    #[props(default = "".to_string())] class: String,
    children: Element,
) -> Element {
    rsx! {
        nav { class: "py-2 px-2 {class}",
            ul { class: "flex flex-col gap-0.5", {children} }
        }
    }
}

#[component]
pub fn SidenavMenuItem(children: Element) -> Element {
    rsx! {
        li { {children} }
    }
}

#[component]
pub fn SidenavMenuButton(
    #[props(default = false)] active: bool,
    #[props(default = "".to_string())] class: String,
    #[props(default = None)] onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let active_cls = if active {
        "text-foreground bg-accent font-medium"
    } else {
        "text-muted-foreground hover:text-foreground hover:bg-accent"
    };

    rsx! {
        button {
            r#type: "button",
            class: "flex items-center gap-2 w-full px-2 py-2 rounded-md text-sm transition-colors {active_cls} {class}",
            onclick: move |e| { if let Some(h) = &onclick { h.call(e); } },
            {children}
        }
    }
}

// ── Group label ───────────────────────────────────────────────────────────────

#[component]
pub fn SidenavGroupLabel(
    #[props(default = "".to_string())] class: String,
    children: Element,
) -> Element {
    let ctx = use_context::<SidenavCtx>();
    if !(ctx.open)() {
        return rsx! { div { class: "h-px bg-border mx-2 my-1" } };
    }
    rsx! {
        div { class: "pt-3 pb-1 px-2 {class}",
            span { class: "text-[10px] font-semibold uppercase tracking-widest text-muted-foreground/60",
                {children}
            }
        }
    }
}

// ── Separator ─────────────────────────────────────────────────────────────────

#[component]
pub fn SidenavSeparator() -> Element {
    rsx! {
        div { class: "h-px bg-border mx-2 my-1" }
    }
}
