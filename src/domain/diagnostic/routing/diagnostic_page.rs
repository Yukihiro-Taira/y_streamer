use dioxus::html::HasFileData;
use dioxus::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::JsFuture;

use crate::components::diagnostic_compare_panel::ComparisonPanel;
use crate::domain::diagnostic::data::diagnostic_report::{
    DiagnosticCheck, DiagnosticReport, DiagnosticStatus,
};
use crate::domain::diagnostic::data::platform_profile::PlatformProfile;
use crate::domain::diagnostic::service::diagnostic_rules;
use crate::domain::media_read::data::diagnostic_progress::{DiagnosticProgress, ProgressStage};
#[cfg(target_arch = "wasm32")]
use crate::domain::media_read::data::media_probe_report::MediaProbeErrorResponse;
use crate::domain::media_read::data::media_probe_report::{
    LoudnessReport, MediaInfoReport, MediaProbeReport, MediaSceneCut, MediaStreamInfo,
};

// ── Page ──────────────────────────────────────────────────────────────────────

#[component]
pub fn DiagnosticPage() -> Element {
    let mut probe_report = use_signal(|| None::<MediaProbeReport>);
    let mut diagnostic = use_signal(|| None::<DiagnosticReport>);
    let mut error = use_signal(|| None::<String>);
    let mut loading = use_signal(|| false);
    let mut drag_active = use_signal(|| false);
    let mut compare_probe_report = use_signal(|| None::<MediaProbeReport>);
    let mut compare_diagnostic = use_signal(|| None::<DiagnosticReport>);
    let mut compare_error = use_signal(|| None::<String>);
    let mut compare_loading = use_signal(|| false);
    let mut compare_drag_active = use_signal(|| false);
    let mut profile = use_signal(|| PlatformProfile::Web);
    let mut progress = use_signal(|| None::<DiagnosticProgress>);
    let mut upload_progress = use_signal(|| None::<(u64, u64)>);
    let mut compare_progress = use_signal(|| None::<DiagnosticProgress>);
    let mut compare_upload_progress = use_signal(|| None::<(u64, u64)>);
    let mut preview_url = use_signal(|| None::<String>);
    let mut compare_preview_url = use_signal(|| None::<String>);

    let mut inspect = move |file: dioxus::html::FileData| {
        revoke_browser_preview_url(preview_url());
        preview_url.set(browser_preview_url(&file));
        error.set(None);
        probe_report.set(None);
        diagnostic.set(None);
        progress.set(None);
        upload_progress.set(None);
        loading.set(true);

        spawn({
            let current_profile = profile();
            async move {
                // Phase 1: upload file, get trace_id immediately
                let trace_id = match start_upload(file, move |loaded, total| {
                    upload_progress.set(Some((loaded, total)));
                })
                .await
                {
                    Ok(t) => t,
                    Err(e) => {
                        error.set(Some(e));
                        loading.set(false);
                        return;
                    }
                };
                upload_progress.set(None);

                // Phase 2: poll progress until Done or Failed
                #[cfg(target_arch = "wasm32")]
                loop {
                    sleep_ms(500).await;

                    match poll_progress(&trace_id).await {
                        Ok(p) => match p.stage.clone() {
                            ProgressStage::Done { report } => {
                                let diag = diagnostic_rules::run(&report, &current_profile);
                                diagnostic.set(Some(diag));
                                probe_report.set(Some(*report));
                                progress.set(None);
                                loading.set(false);
                                break;
                            }
                            ProgressStage::Failed { message } => {
                                error.set(Some(message));
                                progress.set(None);
                                loading.set(false);
                                break;
                            }
                            _ => {
                                progress.set(Some(p));
                            }
                        },
                        Err(e) => {
                            error.set(Some(e));
                            loading.set(false);
                            break;
                        }
                    }
                }
                #[cfg(not(target_arch = "wasm32"))]
                loading.set(false);
            }
        });
    };

    let mut inspect_compare = move |file: dioxus::html::FileData| {
        revoke_browser_preview_url(compare_preview_url());
        compare_preview_url.set(browser_preview_url(&file));
        compare_error.set(None);
        compare_probe_report.set(None);
        compare_diagnostic.set(None);
        compare_progress.set(None);
        compare_upload_progress.set(None);
        compare_loading.set(true);

        spawn({
            let current_profile = profile();
            async move {
                let trace_id = match start_upload(file, move |loaded, total| {
                    compare_upload_progress.set(Some((loaded, total)));
                })
                .await
                {
                    Ok(t) => t,
                    Err(e) => {
                        compare_error.set(Some(e));
                        compare_loading.set(false);
                        return;
                    }
                };
                compare_upload_progress.set(None);

                #[cfg(target_arch = "wasm32")]
                loop {
                    sleep_ms(500).await;

                    match poll_progress(&trace_id).await {
                        Ok(p) => match p.stage.clone() {
                            ProgressStage::Done { report } => {
                                let diag = diagnostic_rules::run(&report, &current_profile);
                                compare_diagnostic.set(Some(diag));
                                compare_probe_report.set(Some(*report));
                                compare_progress.set(None);
                                compare_loading.set(false);
                                break;
                            }
                            ProgressStage::Failed { message } => {
                                compare_error.set(Some(message));
                                compare_progress.set(None);
                                compare_loading.set(false);
                                break;
                            }
                            _ => {
                                compare_progress.set(Some(p));
                            }
                        },
                        Err(e) => {
                            compare_error.set(Some(e));
                            compare_loading.set(false);
                            break;
                        }
                    }
                }
                #[cfg(not(target_arch = "wasm32"))]
                compare_loading.set(false);
            }
        });
    };

    // Re-run rules when profile changes without re-uploading
    let mut rerun_diagnostic = move |_| {
        if let Some(r) = probe_report() {
            let diag = diagnostic_rules::run(&r, &profile());
            diagnostic.set(Some(diag));
        }
        if let Some(r) = compare_probe_report() {
            let diag = diagnostic_rules::run(&r, &profile());
            compare_diagnostic.set(Some(diag));
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

            // ── Drop zones ───────────────────────────────────────────────
            div { class: "grid gap-4 md:grid-cols-2",
                UploadSlot {
                    slot_label: "File A".to_string(),
                    title: "Primary diagnostic".to_string(),
                    subtitle: "Drop the main file to inspect.".to_string(),
                    file_name: probe_report().as_ref().map(|report| report.file_name.clone()),
                    clear_label: None,
                    drag_active: drag_active(),
                    ondragover: move |evt: Event<DragData>| {
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
                    onchange: move |evt: Event<FormData>| {
                        if let Some(file) = evt.files().into_iter().next() {
                            inspect(file);
                        }
                    },
                    onclear: None,
                }
                UploadSlot {
                    slot_label: "File B".to_string(),
                    title: "Optional comparison".to_string(),
                    subtitle: "Drop a second file only if you want a side-by-side diff.".to_string(),
                    file_name: compare_probe_report().as_ref().map(|report| report.file_name.clone()),
                    clear_label: Some("Clear File B".to_string()),
                    drag_active: compare_drag_active(),
                    ondragover: move |evt: Event<DragData>| {
                        evt.prevent_default();
                        compare_drag_active.set(true);
                    },
                    ondragleave: move |_| compare_drag_active.set(false),
                    ondrop: move |evt: Event<DragData>| {
                        evt.prevent_default();
                        compare_drag_active.set(false);
                        if let Some(file) = evt.files().into_iter().next() {
                            inspect_compare(file);
                        }
                    },
                    onchange: move |evt: Event<FormData>| {
                        if let Some(file) = evt.files().into_iter().next() {
                            inspect_compare(file);
                        }
                    },
                    onclear: move |_| {
                        revoke_browser_preview_url(compare_preview_url());
                        compare_preview_url.set(None);
                        compare_probe_report.set(None);
                        compare_diagnostic.set(None);
                        compare_error.set(None);
                        compare_progress.set(None);
                        compare_upload_progress.set(None);
                        compare_loading.set(false);
                    },
                }
            }

            // ── Status / error ───────────────────────────────────────────
            if loading() {
                if let Some(p) = progress() {
                    ProgressPanel { progress: p }
                } else {
                    UploadingPanel { progress: upload_progress() }
                }
            }
            if let Some(err) = error() {
                p { class: "text-sm text-destructive", "Error: {err}" }
            }
            if compare_loading() {
                if let Some(p) = compare_progress() {
                    ProgressPanel { progress: p }
                } else {
                    UploadingPanel { progress: compare_upload_progress() }
                }
            }
            if let Some(err) = compare_error() {
                p { class: "text-sm text-destructive", "Compare error: {err}" }
            }

            // ── Results ──────────────────────────────────────────────────
            if let (Some(diag), Some(r)) = (diagnostic(), probe_report()) {
                HeaderCard { report: r.clone(), diag: diag.clone() }
                if let Some(url) = preview_url() {
                    VideoPreviewPanel { preview_url: url }
                }
                DiagnosticResults { report: diag }
                if let Some(err) = &r.mediainfo_error {
                    ToolErrorBanner { tool: "mediainfo", message: err.clone() }
                }
                if let Some(err) = &r.loudness_error {
                    ToolErrorBanner { tool: "loudness (ffmpeg)", message: err.clone() }
                }
                if let Some(loudness) = &r.loudness {
                    LoudnessPanel { loudness: loudness.clone() }
                }
                if let Some(mi) = &r.mediainfo {
                    MediaInfoPanel { mi: mi.clone() }
                }
                if !r.waveform_image.is_empty() {
                    WaveformPanel { image_url: r.waveform_image.clone() }
                }
                if !r.scene_cuts.is_empty() {
                    SceneCutsPanel { cuts: r.scene_cuts.clone() }
                }
                RawStreamDataPanel { report: r.clone() }
            }
            if let (Some(left_report), Some(left_diag), Some(right_report), Some(right_diag)) =
                (probe_report(), diagnostic(), compare_probe_report(), compare_diagnostic())
            {
                ComparisonPanel {
                    left_report,
                    left_diag,
                    right_report,
                    right_diag,
                }
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
fn UploadSlot(
    slot_label: String,
    title: String,
    subtitle: String,
    file_name: Option<String>,
    clear_label: Option<String>,
    drag_active: bool,
    ondragover: EventHandler<Event<DragData>>,
    ondragleave: EventHandler<Event<DragData>>,
    ondrop: EventHandler<Event<DragData>>,
    onchange: EventHandler<Event<FormData>>,
    onclear: Option<EventHandler<Event<MouseData>>>,
) -> Element {
    let class = if drag_active {
        "rounded-2xl border-2 border-dashed border-primary bg-primary/5 px-6 py-8 text-center transition-colors cursor-pointer"
    } else {
        "rounded-2xl border-2 border-dashed border-border bg-muted/20 px-6 py-8 text-center transition-colors cursor-pointer"
    };

    rsx! {
        div {
            class,
            ondragover: move |evt| ondragover.call(evt),
            ondragleave: move |evt| ondragleave.call(evt),
            ondrop: move |evt| ondrop.call(evt),
            div { class: "space-y-1",
                span { class: "inline-flex rounded-full bg-muted px-2 py-0.5 text-[11px] font-semibold text-muted-foreground",
                    "{slot_label}"
                }
                p { class: "text-sm font-medium", "{title}" }
                p { class: "text-xs text-muted-foreground", "{subtitle}" }
                if let Some(name) = file_name {
                    p { class: "text-xs font-mono text-foreground truncate", "{name}" }
                }
            }
            input {
                class: "mt-4 block w-full text-sm",
                r#type: "file",
                accept: "video/*,.mov,.mp4,.mkv,.mxf",
                onchange: move |evt| onchange.call(evt),
            }
            if let (Some(label), Some(onclear)) = (clear_label, onclear) {
                button {
                    class: "mt-3 inline-flex rounded-md border border-border px-3 py-1 text-xs text-muted-foreground hover:bg-muted/60",
                    onclick: move |evt| onclear.call(evt),
                    "{label}"
                }
            }
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


// ── Upload progress panel ─────────────────────────────────────────────────────

#[component]
fn UploadingPanel(progress: Option<(u64, u64)>) -> Element {
    rsx! {
        div { class: "rounded-xl border border-border bg-muted/20 px-4 py-4 space-y-3",
            div { class: "flex items-center gap-2",
                span { class: "size-2 rounded-full bg-primary animate-pulse inline-block" }
                p { class: "text-sm font-medium", "Uploading…" }
            }
            if let Some((loaded, total)) = progress {
                {
                    let pct = loaded.checked_mul(100).and_then(|v| v.checked_div(total)).unwrap_or(0) as u32;
                    let loaded_mb = loaded as f64 / (1024.0 * 1024.0);
                    let total_mb = total as f64 / (1024.0 * 1024.0);
                    rsx! {
                        div { class: "space-y-1",
                            div { class: "h-2 w-full rounded-full bg-muted overflow-hidden",
                                div {
                                    class: "h-2 rounded-full bg-primary transition-all duration-200",
                                    style: "width: {pct}%",
                                }
                            }
                            p { class: "text-xs text-muted-foreground",
                                "{pct}%  ·  {loaded_mb:.1} MB / {total_mb:.1} MB"
                            }
                        }
                    }
                }
            }
        }
    }
}

// ── Progress panel ────────────────────────────────────────────────────────────

#[component]
fn ProgressPanel(progress: DiagnosticProgress) -> Element {
    let stage_label = match &progress.stage {
        ProgressStage::Uploading => "Uploading…".to_string(),
        ProgressStage::RunningFfprobe { upload_ms } => {
            format!("Running ffprobe… (upload took {}ms)", upload_ms)
        }
        ProgressStage::RunningEnrichment {
            ffprobe_ms,
            stream_count,
            video_codec,
            resolution,
            audio_codec,
            duration_label,
            mediainfo_done,
            loudness_done,
            thumbnails_done,
            waveform_done,
            scenes_done,
            subtitles_done,
        } => {
            let vc = video_codec.as_deref().unwrap_or("?");
            let res = resolution.as_deref().unwrap_or("?");
            let ac = audio_codec.as_deref().unwrap_or("?");
            let dur = duration_label.as_deref().unwrap_or("?");
            format!(
                "Enriching… ffprobe done in {ffprobe_ms}ms | {stream_count} streams | {vc} {res} | {ac} | {dur}"
            )
        }
        ProgressStage::Done { .. } => "Done".to_string(),
        ProgressStage::Failed { message } => format!("Failed: {message}"),
    };

    let enrichment_rows: Option<Vec<(&'static str, bool)>> =
        if let ProgressStage::RunningEnrichment {
            mediainfo_done,
            loudness_done,
            thumbnails_done,
            waveform_done,
            scenes_done,
            subtitles_done,
            ..
        } = &progress.stage
        {
            Some(vec![
                ("mediainfo", *mediainfo_done),
                ("loudness", *loudness_done),
                ("thumbnails", *thumbnails_done),
                ("waveform", *waveform_done),
                ("scenes", *scenes_done),
                ("subtitles", *subtitles_done),
            ])
        } else {
            None
        };

    rsx! {
        div { class: "rounded-xl border border-border bg-muted/20 px-4 py-4 space-y-3",
            div { class: "flex items-center gap-2",
                span { class: "size-2 rounded-full bg-primary animate-pulse inline-block" }
                p { class: "text-sm font-medium text-foreground", "{stage_label}" }
            }
            if let Some(rows) = enrichment_rows {
                div { class: "grid grid-cols-2 sm:grid-cols-4 gap-2",
                    for (label, done) in rows {
                        div {
                            class: if done {
                                "flex items-center gap-1.5 text-xs text-green-600 dark:text-green-400"
                            } else {
                                "flex items-center gap-1.5 text-xs text-muted-foreground animate-pulse"
                            },
                            span { if done { "✓" } else { "…" } }
                            span { "{label}" }
                        }
                    }
                }
            }
            p { class: "text-xs text-muted-foreground",
                "elapsed: {progress.elapsed_ms}ms"
                if !progress.file_name.is_empty() {
                    " · {progress.file_name}"
                }
                if progress.file_bytes > 0 {
                    " · {progress.file_bytes / 1024}KB"
                }
            }
        }
    }
}

// ── Start upload + polling ────────────────────────────────────────────────────

#[cfg(target_arch = "wasm32")]
async fn sleep_ms(ms: i32) {
    use wasm_bindgen::JsCast;
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms)
            .unwrap();
    });
    let _ = JsFuture::from(promise).await;
}

async fn start_upload(
    file: dioxus::html::FileData,
    on_progress: impl FnMut(u64, u64) + 'static,
) -> Result<String, String> {
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::closure::Closure;

        use crate::domain::media_read::data::diagnostic_progress::StartUploadResponse;

        let web_file = file
            .inner()
            .downcast_ref::<web_sys::File>()
            .cloned()
            .ok_or_else(|| "failed to access browser file handle".to_string())?;

        let xhr = web_sys::XmlHttpRequest::new().map_err(js_err)?;
        xhr.open("POST", "/api/media-read/start").map_err(js_err)?;
        xhr.set_request_header("Accept", "application/json")
            .map_err(js_err)?;

        // XHR upload progress (fetch API has no upload progress events)
        let upload = xhr.upload().map_err(js_err)?;
        let mut on_progress = on_progress;
        let prog_cb = Closure::wrap(Box::new(move |evt: web_sys::ProgressEvent| {
            if evt.length_computable() {
                on_progress(evt.loaded() as u64, evt.total() as u64);
            }
        }) as Box<dyn FnMut(web_sys::ProgressEvent)>);
        upload.set_onprogress(Some(prog_cb.as_ref().unchecked_ref()));
        prog_cb.forget();

        // Wrap onload/onerror as a Promise so we can .await
        let xhr_clone = xhr.clone();
        let promise = js_sys::Promise::new(&mut |resolve, reject| {
            let load_cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
                let _ = resolve.call0(&wasm_bindgen::JsValue::NULL);
            }) as Box<dyn FnMut(web_sys::Event)>);
            xhr_clone.set_onload(Some(load_cb.as_ref().unchecked_ref()));
            load_cb.forget();

            let err_cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
                let _ = reject.call1(
                    &wasm_bindgen::JsValue::NULL,
                    &wasm_bindgen::JsValue::from_str("xhr network error"),
                );
            }) as Box<dyn FnMut(web_sys::Event)>);
            xhr_clone.set_onerror(Some(err_cb.as_ref().unchecked_ref()));
            err_cb.forget();
        });

        let form = web_sys::FormData::new().map_err(js_err)?;
        form.append_with_blob_and_filename("file", &web_file, &web_file.name())
            .map_err(js_err)?;
        xhr.send_with_opt_form_data(Some(&form)).map_err(js_err)?;

        JsFuture::from(promise).await.map_err(js_err)?;

        let status = xhr.status().map_err(js_err)?;
        let body = xhr.response_text().ok().flatten().unwrap_or_default();

        if status >= 400 {
            if let Ok(e) = serde_json::from_str::<MediaProbeErrorResponse>(&body) {
                return Err(format!("HTTP {} [{}]: {}", status, e.trace_id, e.message));
            }
            return Err(format!("HTTP {}: {}", status, body));
        }

        let resp: StartUploadResponse = serde_json::from_str(&body).map_err(|e| e.to_string())?;
        Ok(resp.trace_id)
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (file, on_progress);
        Err("only available in browser build".into())
    }
}

async fn poll_progress(trace_id: &str) -> Result<DiagnosticProgress, String> {
    #[cfg(target_arch = "wasm32")]
    {
        let url = format!("/api/media-read/progress/{trace_id}");
        let options = web_sys::RequestInit::new();
        options.set_method("GET");

        let request = web_sys::Request::new_with_str_and_init(&url, &options).map_err(js_err)?;
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
            return Err(format!("poll HTTP {}: {}", response.status(), body));
        }

        serde_json::from_str::<DiagnosticProgress>(&body).map_err(|e| e.to_string())
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = trace_id;
        Err("only available in browser build".into())
    }
}

// ── Upload (legacy — still used by media_write_page) ─────────────────────────

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

        let request = web_sys::Request::new_with_str_and_init("/api/media-read/upload", &options)
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

// ── Tool error banner ─────────────────────────────────────────────────────────

#[component]
fn ToolErrorBanner(tool: &'static str, message: String) -> Element {
    rsx! {
        div { class: "rounded-lg border border-yellow-300 bg-yellow-50 dark:bg-yellow-950/20 dark:border-yellow-700 px-4 py-3 text-sm",
            span { class: "font-semibold text-yellow-800 dark:text-yellow-400", "{tool}: " }
            span { class: "text-yellow-700 dark:text-yellow-300 font-mono text-xs", "{message}" }
        }
    }
}

// ── Loudness panel ────────────────────────────────────────────────────────────

#[component]
fn LoudnessPanel(loudness: LoudnessReport) -> Element {
    let mut open = use_signal(|| true);

    let integrated: f64 = loudness.integrated_lufs.parse().unwrap_or(f64::NAN);
    let peak: f64 = loudness.true_peak_dbtp.parse().unwrap_or(f64::NAN);

    let lufs_class = if integrated.is_nan() {
        "text-muted-foreground"
    } else if (integrated - -23.0).abs() <= 1.0 {
        "text-green-600 dark:text-green-400 font-semibold"
    } else if (integrated - -23.0).abs() <= 3.0 {
        "text-yellow-600 dark:text-yellow-400 font-semibold"
    } else {
        "text-red-600 dark:text-red-400 font-semibold"
    };

    let peak_class = if peak.is_nan() {
        "text-muted-foreground"
    } else if peak <= -1.0 {
        "text-green-600 dark:text-green-400 font-semibold"
    } else {
        "text-red-600 dark:text-red-400 font-semibold"
    };

    rsx! {
        div { class: "rounded-xl border border-border overflow-hidden",
            button {
                class: "w-full flex items-center justify-between px-4 py-3 bg-muted/40 hover:bg-muted/60 text-sm font-semibold",
                onclick: move |_| open.toggle(),
                span { "R128 Loudness (EBU R128 / ATSC A/85)" }
                span { class: "text-xs text-muted-foreground", if open() { "▲" } else { "▼" } }
            }
            if open() {
                div { class: "px-4 py-3 grid grid-cols-2 gap-x-6 gap-y-2 text-sm",
                    KvRow { label: "Integrated", value: format!("{} LUFS", loudness.integrated_lufs), value_class: lufs_class }
                    KvRow { label: "Target (EBU R128)", value: "-23.0 LUFS".to_string(), value_class: "text-muted-foreground" }
                    KvRow { label: "Threshold", value: format!("{} LUFS", loudness.integrated_threshold), value_class: "text-muted-foreground" }
                    KvRow { label: "LRA", value: format!("{} LU", loudness.lra_lu), value_class: "text-foreground" }
                    KvRow { label: "LRA Low", value: format!("{} LUFS", loudness.lra_low), value_class: "text-muted-foreground" }
                    KvRow { label: "LRA High", value: format!("{} LUFS", loudness.lra_high), value_class: "text-muted-foreground" }
                    KvRow { label: "True Peak", value: format!("{} dBTP", loudness.true_peak_dbtp), value_class: peak_class }
                    KvRow { label: "Target (EBU R128)", value: "≤ -1.0 dBTP".to_string(), value_class: "text-muted-foreground" }
                }
            }
        }
    }
}

// ── MediaInfo panel ───────────────────────────────────────────────────────────

#[component]
fn MediaInfoPanel(mi: MediaInfoReport) -> Element {
    let mut open = use_signal(|| true);

    let rows: Vec<(&'static str, &str)> = vec![
        ("HDR Format", &mi.hdr_format),
        ("HDR Compatibility", &mi.hdr_format_compatibility),
        ("Format Profile", &mi.format_profile),
        ("Scan Order", &mi.scan_order),
        ("Standard", &mi.standard),
        ("Bit Depth", &mi.bit_depth),
        ("Frame Rate Num", &mi.frame_rate_num),
        ("Frame Rate Den", &mi.frame_rate_den),
        ("Audio Delay", &mi.audio_delay_ms),
        ("Writing Library", &mi.writing_library),
        ("Encoded Application", &mi.encoded_application),
    ];

    rsx! {
        div { class: "rounded-xl border border-border overflow-hidden",
            button {
                class: "w-full flex items-center justify-between px-4 py-3 bg-muted/40 hover:bg-muted/60 text-sm font-semibold",
                onclick: move |_| open.toggle(),
                span { "MediaInfo" }
                span { class: "text-xs text-muted-foreground", if open() { "▲" } else { "▼" } }
            }
            if open() {
                div { class: "px-4 py-3 grid grid-cols-2 gap-x-6 gap-y-2 text-sm",
                    for (key, val) in rows.iter().filter(|(_, v)| !v.is_empty()) {
                        KvRow { label: *key, value: val.to_string(), value_class: "text-foreground" }
                    }
                }
            }
        }
    }
}

#[component]
fn VideoPreviewPanel(preview_url: String) -> Element {
    let mut open = use_signal(|| true);

    rsx! {
        div { class: "rounded-xl border border-border overflow-hidden",
            button {
                class: "w-full flex items-center justify-between px-4 py-3 bg-muted/40 hover:bg-muted/60 text-sm font-semibold",
                onclick: move |_| open.toggle(),
                span { "Video Preview" }
                span { class: "text-xs text-muted-foreground", if open() { "▲" } else { "▼" } }
            }
            if open() {
                div { class: "px-4 py-4 bg-card space-y-3",
                    p { class: "text-xs text-muted-foreground",
                        "Local browser preview of the uploaded file for quick visual inspection."
                    }
                    video {
                        class: "aspect-video w-full rounded-lg border border-border bg-black object-contain",
                        src: "{preview_url}",
                        controls: true,
                        preload: "metadata",
                    }
                }
            }
        }
    }
}

#[component]
fn WaveformPanel(image_url: String) -> Element {
    let mut open = use_signal(|| true);

    rsx! {
        div { class: "rounded-xl border border-border overflow-hidden",
            button {
                class: "w-full flex items-center justify-between px-4 py-3 bg-muted/40 hover:bg-muted/60 text-sm font-semibold",
                onclick: move |_| open.toggle(),
                span { "Waveform" }
                span { class: "text-xs text-muted-foreground", if open() { "▲" } else { "▼" } }
            }
            if open() {
                div { class: "px-4 py-4 bg-card space-y-3",
                    p { class: "text-xs text-muted-foreground",
                        "Audio energy overview to spot silence, peaks, and section changes faster."
                    }
                    div { class: "rounded-lg border border-slate-800 bg-slate-950 p-3",
                        img {
                            class: "w-full rounded object-contain",
                            src: "{image_url}",
                            alt: "Audio waveform preview"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SceneCutsPanel(cuts: Vec<MediaSceneCut>) -> Element {
    let mut open = use_signal(|| true);

    rsx! {
        div { class: "rounded-xl border border-border overflow-hidden",
            button {
                class: "w-full flex items-center justify-between px-4 py-3 bg-muted/40 hover:bg-muted/60 text-sm font-semibold",
                onclick: move |_| open.toggle(),
                span { "Scene Detection" }
                span { class: "text-xs text-muted-foreground",
                    "{cuts.len()} cut(s) "
                    if open() { "▲" } else { "▼" }
                }
            }
            if open() {
                div { class: "px-4 py-4 bg-card space-y-2",
                    p { class: "text-xs text-muted-foreground",
                        "Likely visual cut points detected from frame-to-frame changes."
                    }
                    div { class: "divide-y divide-border rounded-lg border border-border overflow-hidden",
                        for cut in cuts.iter() {
                            div { class: "flex items-center justify-between gap-3 px-3 py-2 text-sm",
                                span { class: "font-mono text-foreground", "{format_timestamp(cut.timestamp_secs)}" }
                                if !cut.score.is_empty() {
                                    span { class: "text-xs text-muted-foreground", "score {cut.score}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ── Raw stream data panel ─────────────────────────────────────────────────────

#[component]
fn RawStreamDataPanel(report: MediaProbeReport) -> Element {
    let mut open = use_signal(|| false);
    let video_count = report
        .streams
        .iter()
        .filter(|stream| stream.codec_type == "video")
        .count();
    let audio_count = report
        .streams
        .iter()
        .filter(|stream| stream.codec_type == "audio")
        .count();
    let subtitle_count = report
        .streams
        .iter()
        .filter(|stream| stream.codec_type == "subtitle")
        .count();

    rsx! {
        div { class: "rounded-xl border border-border overflow-hidden",
            button {
                class: "w-full flex items-center justify-between px-4 py-3 bg-muted/40 hover:bg-muted/60 text-sm font-semibold",
                onclick: move |_| open.toggle(),
                span { "Raw Stream Data" }
                span { class: "text-xs text-muted-foreground",
                    "{report.stream_count} streams  "
                    if open() { "▲" } else { "▼" }
                }
            }
            if open() {
                div { class: "divide-y divide-border",
                    div { class: "px-4 py-3 bg-card flex flex-wrap gap-2",
                        SummaryPill { label: "video", value: video_count.to_string() }
                        SummaryPill { label: "audio", value: audio_count.to_string() }
                        SummaryPill { label: "subtitle", value: subtitle_count.to_string() }
                        SummaryPill { label: "chapters", value: report.chapter_count.to_string() }
                    }
                    // Container / format section
                    div { class: "px-4 py-3 space-y-2",
                        p { class: "text-xs font-semibold uppercase tracking-wide text-muted-foreground mb-1", "Container" }
                        div { class: "grid grid-cols-2 gap-x-6 gap-y-1 text-sm",
                            KvRow { label: "Format", value: report.format_name.clone(), value_class: "text-foreground" }
                            KvRow { label: "Format (long)", value: report.format_long_name.clone(), value_class: "text-foreground" }
                            KvRow { label: "Duration", value: report.duration.clone(), value_class: "text-foreground" }
                            KvRow { label: "Size", value: report.size.clone(), value_class: "text-foreground" }
                            KvRow { label: "Bit rate", value: report.bit_rate.clone(), value_class: "text-foreground" }
                            KvRow { label: "Start time", value: report.start_time.clone(), value_class: "text-foreground" }
                            KvRow { label: "Probe score", value: report.probe_score.clone(), value_class: "text-foreground" }
                            KvRow { label: "Programs", value: report.program_count.to_string(), value_class: "text-foreground" }
                        }
                        if !report.format_tags.is_empty() {
                            p { class: "text-xs font-semibold uppercase tracking-wide text-muted-foreground mt-2 mb-1", "Container Tags" }
                            div { class: "grid grid-cols-2 gap-x-6 gap-y-1 text-sm",
                                for tag in &report.format_tags {
                                    KvRow { label: tag.key.clone(), value: tag.value.clone(), value_class: "text-foreground" }
                                }
                            }
                        }
                    }
                    // Per-stream sections
                    for stream in &report.streams {
                        StreamSection { stream: stream.clone() }
                    }
                    // Chapters
                    if !report.chapters.is_empty() {
                        div { class: "px-4 py-3",
                            p { class: "text-xs font-semibold uppercase tracking-wide text-muted-foreground mb-1",
                                "Chapters ({report.chapter_count})"
                            }
                            div { class: "grid grid-cols-2 gap-x-6 gap-y-1 text-sm",
                                for ch in &report.chapters {
                                    KvRow { label: ch.id.to_string(), value: format!("{} → {}", ch.start, ch.end), value_class: "text-foreground" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn StreamSection(stream: MediaStreamInfo) -> Element {
    let title = stream_section_title(&stream);
    let summary = stream_summary(&stream);

    let mut rows: Vec<(&'static str, String)> = vec![
        ("Codec", stream.codec_long_name.clone()),
        ("Profile", stream.profile.clone()),
        ("Codec tag", stream.codec_tag.clone()),
        ("Duration", stream.duration.clone()),
        ("Bit rate", stream.bit_rate.clone()),
        ("Max bit rate", stream.max_bit_rate.clone()),
        ("Frame count", stream.frame_count.clone()),
        ("Nb read frames", stream.nb_read_frames.clone()),
    ];

    if stream.codec_type == "video" {
        rows.extend([
            (
                "Size",
                format!(
                    "{}x{} (coded {}x{})",
                    stream.width, stream.height, stream.coded_width, stream.coded_height
                ),
            ),
            ("DAR", stream.display_aspect_ratio.clone()),
            ("SAR", stream.sample_aspect_ratio.clone()),
            ("Frame rate", stream.frame_rate.clone()),
            ("Avg frame rate", stream.avg_frame_rate.clone()),
            ("R frame rate", stream.r_frame_rate.clone()),
            ("Pixel format", stream.pixel_format.clone()),
            ("Bits/raw sample", stream.bits_per_raw_sample.clone()),
            ("Level", stream.level.clone()),
            ("Field order", stream.field_order.clone()),
            ("Chroma location", stream.chroma_location.clone()),
            ("Color range", stream.color_range.clone()),
            ("Color space", stream.color_space.clone()),
            ("Color transfer", stream.color_transfer.clone()),
            ("Color primaries", stream.color_primaries.clone()),
            ("Has B-frames", stream.has_b_frames.clone()),
            ("Refs", stream.refs.clone()),
            ("Is AVC", stream.is_avc.clone()),
            ("NAL length size", stream.nal_length_size.clone()),
            ("Codec time base", stream.codec_time_base.clone()),
            ("Extradata size", stream.extradata_size.clone()),
            ("Closed captions", stream.closed_captions.clone()),
        ]);
    } else if stream.codec_type == "audio" {
        rows.extend([
            ("Sample rate", stream.sample_rate.clone()),
            ("Channels", stream.channels.clone()),
            ("Channel layout", stream.channel_layout.clone()),
            ("Sample format", stream.sample_format.clone()),
            ("Bits/sample", stream.bits_per_sample.clone()),
            ("Bits/raw sample", stream.bits_per_raw_sample.clone()),
            ("Initial padding", stream.initial_padding.clone()),
        ]);
    }

    rows.extend([
        ("Time base", stream.time_base.clone()),
        ("Start time", stream.start_time.clone()),
        ("Stream ID", stream.stream_id.clone()),
    ]);

    rsx! {
        div { class: "px-4 py-3 space-y-2",
            div { class: "flex flex-wrap items-center gap-2 mb-1",
                p { class: "text-xs font-semibold uppercase tracking-wide text-muted-foreground", "{title}" }
                if !summary.is_empty() {
                    span { class: "inline-flex items-center rounded-full bg-muted px-2 py-0.5 text-[11px] text-muted-foreground",
                        "{summary}"
                    }
                }
            }
            div { class: "grid grid-cols-2 gap-x-6 gap-y-1 text-sm",
                for (key, val) in rows.iter().filter(|(_, v)| !v.is_empty()) {
                    KvRow { label: *key, value: val.clone(), value_class: "text-foreground" }
                }
            }
            if !stream.disposition.is_empty() {
                p { class: "text-xs font-medium text-muted-foreground mt-2", "Disposition" }
                div { class: "grid grid-cols-2 gap-x-6 gap-y-1 text-sm",
                    for d in stream.disposition.iter().filter(|kv| kv.value != "0") {
                        KvRow { label: d.key.clone(), value: d.value.clone(), value_class: "text-foreground" }
                    }
                }
            }
            if !stream.side_data.is_empty() {
                p { class: "text-xs font-medium text-muted-foreground mt-2", "Side Data" }
                div { class: "space-y-1 text-sm",
                    for sd in &stream.side_data {
                        div { class: "rounded bg-muted/50 px-3 py-1.5",
                            p { class: "text-xs font-semibold text-primary", "{sd.key}" }
                            p { class: "text-xs text-muted-foreground font-mono break-all", "{sd.value}" }
                        }
                    }
                }
            }
            if !stream.tags.is_empty() {
                p { class: "text-xs font-medium text-muted-foreground mt-2", "Tags" }
                div { class: "grid grid-cols-2 gap-x-6 gap-y-1 text-sm",
                    for tag in &stream.tags {
                        KvRow { label: tag.key.clone(), value: tag.value.clone(), value_class: "text-foreground" }
                    }
                }
            }
        }
    }
}

// ── Shared ────────────────────────────────────────────────────────────────────

#[component]
fn SummaryPill(label: String, value: String) -> Element {
    rsx! {
        span { class: "inline-flex items-center gap-1 rounded-full border border-border bg-muted/40 px-2.5 py-1 text-[11px] text-muted-foreground",
            span { class: "font-semibold text-foreground", "{value}" }
            span { "{label}" }
        }
    }
}

fn stream_section_title(stream: &MediaStreamInfo) -> String {
    let kind = match stream.codec_type.as_str() {
        "video" => "Video",
        "audio" => "Audio",
        "subtitle" => "Subtitle",
        other if !other.is_empty() => other,
        _ => "Stream",
    };
    if stream.codec_name.is_empty() {
        format!("#{0} {kind}", stream.index)
    } else {
        format!("#{0} {kind} — {1}", stream.index, stream.codec_name)
    }
}

fn stream_summary(stream: &MediaStreamInfo) -> String {
    match stream.codec_type.as_str() {
        "video" => {
            let mut parts = vec![];
            if !stream.width.is_empty() && !stream.height.is_empty() {
                parts.push(format!("{}x{}", stream.width, stream.height));
            }
            if !stream.frame_rate.is_empty() {
                parts.push(stream.frame_rate.clone());
            }
            if !stream.pixel_format.is_empty() {
                parts.push(stream.pixel_format.clone());
            }
            parts.join(" • ")
        }
        "audio" => {
            let mut parts = vec![];
            if !stream.channel_layout.is_empty() {
                parts.push(stream.channel_layout.clone());
            }
            if !stream.sample_rate.is_empty() {
                parts.push(format!("{} Hz", stream.sample_rate));
            }
            if !stream.codec_name.is_empty() && stream.sample_format.is_empty() {
                parts.push(stream.codec_name.clone());
            } else if !stream.sample_format.is_empty() {
                parts.push(stream.sample_format.clone());
            }
            parts.join(" • ")
        }
        "subtitle" => {
            let mut parts = vec![];
            if !stream.codec_name.is_empty() {
                parts.push(stream.codec_name.clone());
            }
            if let Some(language) = stream
                .tags
                .iter()
                .find(|kv| kv.key == "language" && !kv.value.is_empty())
                .map(|kv| kv.value.clone())
            {
                parts.push(language);
            }
            parts.join(" • ")
        }
        _ => String::new(),
    }
}

fn format_timestamp(timestamp_secs: f64) -> String {
    let total_ms = (timestamp_secs * 1000.0).round() as u64;
    let minutes = total_ms / 60_000;
    let seconds = (total_ms % 60_000) / 1000;
    let millis = total_ms % 1000;
    format!("{minutes:02}:{seconds:02}.{millis:03}")
}

fn browser_preview_url(file: &dioxus::html::FileData) -> Option<String> {
    #[cfg(target_arch = "wasm32")]
    {
        file.inner()
            .downcast_ref::<web_sys::File>()
            .and_then(|web_file| web_sys::Url::create_object_url_with_blob(web_file).ok())
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = file;
        None
    }
}

fn revoke_browser_preview_url(url: Option<String>) {
    #[cfg(target_arch = "wasm32")]
    if let Some(url) = url {
        web_sys::Url::revoke_object_url(&url).ok();
    }

    #[cfg(not(target_arch = "wasm32"))]
    let _ = url;
}

#[component]
fn KvRow(label: String, value: String, value_class: &'static str) -> Element {
    rsx! {
        div { class: "contents",
            span { class: "text-muted-foreground truncate", "{label}" }
            span { class: "font-mono text-xs break-all {value_class}", "{value}" }
        }
    }
}
