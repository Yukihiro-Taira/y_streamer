use dioxus::html::HasFileData;
use dioxus::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::JsFuture;

#[cfg(target_arch = "wasm32")]
use crate::domain::media_read::data::media_probe_report::MediaProbeErrorResponse;
use crate::domain::diagnostic::data::diagnostic_report::{
    DiagnosticCheck, DiagnosticReport, DiagnosticStatus,
};
use crate::domain::diagnostic::data::platform_profile::PlatformProfile;
use crate::domain::diagnostic::service::diagnostic_rules;
use crate::domain::media_read::data::media_probe_report::MediaProbeReport;

// ── Page ──────────────────────────────────────────────────────────────────────

#[component]
pub fn DiagnosticPage() -> Element {
    let mut probe_report = use_signal(|| None::<MediaProbeReport>);
    let mut diagnostic = use_signal(|| None::<DiagnosticReport>);
    let mut error = use_signal(|| None::<String>);
    let mut loading = use_signal(|| false);
    let mut drag_active = use_signal(|| false);
    let mut profile = use_signal(|| PlatformProfile::Web);

    let mut inspect = move |file: dioxus::html::FileData| {
        error.set(None);
        probe_report.set(None);
        diagnostic.set(None);
        loading.set(true);

        spawn({
            let mut loading = loading;
            let mut error = error;
            let mut probe_report = probe_report;
            let mut diagnostic = diagnostic;
            let current_profile = profile();
            async move {
                match upload_file(file).await {
                    Ok(r) => {
                        let diag = diagnostic_rules::run(&r, &current_profile);
                        diagnostic.set(Some(diag));
                        probe_report.set(Some(r));
                    }
                    Err(e) => error.set(Some(e)),
                }
                loading.set(false);
            }
        });
    };

    // Re-run rules when profile changes without re-uploading
    let mut rerun_diagnostic = move |_| {
        if let Some(r) = probe_report() {
            let diag = diagnostic_rules::run(&r, &profile());
            diagnostic.set(Some(diag));
        }
    };

    rsx! {
        div { class: "max-w-[900px] mx-auto w-full px-6 py-8 space-y-6",

            // ── Header ──────────────────────────────────────────────────────
            div { class: "space-y-1",
                h1 { class: "text-lg font-semibold", "Video Diagnostic" }
                p { class: "text-sm text-muted-foreground",
                    "Drop a video to run diagnostic checks — container, codec, audio, VFR, subtitles, A/V sync."
                }
            }

            // ── Platform profile selector ────────────────────────────────
            div { class: "flex items-center gap-2",
                span { class: "text-xs text-muted-foreground font-medium uppercase tracking-wide mr-1", "Profile" }
                for p in [PlatformProfile::Web, PlatformProfile::Broadcast, PlatformProfile::Mobile] {
                    {
                        let label = p.label();
                        let is_active = profile() == p;
                        let cls = if is_active {
                            "px-3 py-1 rounded-full text-xs font-medium bg-primary text-primary-foreground"
                        } else {
                            "px-3 py-1 rounded-full text-xs font-medium bg-muted text-muted-foreground hover:bg-muted/80 cursor-pointer"
                        };
                        rsx! {
                            button {
                                class: cls,
                                onclick: move |_| {
                                    profile.set(p.clone());
                                    rerun_diagnostic(());
                                },
                                "{label}"
                            }
                        }
                    }
                }
            }

            // ── Drop zone ────────────────────────────────────────────────
            div {
                class: if drag_active() {
                    "rounded-2xl border-2 border-dashed border-primary bg-primary/5 px-6 py-10 text-center transition-colors cursor-pointer"
                } else {
                    "rounded-2xl border-2 border-dashed border-border bg-muted/20 px-6 py-10 text-center transition-colors cursor-pointer"
                },
                ondragover: move |evt| {
                    evt.prevent_default();
                    drag_active.set(true);
                },
                ondragleave: move |_| drag_active.set(false),
                ondrop: move |evt: Event<DragData>| {
                    evt.prevent_default();
                    drag_active.set(false);
                    if let Some(file) = evt.files().into_iter().next() {
                        inspect(file);
                    }
                },
                p { class: "text-sm font-medium", "Drop video file here" }
                p { class: "mt-1 text-xs text-muted-foreground", "or choose one below" }
                input {
                    class: "mt-4 block w-full text-sm",
                    r#type: "file",
                    accept: "video/*,.mov,.mp4,.mkv,.mxf",
                    onchange: move |evt: Event<FormData>| {
                        if let Some(file) = evt.files().into_iter().next() {
                            inspect(file);
                        }
                    },
                }
            }

            // ── Status / error ───────────────────────────────────────────
            if loading() {
                p { class: "text-sm text-muted-foreground animate-pulse", "Running ffprobe on server..." }
            }
            if let Some(err) = error() {
                p { class: "text-sm text-destructive", "Error: {err}" }
            }

            // ── Results ──────────────────────────────────────────────────
            if let (Some(diag), Some(r)) = (diagnostic(), probe_report()) {
                HeaderCard { report: r.clone(), diag: diag.clone() }
                DiagnosticResults { report: diag }
            }
        }
    }
}

// ── Header card ───────────────────────────────────────────────────────────────

#[component]
fn HeaderCard(report: MediaProbeReport, diag: DiagnosticReport) -> Element {
    let fail_count = diag.fail_count();
    let warn_count = diag.warn_count();

    let overall_class = if fail_count > 0 {
        "border-red-400 bg-red-50 dark:bg-red-950/20"
    } else if warn_count > 0 {
        "border-yellow-400 bg-yellow-50 dark:bg-yellow-950/20"
    } else {
        "border-green-400 bg-green-50 dark:bg-green-950/20"
    };

    let overall_label = if fail_count > 0 {
        format!("✗ {fail_count} fail")
    } else if warn_count > 0 {
        format!("⚠ {warn_count} warn")
    } else {
        "✓ All pass".into()
    };

    let overall_label_class = if fail_count > 0 {
        "text-red-700 dark:text-red-400 font-semibold text-sm"
    } else if warn_count > 0 {
        "text-yellow-700 dark:text-yellow-400 font-semibold text-sm"
    } else {
        "text-green-700 dark:text-green-400 font-semibold text-sm"
    };

    rsx! {
        div { class: "rounded-xl border-2 p-4 flex items-start gap-4 {overall_class}",
            // Thumbnail preview
            if let Some(thumb) = report.thumbnails.first() {
                img {
                    class: "w-24 h-14 object-cover rounded-md shrink-0 border border-border",
                    src: "{thumb}",
                    alt: "Video preview"
                }
            } else {
                div { class: "w-24 h-14 rounded-md shrink-0 border border-border bg-muted flex items-center justify-center text-muted-foreground text-xs",
                    "No preview"
                }
            }
            // File info
            div { class: "flex-1 min-w-0 space-y-1",
                p { class: "text-sm font-semibold truncate", "{report.file_name}" }
                div { class: "flex flex-wrap gap-2 text-xs text-muted-foreground",
                    if !report.duration.is_empty() {
                        span { "⏱ {report.duration}" }
                    }
                    if !report.size.is_empty() {
                        span { "💾 {report.size}" }
                    }
                    if !report.format_name.is_empty() {
                        span { "📦 {report.format_name}" }
                    }
                }
                span { class: overall_label_class, "{overall_label}" }
            }
        }
    }
}

// ── Results ───────────────────────────────────────────────────────────────────

#[component]
fn DiagnosticResults(report: DiagnosticReport) -> Element {
    let pass_count = report.pass_count();
    let warn_count = report.warn_count();
    let fail_count = report.fail_count();

    rsx! {
        div { class: "space-y-3",
            div { class: "flex items-center gap-3",
                if pass_count > 0 {
                    StatusPill { count: pass_count, kind: "pass" }
                }
                if warn_count > 0 {
                    StatusPill { count: warn_count, kind: "warn" }
                }
                if fail_count > 0 {
                    StatusPill { count: fail_count, kind: "fail" }
                }
            }
            div { class: "divide-y rounded-xl border border-border overflow-hidden",
                for check in report.checks.iter() {
                    CheckRow { check: check.clone() }
                }
            }
        }
    }
}

#[component]
fn StatusPill(count: usize, kind: &'static str) -> Element {
    let (class, icon, label) = match kind {
        "warn" => (
            "bg-yellow-100 text-yellow-800 dark:bg-yellow-900/30 dark:text-yellow-400",
            "⚠",
            "warn",
        ),
        "fail" => (
            "bg-red-100 text-red-800 dark:bg-red-900/30 dark:text-red-400",
            "✗",
            "fail",
        ),
        _ => (
            "bg-green-100 text-green-800 dark:bg-green-900/30 dark:text-green-400",
            "✓",
            "pass",
        ),
    };
    rsx! {
        span { class: "inline-flex items-center gap-1 rounded-full px-2.5 py-0.5 text-xs font-medium {class}",
            "{icon} {count} {label}"
        }
    }
}

#[component]
fn CheckRow(check: DiagnosticCheck) -> Element {
    let (badge_class, icon) = match check.status {
        DiagnosticStatus::Pass => (
            "bg-green-100 text-green-800 dark:bg-green-900/30 dark:text-green-400",
            "✓",
        ),
        DiagnosticStatus::Warn => (
            "bg-yellow-100 text-yellow-800 dark:bg-yellow-900/30 dark:text-yellow-400",
            "⚠",
        ),
        DiagnosticStatus::Fail => (
            "bg-red-100 text-red-800 dark:bg-red-900/30 dark:text-red-400",
            "✗",
        ),
    };

    rsx! {
        div { class: "flex items-center gap-3 px-4 py-3 bg-card",
            span { class: "inline-flex items-center justify-center size-6 rounded-full text-xs font-bold shrink-0 {badge_class}",
                "{icon}"
            }
            div { class: "flex-1 min-w-0",
                span { class: "text-sm font-medium", "{check.label}" }
                span { class: "ml-2 text-xs text-muted-foreground", "{check.detail}" }
            }
        }
    }
}

// ── Upload ────────────────────────────────────────────────────────────────────

async fn upload_file(file: dioxus::html::FileData) -> Result<MediaProbeReport, String> {
    #[cfg(target_arch = "wasm32")]
    {
        let web_file = file
            .inner()
            .downcast_ref::<web_sys::File>()
            .cloned()
            .ok_or_else(|| "failed to access browser file handle".to_string())?;

        let form_data = web_sys::FormData::new().map_err(js_err)?;
        form_data
            .append_with_blob_and_filename("file", &web_file, &web_file.name())
            .map_err(js_err)?;

        let options = web_sys::RequestInit::new();
        options.set_method("POST");
        options.set_body(&form_data);

        let request =
            web_sys::Request::new_with_str_and_init("/api/media-read/upload", &options)
                .map_err(js_err)?;
        request
            .headers()
            .set("Accept", "application/json")
            .map_err(js_err)?;

        let window = web_sys::window().ok_or_else(|| "missing browser window".to_string())?;
        let response = JsFuture::from(window.fetch_with_request(&request))
            .await
            .map_err(js_err)?;
        let response: web_sys::Response = response
            .dyn_into()
            .map_err(|_| "failed to cast fetch response".to_string())?;

        let body = JsFuture::from(response.text().map_err(js_err)?)
            .await
            .map_err(js_err)?
            .as_string()
            .unwrap_or_default();

        if !response.ok() {
            if let Ok(e) = serde_json::from_str::<MediaProbeErrorResponse>(&body) {
                return Err(format!(
                    "HTTP {} [{}]: {}",
                    response.status(),
                    e.trace_id,
                    e.message
                ));
            }
            return Err(format!("HTTP {}: {}", response.status(), body));
        }

        serde_json::from_str(&body).map_err(|e| e.to_string())
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = file;
        Err("only available in browser build".into())
    }
}

#[cfg(target_arch = "wasm32")]
fn js_err(err: wasm_bindgen::JsValue) -> String {
    err.as_string().unwrap_or_else(|| format!("{err:?}"))
}
