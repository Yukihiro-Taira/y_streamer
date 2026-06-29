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
// Renders a wrapper div that owns --sidenav-width CSS var + data-min/max-width.
// All children share the SidenavCtx via context.

#[component]
pub fn SidenavProvider(
    #[props(default = true)] default_open: bool,
    #[props(default = 160)] min_width: u32,
    #[props(default = 480)] max_width: u32,
    children: Element,
) -> Element {
    let open = use_signal(|| default_open);
    use_context_provider(|| SidenavCtx { open });

    rsx! {
        div {
            "data-sidenav-wrapper": true,
            "data-min-width": "{min_width}",
            "data-max-width": "{max_width}",
            style: "--sidenav-width: 256px",
            class: "flex h-full w-full",
            {children}
        }
    }
}

// ── Sidenav ───────────────────────────────────────────────────────────────────
// Width driven by --sidenav-width CSS var (set on SidenavProvider wrapper).
// Collapsed → fixed 64px override. SidenavResizeHandle handles drag + localStorage.

#[component]
pub fn Sidenav(children: Element) -> Element {
    let ctx = use_context::<SidenavCtx>();
    let open = (ctx.open)();

    let aside_style = if open {
        "width: var(--sidenav-width)"
    } else {
        "width: 64px"
    };

    rsx! {
        aside {
            class: "relative flex flex-col h-screen border-r bg-sidenav text-sidenav-foreground shrink-0 sticky top-0 overflow-hidden transition-[width] duration-200",
            style: "{aside_style}",
            {children}
        }
    }
}

// ── Resize handle ─────────────────────────────────────────────────────────────
// Place inside Sidenav on the trailing edge.
// Finds nearest [data-sidenav-wrapper], reads data-min/max-width for clamping,
// updates --sidenav-width CSS var, persists to localStorage.

#[component]
pub fn SidenavResizeHandle(#[props(default = "".to_string())] class: String) -> Element {
    use_effect(move || {
        eval(r#"(function() {
            const h = document.querySelector('[data-sidenav-resize-handle]');
            if (!h || h.dataset.initialized) return;
            h.dataset.initialized = '1';

            const wrapper = h.closest('[data-sidenav-wrapper]');
            if (!wrapper) return;

            const saved = localStorage.getItem('sidenav-width');
            if (saved) wrapper.style.setProperty('--sidenav-width', saved + 'px');

            h.addEventListener('mousedown', function(e) {
                e.preventDefault();
                document.body.style.userSelect = 'none';
                const aside = wrapper.querySelector('aside');
                const x0 = e.clientX;
                const w0 = aside ? aside.getBoundingClientRect().width : 256;
                const mn = parseInt(wrapper.dataset.minWidth) || 160;
                const mx = parseInt(wrapper.dataset.maxWidth) || 480;

                function move(e) {
                    const w = Math.max(mn, Math.min(mx, w0 + e.clientX - x0));
                    wrapper.style.setProperty('--sidenav-width', w + 'px');
                }
                function up() {
                    document.body.style.userSelect = '';
                    const aside = wrapper.querySelector('aside');
                    if (aside) localStorage.setItem('sidenav-width', aside.getBoundingClientRect().width);
                    document.removeEventListener('mousemove', move);
                    document.removeEventListener('mouseup', up);
                }
                document.addEventListener('mousemove', move);
                document.addEventListener('mouseup', up);
            });
        })();"#);
    });

    let ctx = use_context::<SidenavCtx>();
    let open = (ctx.open)();

    if !open {
        return rsx! {};
    }

    rsx! {
        div {
            "data-sidenav-resize-handle": true,
            class: "absolute top-0 right-0 w-1 h-full z-50 cursor-col-resize hover:bg-border transition-colors {class}",
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
