use dioxus::html::HasFileData;
use dioxus::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::JsFuture;

use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};
use crate::components::ui::input::{Input, InputType};
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
    let mut operation = use_signal(|| MediaWriteOperation::Compress);
    let mut output_container = use_signal(|| "mp4".to_string());
    let mut video_codec = use_signal(|| "h264".to_string());
    let mut audio_codec = use_signal(|| "aac".to_string());
    let mut crf = use_signal(|| "23".to_string());
    let mut preset = use_signal(|| "fast".to_string());
    let mut audio_bitrate = use_signal(|| "128k".to_string());
    let mut loading = use_signal(|| false);
    let mut error = use_signal(|| None::<String>);
    let mut result = use_signal(|| None::<MediaWriteResult>);
    let mut drag_active = use_signal(|| false);

    let mut set_file = move |file: dioxus::html::FileData| {
        selected_name.set(Some(file.name()));
        selected_file.set(Some(file));
        error.set(None);
        result.set(None);
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

    let on_submit = move |_| {
        let maybe_file = selected_file.read().clone();
        let current_operation = operation();
        let current_output_container = output_container.read().clone();
        let current_video_codec = video_codec.read().clone();
        let current_audio_codec = audio_codec.read().clone();
        let current_crf = crf.read().clone();
        let current_preset = preset.read().clone();
        let current_audio_bitrate = audio_bitrate.read().clone();

        error.set(None);
        result.set(None);

        let Some(file) = maybe_file else {
            error.set(Some("Select a file first.".into()));
            return;
        };

        loading.set(true);
        spawn({
            let mut loading = loading;
            let mut error = error;
            let mut result = result;
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
                    Ok(job_result) => result.set(Some(job_result)),
                    Err(err) => error.set(Some(err)),
                }
                loading.set(false);
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
                        "Upload one file, choose compress or transcode, then run ffmpeg on the server."
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
                        p { class: "text-sm text-muted-foreground", "Selected: {name}" }
                    }

                    div { class: "grid gap-4 md:grid-cols-2 xl:grid-cols-4",
                        LabeledSelect {
                            label: "Operation",
                            value: operation().as_str().to_string(),
                            onchange: move |value| {
                                if value == "transcode" {
                                    operation.set(MediaWriteOperation::Transcode);
                                } else {
                                    operation.set(MediaWriteOperation::Compress);
                                    output_container.set("mp4".into());
                                    video_codec.set("h264".into());
                                    audio_codec.set("aac".into());
                                }
                            },
                            options: vec![
                                ("compress".to_string(), "Compress".to_string()),
                                ("transcode".to_string(), "Transcode".to_string()),
                            ]
                        }

                        LabeledSelect {
                            label: "Container",
                            value: output_container.read().clone(),
                            onchange: move |value| output_container.set(value),
                            options: vec![
                                ("mp4".to_string(), "mp4".to_string()),
                                ("mov".to_string(), "mov".to_string()),
                                ("mkv".to_string(), "mkv".to_string()),
                                ("webm".to_string(), "webm".to_string()),
                            ]
                        }

                        LabeledSelect {
                            label: "Video codec",
                            value: video_codec.read().clone(),
                            onchange: move |value| video_codec.set(value),
                            options: {
                                let mut values = vec![
                                    ("h264".to_string(), "h264".to_string()),
                                    ("hevc".to_string(), "hevc".to_string()),
                                    ("vp9".to_string(), "vp9".to_string()),
                                ];
                                if operation() == MediaWriteOperation::Transcode {
                                    values.push(("copy".to_string(), "copy".to_string()));
                                }
                                values
                            }
                        }

                        LabeledSelect {
                            label: "Audio codec",
                            value: audio_codec.read().clone(),
                            onchange: move |value| audio_codec.set(value),
                            options: {
                                let mut values = vec![
                                    ("aac".to_string(), "aac".to_string()),
                                    ("opus".to_string(), "opus".to_string()),
                                    ("mp3".to_string(), "mp3".to_string()),
                                ];
                                if operation() == MediaWriteOperation::Transcode {
                                    values.push(("copy".to_string(), "copy".to_string()));
                                }
                                values
                            }
                        }

                        LabeledInput {
                            label: "CRF",
                            value: crf.read().clone(),
                            oninput: move |value| crf.set(value),
                            disabled: operation() != MediaWriteOperation::Compress,
                        }

                        LabeledSelect {
                            label: "Preset",
                            value: preset.read().clone(),
                            onchange: move |value| preset.set(value),
                            options: vec![
                                ("ultrafast".to_string(), "ultrafast".to_string()),
                                ("superfast".to_string(), "superfast".to_string()),
                                ("veryfast".to_string(), "veryfast".to_string()),
                                ("fast".to_string(), "fast".to_string()),
                                ("medium".to_string(), "medium".to_string()),
                                ("slow".to_string(), "slow".to_string()),
                            ]
                        }

                        LabeledSelect {
                            label: "Audio bitrate",
                            value: audio_bitrate.read().clone(),
                            onchange: move |value| audio_bitrate.set(value),
                            options: vec![
                                ("96k".to_string(), "96k".to_string()),
                                ("128k".to_string(), "128k".to_string()),
                                ("160k".to_string(), "160k".to_string()),
                                ("192k".to_string(), "192k".to_string()),
                                ("256k".to_string(), "256k".to_string()),
                            ]
                        }
                    }

                    div { class: "flex items-center gap-3",
                        Button {
                            disabled: loading(),
                            onclick: on_submit,
                            if loading() { "Running ffmpeg..." } else { "Run Job" }
                        }
                        if let Some(err) = error() {
                            p { class: "text-sm text-destructive", "{err}" }
                        }
                    }
                }
            }

            if let Some(job_result) = result() {
                MediaWriteResultCard { result: job_result }
            }
        }
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

#[component]
fn LabeledInput(
    label: String,
    value: String,
    oninput: EventHandler<String>,
    #[props(default = false)] disabled: bool,
) -> Element {
    rsx! {
        div { class: "space-y-1.5",
            p { class: "text-[11px] uppercase tracking-wide text-muted-foreground", "{label}" }
            Input {
                r#type: InputType::Text,
                value: value,
                disabled: disabled,
                oninput: move |evt: FormEvent| oninput.call(evt.value()),
            }
        }
    }
}

#[component]
fn LabeledSelect(
    label: String,
    value: String,
    onchange: EventHandler<String>,
    options: Vec<(String, String)>,
) -> Element {
    rsx! {
        div { class: "space-y-1.5",
            p { class: "text-[11px] uppercase tracking-wide text-muted-foreground", "{label}" }
            select {
                class: "border-input focus-visible:border-ring focus-visible:ring-ring/50 h-9 w-full rounded-md border bg-transparent px-3 py-1 text-sm shadow-xs outline-none focus-visible:ring-2",
                value: "{value}",
                onchange: move |evt| onchange.call(evt.value()),
                for (option_value, option_label) in options {
                    option { value: "{option_value}", "{option_label}" }
                }
            }
        }
    }
}
