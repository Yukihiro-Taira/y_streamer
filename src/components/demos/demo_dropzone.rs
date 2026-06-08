use dioxus::prelude::*;
use icons::{FileText, Upload};

use crate::components::ui::dropzone::FileDropzone;

// ── Types ─────────────────────────────────────────────────────────────────────

#[derive(Clone, PartialEq)]
pub struct DroppedFile {
    pub name: String,
    pub size_bytes: u64,
}

impl DroppedFile {
    pub fn size_display(&self) -> String {
        match self.size_bytes {
            b if b < 1_024 => format!("{b} B"),
            b if b < 1_048_576 => format!("{:.1} KB", b as f64 / 1_024.0),
            b if b < 1_073_741_824 => format!("{:.1} MB", b as f64 / 1_048_576.0),
            b => format!("{:.2} GB", b as f64 / 1_073_741_824.0),
        }
    }
}

// ── Hook ──────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy)]
pub struct FileDropState {
    pub files: Signal<Vec<DroppedFile>>,
    pub is_dragging: Signal<bool>,
}

pub fn use_file_drop() -> FileDropState {
    let mut files = use_signal(Vec::<DroppedFile>::new);
    let mut is_dragging = use_signal(|| false);

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
                            dropped.push(DroppedFile {
                                name: f.name(),
                                size_bytes: f.size() as u64,
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

    FileDropState { files, is_dragging }
}

// ── Demo ──────────────────────────────────────────────────────────────────────

#[component]
pub fn DemoDropzone() -> Element {
    let state = use_file_drop();
    let dragging = *state.is_dragging.read();
    let files = state.files.read();

    rsx! {
        // Full-page drag overlay
        FileDropzone { is_dragging: dragging }

        // Static drop zone hint
        div { class: "border-2 border-dashed border-border rounded-xl min-h-64 flex items-center justify-center transition-colors hover:border-primary/40 hover:bg-accent/10",
            if files.is_empty() {
                div { class: "flex flex-col items-center gap-3 text-muted-foreground select-none",
                    Upload { class: "size-10" }
                    p { class: "text-base font-medium", "Drag & drop files anywhere on this page" }
                    p { class: "text-sm", "Any file type — name and size appear below" }
                }
            } else {
                div { class: "flex flex-col items-center gap-2 text-muted-foreground select-none",
                    p { class: "text-sm font-medium",
                        "{files.len()} file(s) dropped — drag more to replace"
                    }
                }
            }
        }

        // File list
        if !files.is_empty() {
            div { class: "space-y-2",
                p { class: "text-xs font-medium text-muted-foreground uppercase tracking-wider",
                    "Dropped files"
                }
                div { class: "rounded-lg border divide-y overflow-hidden",
                    for file in files.iter() {
                        div { class: "flex items-center gap-3 px-4 py-3",
                            div { class: "size-8 rounded-md bg-muted flex items-center justify-center shrink-0",
                                FileText { class: "size-4 text-muted-foreground" }
                            }
                            span { class: "text-sm font-medium truncate flex-1 min-w-0",
                                "{file.name}"
                            }
                            span { class: "text-xs text-muted-foreground shrink-0 tabular-nums",
                                "{file.size_display()}"
                            }
                        }
                    }
                }
            }
        }
    }
}
