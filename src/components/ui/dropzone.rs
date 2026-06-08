use dioxus::prelude::*;
use icons::FileText;
use tw_merge::tw_merge;

// ── File type ─────────────────────────────────────────────────────────────────

#[derive(Clone, PartialEq)]
pub struct DropzoneFile {
    pub name: String,
    pub size_bytes: u64,
    /// Object URL for image preview (wasm32 only, None otherwise)
    pub preview_url: Option<String>,
}

impl DropzoneFile {
    pub fn size_display(&self) -> String {
        match self.size_bytes {
            b if b < 1_024 => format!("{b} B"),
            b if b < 1_048_576 => format!("{:.2} KB", b as f64 / 1_024.0),
            b if b < 1_073_741_824 => format!("{:.2} MB", b as f64 / 1_048_576.0),
            b => format!("{:.2} GB", b as f64 / 1_073_741_824.0),
        }
    }
}

// ── Context ───────────────────────────────────────────────────────────────────

#[derive(Clone, Copy)]
pub struct DropzoneCtx {
    pub is_dragging: Signal<bool>,
    pub files: Signal<Vec<DropzoneFile>>,
}

// ── Dropzone (root) ───────────────────────────────────────────────────────────

#[component]
pub fn Dropzone(children: Element) -> Element {
    let mut files = use_signal(Vec::<DropzoneFile>::new);
    let mut is_dragging = use_signal(|| false);

    use_context_provider(|| DropzoneCtx { files, is_dragging });

    #[cfg(target_arch = "wasm32")]
    {
        let mut drag_count = use_signal(|| 0u32);

        use_effect(move || {
            use wasm_bindgen::closure::Closure;
            use wasm_bindgen::JsCast;

            let win = web_sys::window().expect("no window");

            let on_dragover: Closure<dyn Fn(web_sys::DragEvent)> =
                Closure::new(move |e: web_sys::DragEvent| {
                    e.prevent_default();
                });

            let on_dragenter: Closure<dyn FnMut(web_sys::DragEvent)> =
                Closure::new(move |e: web_sys::DragEvent| {
                    e.prevent_default();
                    let count = *drag_count.read() + 1;
                    drag_count.set(count);
                    if count == 1 {
                        is_dragging.set(true);
                    }
                });

            let on_dragleave: Closure<dyn FnMut(web_sys::DragEvent)> =
                Closure::new(move |e: web_sys::DragEvent| {
                    e.prevent_default();
                    let count = drag_count.read().saturating_sub(1);
                    drag_count.set(count);
                    if count == 0 {
                        is_dragging.set(false);
                    }
                });

            let on_drop: Closure<dyn FnMut(web_sys::DragEvent)> =
                Closure::new(move |e: web_sys::DragEvent| {
                    e.prevent_default();
                    e.stop_propagation();
                    drag_count.set(0);
                    is_dragging.set(false);

                    let Some(dt) = e.data_transfer() else { return };
                    let Some(file_list) = dt.files() else { return };

                    let mut dropped = Vec::new();
                    for i in 0..file_list.length() {
                        if let Some(f) = file_list.item(i) {
                            let mime = f.type_();
                            let preview_url = if mime.starts_with("image/") {
                                web_sys::Url::create_object_url_with_blob(&f).ok()
                            } else {
                                None
                            };
                            dropped.push(DropzoneFile {
                                name: f.name(),
                                size_bytes: f.size() as u64,
                                preview_url,
                            });
                        }
                    }
                    files.set(dropped);
                });

            win.add_event_listener_with_callback("dragover", on_dragover.as_ref().unchecked_ref())
                .ok();
            win.add_event_listener_with_callback("dragenter", on_dragenter.as_ref().unchecked_ref())
                .ok();
            win.add_event_listener_with_callback("dragleave", on_dragleave.as_ref().unchecked_ref())
                .ok();
            win.add_event_listener_with_callback("drop", on_drop.as_ref().unchecked_ref())
                .ok();

            on_dragover.forget();
            on_dragenter.forget();
            on_dragleave.forget();
            on_drop.forget();
        });
    }

    rsx! { {children} }
}

// ── DropzoneOverlay ───────────────────────────────────────────────────────────

#[component]
pub fn DropzoneOverlay(#[props(into, optional)] class: Option<String>) -> Element {
    let ctx = use_context::<DropzoneCtx>();

    if !*ctx.is_dragging.read() {
        return rsx! {};
    }

    let merged = tw_merge!(
        "fixed inset-0 z-50 flex items-center justify-center backdrop-blur-sm bg-background/70 pointer-events-none",
        class.as_deref().unwrap_or("")
    );

    rsx! {
        div { class: "{merged}",
            div { class: "flex flex-col items-center gap-5 rounded-2xl border-2 border-dashed border-primary bg-background/80 px-20 py-14 shadow-2xl text-primary select-none",
                icons::Upload { class: "size-10 animate-bounce" }
                p { class: "text-xl font-semibold tracking-tight", "Drop files here" }
                p { class: "text-sm text-muted-foreground font-normal", "Release to upload" }
            }
        }
    }
}

// ── DropzoneArea ──────────────────────────────────────────────────────────────

#[component]
pub fn DropzoneArea(
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let ctx = use_context::<DropzoneCtx>();
    let dragging = *ctx.is_dragging.read();

    let base = if dragging {
        "w-full min-h-[200px] border border-dashed border-primary bg-primary/5 rounded-xl py-12 px-10 flex flex-col items-center justify-center gap-3 transition-colors cursor-pointer"
    } else {
        "w-full min-h-[200px] border border-dashed border-border/60 bg-accent/40 rounded-xl py-12 px-10 flex flex-col items-center justify-center gap-3 transition-colors cursor-pointer hover:border-border hover:bg-accent/60"
    };

    rsx! {
        div { class: "{tw_merge!(base, class.as_deref().unwrap_or(\"\"))}", {children} }
    }
}

// ── DropzoneIcon ──────────────────────────────────────────────────────────────

#[component]
pub fn DropzoneIcon(
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let merged = tw_merge!("text-muted-foreground", class.as_deref().unwrap_or(""));
    rsx! { div { class: "{merged}", {children} } }
}

// ── DropzoneLabel ─────────────────────────────────────────────────────────────

#[component]
pub fn DropzoneLabel(
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let merged = tw_merge!(
        "text-sm font-semibold text-foreground text-center",
        class.as_deref().unwrap_or("")
    );
    rsx! { p { class: "{merged}", {children} } }
}

// ── DropzoneHint ──────────────────────────────────────────────────────────────

#[component]
pub fn DropzoneHint(
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let merged = tw_merge!(
        "text-xs text-muted-foreground text-center",
        class.as_deref().unwrap_or("")
    );
    rsx! { p { class: "{merged}", {children} } }
}

// ── DropzoneFileList ──────────────────────────────────────────────────────────

#[component]
pub fn DropzoneFileList(#[props(into, optional)] class: Option<String>) -> Element {
    let ctx = use_context::<DropzoneCtx>();
    let files = ctx.files.read();

    if files.is_empty() {
        return rsx! {};
    }

    let merged = tw_merge!("divide-y", class.as_deref().unwrap_or(""));

    rsx! {
        div { class: "{merged}",
            for (idx, file) in files.iter().enumerate() {
                div { class: "flex items-center gap-3 py-3",
                    // Thumbnail or file icon
                    if let Some(url) = &file.preview_url {
                        img {
                            src: "{url}",
                            class: "size-10 rounded object-cover shrink-0 bg-muted",
                        }
                    } else {
                        div { class: "size-10 rounded bg-muted flex items-center justify-center shrink-0",
                            FileText { class: "size-4 text-muted-foreground" }
                        }
                    }
                    // Name + size
                    div { class: "flex flex-col flex-1 min-w-0",
                        span { class: "text-sm font-medium truncate", "{file.name}" }
                        span { class: "text-xs text-muted-foreground", "{file.size_display()}" }
                    }
                    // Remove
                    button {
                        class: "shrink-0 size-5 rounded flex items-center justify-center text-muted-foreground hover:text-foreground hover:bg-accent transition-colors text-base leading-none",
                        onclick: move |_| {
                            let mut files = ctx.files;
                            files.write().remove(idx);
                        },
                        "×"
                    }
                }
            }
        }
    }
}
