use dioxus::html::HasFileData;
use dioxus::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::JsFuture;

use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};
#[cfg(target_arch = "wasm32")]
use crate::domain::media_read::data::media_probe_report::MediaProbeErrorResponse;
use crate::domain::media_read::data::media_probe_report::MediaProbeReport;
#[cfg(target_arch = "wasm32")]
use crate::domain::media_write::data::media_write_job::MediaWriteErrorResponse;
use crate::domain::media_write::data::media_write_job::MediaWriteResult;

#[derive(Clone, Copy, PartialEq, Eq)]
enum MediaWriteOperation {
    Compress,
    Transcode,
}

impl MediaWriteOperation {
    fn as_str(self) -> &'static str {
        match self {
            Self::Compress => "compress",
            Self::Transcode => "transcode",
        }
    }
}

#[component]
pub fn MediaWritePage() -> Element {
    let mut selected_file = use_signal(|| None::<dioxus::html::FileData>);
    let mut selected_name = use_signal(|| None::<String>);
    let mut selected_size_bytes = use_signal(|| None::<u64>);
    let mut operation = use_signal(|| MediaWriteOperation::Compress);
    let mut read_loading = use_signal(|| false);
    let mut write_loading = use_signal(|| false);
    let mut error = use_signal(|| None::<String>);
    let mut read_report = use_signal(|| None::<MediaProbeReport>);
    let mut result = use_signal(|| None::<MediaWriteResult>);
    let mut drag_active = use_signal(|| false);
    let mut debug_events = use_signal(Vec::<String>::new);

    let mut set_file = move |file: dioxus::html::FileData| {
        let file_for_state = file.clone();
        let file_name = file.name();
        push_debug_event(&mut debug_events, format!("file selected: {file_name}"));
        selected_name.set(Some(file_name.clone()));
        #[cfg(target_arch = "wasm32")]
        {
            let size_bytes = file
                .inner()
                .downcast_ref::<web_sys::File>()
                .map(|value| value.size() as u64);
            if let Some(size_bytes) = size_bytes {
                push_debug_event(
                    &mut debug_events,
                    format!("browser file handle ready: {file_name} ({size_bytes} bytes)"),
                );
            }
            selected_size_bytes.set(size_bytes);
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            selected_size_bytes.set(None);
        }
        selected_file.set(Some(file_for_state));
        error.set(None);
        read_report.set(None);
        result.set(None);
        read_loading.set(true);
        push_debug_event(&mut debug_events, "ffprobe upload started".to_string());
        spawn({
            let mut read_loading = read_loading;
            let mut error = error;
            let mut read_report = read_report;
            let mut debug_events = debug_events;
            async move {
                match inspect_media_file(file).await {
                    Ok(report) => {
                        push_debug_event(
                            &mut debug_events,
                            format!(
                                "ffprobe response parsed: trace_id={} streams={} upload_bytes={}",
                                report.trace_id, report.stream_count, report.upload_bytes
                            ),
                        );
                        read_report.set(Some(report));
                        push_debug_event(
                            &mut debug_events,
                            "report stored in component state".to_string(),
                        );
                    }
                    Err(err) => {
                        push_debug_event(&mut debug_events, format!("ffprobe failed: {err}"));
                        error.set(Some(err));
                    }
                }
                read_loading.set(false);
                push_debug_event(&mut debug_events, "ffprobe loading finished".to_string());
            }
        });
    };

    let on_input = move |evt: Event<FormData>| {
        if let Some(file) = evt.files().into_iter().next() {
            set_file(file);
        }
    };

    let on_drop = move |evt: Event<DragData>| {
        evt.prevent_default();
        drag_active.set(false);
        if let Some(file) = evt.files().into_iter().next() {
            set_file(file);
        }
    };

    let mut run_job = move |current_operation: MediaWriteOperation| {
        let maybe_file = selected_file.read().clone();
        operation.set(current_operation);
        let current_output_container = "mp4".to_string();
        let current_video_codec = "h264".to_string();
        let current_audio_codec = "aac".to_string();
        let current_crf = "23".to_string();
        let current_preset = "fast".to_string();
        let current_audio_bitrate = "128k".to_string();

        error.set(None);
        result.set(None);

        let Some(file) = maybe_file else {
            error.set(Some("Select a file first.".into()));
            return;
        };

        write_loading.set(true);
        push_debug_event(
            &mut debug_events,
            format!("{} job started", current_operation.as_str()),
        );
        spawn({
            let mut write_loading = write_loading;
            let mut error = error;
            let mut result = result;
            let mut debug_events = debug_events;
            async move {
                match upload_media_write_job(
                    file,
                    current_operation,
                    current_output_container,
                    current_video_codec,
                    current_audio_codec,
                    current_crf,
                    current_preset,
                    current_audio_bitrate,
                )
                .await
                {
                    Ok(job_result) => {
                        push_debug_event(
                            &mut debug_events,
                            format!(
                                "{} job finished: before={} after={} saved={:.2}%",
                                job_result.operation,
                                job_result.input_bytes,
                                job_result.output_bytes,
                                job_result.saved_percent
                            ),
                        );
                        result.set(Some(job_result));
                    }
                    Err(err) => {
                        push_debug_event(&mut debug_events, format!("write job failed: {err}"));
                        error.set(Some(err));
                    }
                }
                write_loading.set(false);
                push_debug_event(&mut debug_events, "write loading finished".to_string());
            }
        });
    };

    rsx! {
        div { class: "max-w-[1100px] mx-auto w-full px-6 py-8 space-y-6",
            div { class: "space-y-2",
                h1 { class: "text-xl font-semibold", "Media Write" }
                p { class: "text-sm text-muted-foreground",
                    "Real ffmpeg jobs for server-side compress and transcode operations."
                }
            }

            Card {
                CardHeader {
                    CardTitle { "Create Job" }
                CardDescription {
                        "Drop one file. The app probes it immediately, shows the raw ffprobe JSON, then unlocks the write actions."
                    }
                }
                CardContent { class: "space-y-4",
                    div {
                        class: if drag_active() {
                            "rounded-2xl border-2 border-dashed border-primary bg-primary/5 px-6 py-8 text-center transition-colors"
                        } else {
                            "rounded-2xl border-2 border-dashed border-border bg-muted/20 px-6 py-8 text-center transition-colors"
                        },
                        ondragover: move |evt| {
                            evt.prevent_default();
                            drag_active.set(true);
                        },
                        ondragleave: move |_| drag_active.set(false),
                        ondrop: on_drop,
                        p { class: "text-sm font-medium", "Drop a media file here" }
                        p { class: "mt-1 text-xs text-muted-foreground", "or choose one below" }
                        input {
                            class: "mt-4 block w-full text-sm",
                            r#type: "file",
                            accept: "video/*,.mov,.mp4,.mkv,.webm,.mxf",
                            onchange: on_input,
                        }
                    }

                    if let Some(name) = selected_name() {
                        div { class: "rounded-xl border bg-muted/20 p-4 space-y-3",
                            div { class: "flex flex-col gap-1 md:flex-row md:items-start md:justify-between",
                                div {
                                    p { class: "text-sm font-medium", "{name}" }
                                    if let Some(size_bytes) = selected_size_bytes() {
                                        p { class: "text-xs text-muted-foreground", "Before: {format_bytes(size_bytes)}" }
                                    }
                                }
                                if read_loading() {
                                    p { class: "text-xs text-muted-foreground", "Reading metadata..." }
                                } else if read_report().is_some() {
                                    div { class: "flex flex-col gap-2 sm:flex-row",
                                        button {
                                            class: "inline-flex h-9 items-center justify-center rounded-md border border-green-500 bg-green-500 px-4 py-2 text-sm font-medium text-white transition-colors hover:bg-green-600 disabled:pointer-events-none disabled:opacity-50",
                                            r#type: "button",
                                            disabled: write_loading(),
                                            onclick: move |_| run_job(MediaWriteOperation::Compress),
                                            if write_loading() && operation() == MediaWriteOperation::Compress {
                                                "Compression in progress..."
                                            } else {
                                                "Compress Video"
                                            }
                                        }
                                        button {
                                            class: "inline-flex h-9 items-center justify-center rounded-md border border-border bg-background px-4 py-2 text-sm font-medium text-foreground transition-colors hover:bg-accent disabled:pointer-events-none disabled:opacity-50",
                                            r#type: "button",
                                            disabled: write_loading(),
                                            onclick: move |_| run_job(MediaWriteOperation::Transcode),
                                            if write_loading() && operation() == MediaWriteOperation::Transcode {
                                                "Transcoding in progress..."
                                            } else {
                                                "Transcode Video"
                                            }
                                        }
                                    }
                                }
                            }

                            if let Some(report) = read_report() {
                                div { class: "rounded-lg border bg-background px-3 py-3 text-xs text-muted-foreground space-y-1",
                                    p { class: "font-medium text-foreground", "Probe ready" }
                                    p { "trace_id: {report.trace_id}" }
                                    p { "format: {report.format_name}" }
                                    p { "streams: total={report.stream_count} video={report.video_count} audio={report.audio_count}" }
                                    p { "preset used for write actions: h264 / aac / mp4 / crf 23 / fast / 128k" }
                                }
                            }

                            if write_loading() {
                                div { class: "space-y-2",
                                    div { class: "h-2 overflow-hidden rounded-full bg-muted",
                                        div { class: "h-full w-1/2 animate-pulse rounded-full bg-green-600" }
                                    }
                                    p { class: "text-xs text-muted-foreground",
                                        if operation() == MediaWriteOperation::Compress {
                                            "Compression in progress. Wait for the before / after result card."
                                        } else {
                                            "Transcoding in progress. Wait for the result card."
                                        }
                                    }
                                }
                            }
                        }
                    }

                    if let Some(err) = error() {
                        p { class: "text-sm text-destructive", "{err}" }
                    }
                }
            }

            if let Some(job_result) = result() {
                MediaWriteResultCard { result: job_result }
            }

            if let Some(report) = read_report() {
                Card {
                    CardHeader {
                        CardTitle { "Raw ffprobe JSON" }
                        CardDescription { "{report.file_name} [{report.trace_id}]" }
                    }
                    CardContent {
                        pre { class: "max-h-[300px] overflow-auto rounded-xl bg-muted/30 p-4 font-mono text-xs leading-5 whitespace-pre-wrap break-all",
                            "{report.raw_json_pretty}"
                        }
                    }
                }
            }

            if !debug_events.read().is_empty() {
                Card {
                    CardHeader {
                        CardTitle { "Client Debug Trace" }
                        CardDescription { "Browser-side events for file selection, ffprobe upload, state updates, and ffmpeg jobs." }
                    }
                    CardContent {
                        pre { class: "max-h-[220px] overflow-auto rounded-xl bg-muted/30 p-4 font-mono text-xs leading-5 whitespace-pre-wrap break-all",
                            "{debug_events.read().join(\"\\n\")}"
                        }
                    }
                }
            }
        }
    }
}

async fn inspect_media_file(file: dioxus::html::FileData) -> Result<MediaProbeReport, String> {
    #[cfg(target_arch = "wasm32")]
    {
        let web_file = file
            .inner()
            .downcast_ref::<web_sys::File>()
            .cloned()
            .ok_or_else(|| "failed to access browser file handle".to_string())?;
        let form_data = web_sys::FormData::new().map_err(js_error)?;
        form_data
            .append_with_blob_and_filename("file", &web_file, &web_file.name())
            .map_err(js_error)?;

        let options = web_sys::RequestInit::new();
        options.set_method("POST");
        options.set_body(&form_data);

        let request = web_sys::Request::new_with_str_and_init("/api/media-read/upload", &options)
            .map_err(js_error)?;
        request
            .headers()
            .set("Accept", "application/json")
            .map_err(js_error)?;

        let window = web_sys::window().ok_or_else(|| "missing browser window".to_string())?;
        let response = JsFuture::from(window.fetch_with_request(&request))
            .await
            .map_err(js_error)?;
        let response: web_sys::Response = response
            .dyn_into()
            .map_err(|_| "failed to cast fetch response".to_string())?;

        let text_promise = response.text().map_err(js_error)?;
        let body = JsFuture::from(text_promise)
            .await
            .map_err(js_error)?
            .as_string()
            .unwrap_or_default();

        if !response.ok() {
            if let Ok(api_error) = serde_json::from_str::<MediaProbeErrorResponse>(&body) {
                return Err(format!(
                    "HTTP {} [{}]: {}",
                    response.status(),
                    api_error.trace_id,
                    api_error.message
                ));
            }
            return Err(format!("HTTP {}: {}", response.status(), body));
        }

        serde_json::from_str(&body).map_err(|e| e.to_string())
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = file;
        Err("media read is only available in the browser build".into())
    }
}

async fn upload_media_write_job(
    file: dioxus::html::FileData,
    operation: MediaWriteOperation,
    output_container: String,
    video_codec: String,
    audio_codec: String,
    crf: String,
    preset: String,
    audio_bitrate: String,
) -> Result<MediaWriteResult, String> {
    #[cfg(target_arch = "wasm32")]
    {
        let web_file = file
            .inner()
            .downcast_ref::<web_sys::File>()
            .cloned()
            .ok_or_else(|| "failed to access browser file handle".to_string())?;
        let form_data = web_sys::FormData::new().map_err(js_error)?;
        form_data
            .append_with_blob_and_filename("file", &web_file, &web_file.name())
            .map_err(js_error)?;
        form_data
            .append_with_str("output_container", &output_container)
            .map_err(js_error)?;
        form_data
            .append_with_str("video_codec", &video_codec)
            .map_err(js_error)?;
        form_data
            .append_with_str("audio_codec", &audio_codec)
            .map_err(js_error)?;
        form_data.append_with_str("crf", &crf).map_err(js_error)?;
        form_data
            .append_with_str("preset", &preset)
            .map_err(js_error)?;
        form_data
            .append_with_str("audio_bitrate", &audio_bitrate)
            .map_err(js_error)?;

        let options = web_sys::RequestInit::new();
        options.set_method("POST");
        options.set_body(&form_data);

        let url = format!("/api/media-write/{}", operation.as_str());
        let request = web_sys::Request::new_with_str_and_init(&url, &options).map_err(js_error)?;
        request
            .headers()
            .set("Accept", "application/json")
            .map_err(js_error)?;

        let window = web_sys::window().ok_or_else(|| "missing browser window".to_string())?;
        let response = JsFuture::from(window.fetch_with_request(&request))
            .await
            .map_err(js_error)?;
        let response: web_sys::Response = response
            .dyn_into()
            .map_err(|_| "failed to cast fetch response".to_string())?;

        let text_promise = response.text().map_err(js_error)?;
        let body = JsFuture::from(text_promise)
            .await
            .map_err(js_error)?
            .as_string()
            .unwrap_or_default();

        if !response.ok() {
            if let Ok(api_error) = serde_json::from_str::<MediaWriteErrorResponse>(&body) {
                return Err(format!(
                    "HTTP {} [{}]: {}",
                    response.status(),
                    api_error.trace_id,
                    api_error.message
                ));
            }
            return Err(format!("HTTP {}: {}", response.status(), body));
        }

        serde_json::from_str(&body).map_err(|e| e.to_string())
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (
            file,
            operation,
            output_container,
            video_codec,
            audio_codec,
            crf,
            preset,
            audio_bitrate,
        );
        Err("media write is only available in the browser build".into())
    }
}

#[cfg(target_arch = "wasm32")]
fn js_error(err: wasm_bindgen::JsValue) -> String {
    err.as_string().unwrap_or_else(|| format!("{err:?}"))
}

fn push_debug_event(debug_events: &mut Signal<Vec<String>>, message: String) {
    debug_log(&message);
    let mut events = debug_events.read().clone();
    events.push(message);
    if events.len() > 40 {
        let overflow = events.len() - 40;
        events.drain(0..overflow);
    }
    debug_events.set(events);
}

fn debug_log(message: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::console::log_1(&format!("[media_write] {message}").into());
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        println!("[media_write] {message}");
    }
}

#[component]
fn MediaWriteResultCard(result: MediaWriteResult) -> Element {
    rsx! {
        Card {
            CardHeader {
                CardTitle { "Job Result" }
                CardDescription {
                    "{result.operation} [{result.trace_id}]"
                }
            }
            CardContent { class: "space-y-4",
                div { class: "grid gap-3 md:grid-cols-2 xl:grid-cols-4",
                    ResultField { label: "Output", value: result.output_file_name.clone() }
                    ResultField { label: "Container", value: result.output_container.clone() }
                    ResultField { label: "Video codec", value: result.video_codec.clone() }
                    ResultField { label: "Audio codec", value: result.audio_codec.clone() }
                    ResultField { label: "Input bytes", value: result.input_bytes.to_string() }
                    ResultField { label: "Output bytes", value: result.output_bytes.to_string() }
                    ResultField { label: "Saved bytes", value: result.saved_bytes.to_string() }
                    ResultField { label: "Saved %", value: format!("{:.2}%", result.saved_percent) }
                    ResultField { label: "Elapsed", value: format!("{} ms", result.elapsed_ms) }
                    ResultField { label: "Timeout", value: format!("{} s", result.ffmpeg_timeout_secs) }
                    ResultField { label: "Job ID", value: result.job_id.clone() }
                }

                div { class: "flex items-center gap-3",
                    Button {
                        variant: ButtonVariant::Outline,
                        href: result.download_url.clone(),
                        "Download Output"
                    }
                }

                details { class: "rounded-xl border bg-muted/20 p-4",
                    summary { class: "cursor-pointer text-sm font-medium", "ffmpeg command" }
                    pre { class: "mt-3 overflow-auto rounded-lg bg-background p-3 font-mono text-xs whitespace-pre-wrap break-all",
                        "{result.command_summary}"
                    }
                }

                if !result.stderr_excerpt.trim().is_empty() {
                    details { class: "rounded-xl border bg-muted/20 p-4",
                        summary { class: "cursor-pointer text-sm font-medium", "stderr excerpt" }
                        pre { class: "mt-3 max-h-[240px] overflow-auto rounded-lg bg-background p-3 font-mono text-xs whitespace-pre-wrap break-all",
                            "{result.stderr_excerpt}"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ResultField(label: String, value: String) -> Element {
    rsx! {
        div { class: "rounded-lg border bg-background px-3 py-2",
            p { class: "text-[11px] uppercase tracking-wide text-muted-foreground", "{label}" }
            p { class: "mt-1 text-sm font-medium break-all", "{value}" }
        }
    }
}

fn format_bytes(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;
    let value = bytes as f64;
    if value >= GB {
        format!("{:.2} GB", value / GB)
    } else if value >= MB {
        format!("{:.2} MB", value / MB)
    } else if value >= KB {
        format!("{:.2} KB", value / KB)
    } else {
        format!("{bytes} B")
    }
}
