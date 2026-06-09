use dioxus::html::HasFileData;
use dioxus::prelude::*;

use crate::components::ui::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};
use crate::domain::media_inspector::data::media_probe_report::{
    MediaChapterInfo, MediaKeyValue, MediaProbeReport, MediaStreamInfo,
};
use crate::domain::media_inspector::service::inspect_media_upload::inspect_media_upload;

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
                let file_name = file.name();
                match file.read_bytes().await {
                    Ok(bytes) => match inspect_media_upload(file_name, bytes.to_vec()).await {
                        Ok(result) => report.set(Some(result)),
                        Err(err) => error.set(Some(err.to_string())),
                    },
                    Err(err) => error.set(Some(err.to_string())),
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

#[component]
fn MediaInspectorReport(report: MediaProbeReport) -> Element {
    rsx! {
        div { class: "space-y-6",
            Card {
                CardHeader {
                    CardTitle { "Summary" }
                    CardDescription { "{report.file_name}" }
                }
                CardContent { class: "grid gap-3 md:grid-cols-2 xl:grid-cols-3",
                    ReportField { label: "Format", value: format!("{} ({})", report.format_name, report.format_long_name) }
                    ReportField { label: "Duration", value: report.duration.clone() }
                    ReportField { label: "Size", value: report.size.clone() }
                    ReportField { label: "Bit rate", value: report.bit_rate.clone() }
                    ReportField { label: "Start time", value: report.start_time.clone() }
                    ReportField { label: "Probe score", value: report.probe_score.clone() }
                    ReportField { label: "Programs", value: report.program_count.to_string() }
                    ReportField { label: "Stream groups", value: report.stream_group_count.to_string() }
                    ReportField { label: "Streams", value: format!("total={} video={} audio={} subtitle={}", report.stream_count, report.video_count, report.audio_count, report.subtitle_count) }
                    ReportField { label: "Chapters", value: report.chapter_count.to_string() }
                    ReportField { label: "Temp path", value: report.path_hint.clone() }
                }
            }

            if !report.format_tags.is_empty() {
                Card {
                    CardHeader { CardTitle { "Format Tags" } }
                    CardContent { class: "space-y-2",
                        for tag in report.format_tags {
                            KeyValueRow { item: tag }
                        }
                    }
                }
            }

            Card {
                CardHeader { CardTitle { "Streams" } }
                CardContent { class: "space-y-4",
                    for stream in report.streams {
                        StreamCard { stream }
                    }
                }
            }

            if !report.chapters.is_empty() {
                Card {
                    CardHeader { CardTitle { "Chapters" } }
                    CardContent { class: "space-y-4",
                        for chapter in report.chapters {
                            ChapterCard { chapter }
                        }
                    }
                }
            }

            Card {
                CardHeader {
                    CardTitle { "Raw ffprobe JSON" }
                    CardDescription { "Complete server-side ffprobe output." }
                }
                CardContent {
                    pre { class: "max-h-[720px] overflow-auto rounded-xl bg-muted/30 p-4 text-xs leading-5 whitespace-pre-wrap break-all",
                        "{report.raw_json_pretty}"
                    }
                }
            }
        }
    }
}

#[component]
fn StreamCard(stream: MediaStreamInfo) -> Element {
    rsx! {
        div { class: "rounded-2xl border border-border/60 p-4 space-y-3",
            div {
                h3 { class: "text-sm font-semibold",
                    "Stream #{stream.index} [{stream.codec_type}] {stream.codec_name}"
                }
                p { class: "text-xs text-muted-foreground", "{stream.codec_long_name}" }
            }
            div { class: "grid gap-3 md:grid-cols-2 xl:grid-cols-4",
                ReportField { label: "ID", value: stream.stream_id.clone() }
                ReportField { label: "Profile", value: stream.profile.clone() }
                ReportField { label: "Codec tag", value: stream.codec_tag.clone() }
                ReportField { label: "Duration", value: stream.duration.clone() }
                ReportField { label: "Bit rate", value: stream.bit_rate.clone() }
                ReportField { label: "Time base", value: stream.time_base.clone() }
                ReportField { label: "Start time", value: stream.start_time.clone() }
                ReportField { label: "Frames", value: stream.frame_count.clone() }
                ReportField { label: "Resolution", value: join_resolution(&stream.width, &stream.height) }
                ReportField { label: "Coded resolution", value: join_resolution(&stream.coded_width, &stream.coded_height) }
                ReportField { label: "Frame rate", value: stream.frame_rate.clone() }
                ReportField { label: "Pixel format", value: stream.pixel_format.clone() }
                ReportField { label: "Sample format", value: stream.sample_format.clone() }
                ReportField { label: "Sample rate", value: stream.sample_rate.clone() }
                ReportField { label: "Channels", value: stream.channels.clone() }
                ReportField { label: "Channel layout", value: stream.channel_layout.clone() }
                ReportField { label: "Display aspect", value: stream.display_aspect_ratio.clone() }
                ReportField { label: "Sample aspect", value: stream.sample_aspect_ratio.clone() }
                ReportField { label: "Field order", value: stream.field_order.clone() }
                ReportField { label: "Level", value: stream.level.clone() }
                ReportField { label: "Chroma location", value: stream.chroma_location.clone() }
                ReportField { label: "Color range", value: stream.color_range.clone() }
                ReportField { label: "Color space", value: stream.color_space.clone() }
                ReportField { label: "Color transfer", value: stream.color_transfer.clone() }
                ReportField { label: "Color primaries", value: stream.color_primaries.clone() }
                ReportField { label: "Bits/sample", value: stream.bits_per_sample.clone() }
                ReportField { label: "Bits/raw sample", value: stream.bits_per_raw_sample.clone() }
                ReportField { label: "Refs", value: stream.refs.clone() }
                ReportField { label: "B-frames", value: stream.has_b_frames.clone() }
                ReportField { label: "is_avc", value: stream.is_avc.clone() }
                ReportField { label: "NAL length size", value: stream.nal_length_size.clone() }
            }

            if !stream.disposition.is_empty() {
                div { class: "space-y-2",
                    h4 { class: "text-xs font-semibold uppercase tracking-wide text-muted-foreground", "Disposition" }
                    for item in stream.disposition {
                        KeyValueRow { item }
                    }
                }
            }

            if !stream.tags.is_empty() {
                div { class: "space-y-2",
                    h4 { class: "text-xs font-semibold uppercase tracking-wide text-muted-foreground", "Tags" }
                    for item in stream.tags {
                        KeyValueRow { item }
                    }
                }
            }
        }
    }
}

#[component]
fn ChapterCard(chapter: MediaChapterInfo) -> Element {
    rsx! {
        div { class: "rounded-2xl border border-border/60 p-4 space-y-3",
            div { class: "grid gap-3 md:grid-cols-3",
                ReportField { label: "ID", value: chapter.id.to_string() }
                ReportField { label: "Start", value: chapter.start.clone() }
                ReportField { label: "End", value: chapter.end.clone() }
                ReportField { label: "Time base", value: chapter.time_base.clone() }
            }
            if !chapter.tags.is_empty() {
                div { class: "space-y-2",
                    for item in chapter.tags {
                        KeyValueRow { item }
                    }
                }
            }
        }
    }
}

#[component]
fn ReportField(label: String, value: String) -> Element {
    rsx! {
        div { class: "rounded-xl bg-muted/30 px-3 py-2",
            p { class: "text-[11px] uppercase tracking-wide text-muted-foreground", "{label}" }
            p { class: "text-sm font-medium break-all", "{empty_to_dash(&value)}" }
        }
    }
}

#[component]
fn KeyValueRow(item: MediaKeyValue) -> Element {
    rsx! {
        div { class: "grid gap-1 rounded-xl bg-muted/20 px-3 py-2 md:grid-cols-[220px_1fr]",
            p { class: "text-xs font-medium text-muted-foreground break-all", "{item.key}" }
            p { class: "text-xs break-all", "{empty_to_dash(&item.value)}" }
        }
    }
}

fn join_resolution(width: &str, height: &str) -> String {
    if width.is_empty() && height.is_empty() {
        "—".into()
    } else {
        format!("{width} x {height}")
    }
}

fn empty_to_dash(value: &str) -> String {
    if value.trim().is_empty() {
        "—".into()
    } else {
        value.to_string()
    }
}
