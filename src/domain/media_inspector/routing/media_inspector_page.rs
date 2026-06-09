use dioxus::html::HasFileData;
use dioxus::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::JsFuture;

use crate::components::ui::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};
#[cfg(target_arch = "wasm32")]
use crate::domain::media_inspector::data::media_probe_report::MediaProbeErrorResponse;
use crate::domain::media_inspector::data::media_probe_report::MediaProbeReport;

#[component]
pub fn MediaInspectorPage() -> Element {
    let mut selected_name = use_signal(|| None::<String>);
    let mut report = use_signal(|| None::<MediaProbeReport>);
    let mut error = use_signal(|| None::<String>);
    let mut loading = use_signal(|| false);
    let mut drag_active = use_signal(|| false);

    let mut inspect_file = move |file: dioxus::html::FileData| {
        selected_name.set(Some(file.name()));
        error.set(None);
        report.set(None);
        loading.set(true);

        spawn({
            let mut loading = loading;
            let mut error = error;
            let mut report = report;
            async move {
                match upload_media_file(file).await {
                    Ok(result) => report.set(Some(result)),
                    Err(err) => error.set(Some(err)),
                }
                loading.set(false);
            }
        });
    };

    let on_input = move |evt: Event<FormData>| {
        if let Some(file) = evt.files().into_iter().next() {
            inspect_file(file);
        }
    };

    let on_drop = move |evt: Event<DragData>| {
        evt.prevent_default();
        drag_active.set(false);
        if let Some(file) = evt.files().into_iter().next() {
            inspect_file(file);
        }
    };

    rsx! {
        div { class: "max-w-[1100px] mx-auto w-full px-6 py-8 space-y-6",
            Card {
                CardHeader {
                    CardTitle { "Media Inspector" }
                    CardDescription {
                        "Upload a local media file. The browser sends the bytes to the Rust server, which runs ffprobe and returns a structured report."
                    }
                }
                CardContent { class: "space-y-4",
                    div {
                        class: if drag_active() {
                            "rounded-2xl border-2 border-dashed border-primary bg-primary/5 px-6 py-10 text-center transition-colors"
                        } else {
                            "rounded-2xl border-2 border-dashed border-border bg-muted/20 px-6 py-10 text-center transition-colors"
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
                            accept: "video/*,audio/*,.mov,.mp4,.mkv,.webm,.mxf,.mp3,.wav,.aac,.flac",
                            onchange: on_input,
                        }
                    }

                    if let Some(name) = selected_name() {
                        p { class: "text-sm text-muted-foreground", "Selected: {name}" }
                    }
                    if loading() {
                        p { class: "text-sm text-muted-foreground", "Running ffprobe on server..." }
                    }
                    if let Some(err) = error() {
                        p { class: "text-sm text-destructive", "Error: {err}" }
                    }
                }
            }

            if let Some(result) = report() {
                MediaInspectorReport { report: result }
            }
        }
    }
}

async fn upload_media_file(file: dioxus::html::FileData) -> Result<MediaProbeReport, String> {
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

        let request =
            web_sys::Request::new_with_str_and_init("/api/media-inspector/upload", &options)
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
        Err("media upload is only available in the browser build".into())
    }
}

#[cfg(target_arch = "wasm32")]
fn js_error(err: wasm_bindgen::JsValue) -> String {
    err.as_string().unwrap_or_else(|| format!("{err:?}"))
}

#[component]
fn MediaInspectorReport(report: MediaProbeReport) -> Element {
    rsx! {
        Card {
            CardHeader {
                CardTitle { "Raw ffprobe JSON" }
                CardDescription { "{report.file_name} [{report.trace_id}]" }
            }
            CardContent {
                pre { class: "max-h-[80vh] overflow-auto rounded-xl bg-muted/30 p-4 font-mono text-xs leading-5 whitespace-pre-wrap break-all",
                    "{report.raw_json_pretty}"
                }
            }
        }
    }
}
