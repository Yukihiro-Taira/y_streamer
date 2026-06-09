use dioxus::prelude::*;
use icons::{FileVideo, Upload};

use crate::components::ui::dropzone::{
    Dropzone, DropzoneArea, DropzoneCtx, DropzoneHint, DropzoneIcon, DropzoneLabel,
};

#[derive(Clone, PartialEq)]
struct VideoMetadata {
    duration_seconds: f64,
    width: u32,
    height: u32,
}

#[component]
pub fn DemoVideoInspector() -> Element {
    rsx! {
        div { class: "max-w-[700px] mx-auto w-full space-y-4",
            div { class: "space-y-1",
                h2 { class: "text-base font-semibold", "Video metadata demo" }
                p { class: "text-sm text-muted-foreground",
                    "Drop a video file to inspect the basic metadata needed for the next UI step."
                }
            }
            Dropzone {
                div { class: "space-y-6",
                    DropzoneArea {
                        DropzoneIcon {
                            Upload { class: "size-7" }
                        }
                        DropzoneLabel { "Drag video files here" }
                        DropzoneHint {
                            "We display file name, mime type, size, duration, and resolution."
                        }
                    }
                    VideoInspectorResults {}
                }
            }
        }
    }
}

#[component]
fn VideoInspectorResults() -> Element {
    let ctx = use_context::<DropzoneCtx>();
    let files = ctx.files.read().clone();

    if files.is_empty() {
        return rsx! {
            div { class: "rounded-xl border border-dashed border-border/60 bg-muted/20 px-4 py-6 text-sm text-muted-foreground",
                "No files dropped yet."
            }
        };
    }

    let video_count = files
        .iter()
        .filter(|file| file.mime_type.starts_with("video/"))
        .count();
    let total_bytes = files.iter().map(|file| file.size_bytes).sum::<u64>();

    rsx! {
        div { class: "space-y-4",
            div { class: "flex flex-wrap items-center gap-3 text-sm",
                StatPill { label: "Files", value: files.len().to_string() }
                StatPill { label: "Videos", value: video_count.to_string() }
                StatPill { label: "Total size", value: format_bytes(total_bytes) }
            }
            div { class: "space-y-3",
                for file in files {
                    if file.mime_type.starts_with("video/") {
                        VideoFileCard { file }
                    } else {
                        UnsupportedFileCard { file_name: file.name, mime_type: file.mime_type }
                    }
                }
            }
        }
    }
}

#[component]
fn VideoFileCard(file: crate::components::ui::dropzone::DropzoneFile) -> Element {
    let metadata = use_signal(|| None::<VideoMetadata>);
    let extension = file_extension(&file.name);
    let source_url = file.preview_url.clone();

    rsx! {
        div { class: "rounded-2xl border border-border/60 bg-card px-4 py-4 shadow-sm",
            div { class: "flex flex-col gap-4 md:flex-row",
                div { class: "md:w-56 shrink-0 overflow-hidden rounded-xl bg-muted",
                    if let Some(url) = source_url.clone() {
                        VideoPreview {
                            url,
                            details_signal: metadata,
                        }
                    } else {
                        div { class: "flex aspect-video items-center justify-center gap-2 text-sm text-muted-foreground",
                            FileVideo { class: "size-4" }
                            span { "Preview unavailable" }
                        }
                    }
                }
                div { class: "min-w-0 flex-1 space-y-3",
                    div { class: "space-y-1",
                        h3 { class: "truncate text-sm font-semibold", "{file.name}" }
                        p { class: "text-xs text-muted-foreground",
                            "Basic browser-readable video metadata"
                        }
                    }
                    div { class: "grid gap-2 sm:grid-cols-2",
                        InfoRow { label: "Type", value: file.mime_type.clone() }
                        InfoRow { label: "Extension", value: extension }
                        InfoRow { label: "Size", value: file.size_display() }
                        if let Some(details) = metadata() {
                            InfoRow {
                                label: "Duration",
                                value: format_duration(details.duration_seconds),
                            }
                            InfoRow {
                                label: "Resolution",
                                value: format!("{} x {}", details.width, details.height),
                            }
                        } else {
                            InfoRow { label: "Duration", value: "Loading...".to_string() }
                            InfoRow { label: "Resolution", value: "Loading...".to_string() }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[component]
fn VideoPreview(url: String, details_signal: Signal<Option<VideoMetadata>>) -> Element {
    let onmounted = move |event: dioxus::prelude::MountedEvent| {
        use wasm_bindgen::JsCast;
        use wasm_bindgen::closure::Closure;

        let mounted = event.data();
        let Some(raw) = mounted.downcast::<web_sys::Element>() else {
            return;
        };
        let Some(video) = raw.dyn_ref::<web_sys::HtmlVideoElement>() else {
            return;
        };
        let video = video.clone();
        let mut details_signal = details_signal;

        let capture = video.clone();
        let on_loaded: Closure<dyn FnMut(web_sys::Event)> =
            Closure::new(move |_event: web_sys::Event| {
                details_signal.set(Some(VideoMetadata {
                    duration_seconds: capture.duration(),
                    width: capture.video_width(),
                    height: capture.video_height(),
                }));
            });

        video.set_preload("metadata");
        video
            .add_event_listener_with_callback("loadedmetadata", on_loaded.as_ref().unchecked_ref())
            .ok();
        on_loaded.forget();
    };

    rsx! {
        video {
            class: "aspect-video w-full object-cover",
            src: "{url}",
            controls: true,
            preload: "metadata",
            onmounted,
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[component]
fn VideoPreview(url: String, details_signal: Signal<Option<VideoMetadata>>) -> Element {
    let _ = details_signal;
    rsx! {
        video {
            class: "aspect-video w-full object-cover",
            src: "{url}",
            controls: true,
            preload: "metadata",
        }
    }
}

#[component]
fn UnsupportedFileCard(file_name: String, mime_type: String) -> Element {
    rsx! {
        div { class: "rounded-2xl border border-amber-300/60 bg-amber-50 px-4 py-3 text-sm text-amber-950",
            p { class: "font-medium", "{file_name}" }
            p { class: "text-xs text-amber-800",
                "Ignored because it is not a video file"
            }
            p { class: "text-xs text-amber-700", "{mime_type}" }
        }
    }
}

#[component]
fn StatPill(label: String, value: String) -> Element {
    rsx! {
        div { class: "inline-flex items-center gap-2 rounded-full border border-border/60 bg-background px-3 py-1.5",
            span { class: "text-muted-foreground", "{label}" }
            span { class: "font-medium", "{value}" }
        }
    }
}

#[component]
fn InfoRow(label: String, value: String) -> Element {
    rsx! {
        div { class: "rounded-xl bg-muted/40 px-3 py-2",
            p { class: "text-[11px] uppercase tracking-wide text-muted-foreground", "{label}" }
            p { class: "truncate text-sm font-medium", "{value}" }
        }
    }
}

fn file_extension(file_name: &str) -> String {
    file_name
        .rsplit_once('.')
        .map(|(_, ext)| ext.to_ascii_lowercase())
        .filter(|ext| !ext.is_empty())
        .unwrap_or_else(|| "unknown".into())
}

fn format_duration(seconds: f64) -> String {
    if !seconds.is_finite() || seconds.is_sign_negative() {
        return "Unknown".into();
    }

    let total_seconds = seconds.round() as u64;
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let secs = total_seconds % 60;

    if hours > 0 {
        format!("{hours:02}:{minutes:02}:{secs:02}")
    } else {
        format!("{minutes:02}:{secs:02}")
    }
}

fn format_bytes(bytes: u64) -> String {
    match bytes {
        b if b < 1_024 => format!("{b} B"),
        b if b < 1_048_576 => format!("{:.2} KB", b as f64 / 1_024.0),
        b if b < 1_073_741_824 => format!("{:.2} MB", b as f64 / 1_048_576.0),
        b => format!("{:.2} GB", b as f64 / 1_073_741_824.0),
    }
}
