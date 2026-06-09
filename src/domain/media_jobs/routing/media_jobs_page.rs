use dioxus::prelude::*;

use crate::components::ui::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};
use crate::domain::media_inspector::routing::media_inspector_page::MediaInspectorPage;

#[component]
pub fn MediaJobsPage() -> Element {
    rsx! {
        div { class: "max-w-[1100px] mx-auto w-full px-6 py-8 space-y-6",
            div { class: "space-y-2",
                h1 { class: "text-xl font-semibold", "Media Jobs Test" }
                p { class: "text-sm text-muted-foreground",
                    "Public test page for the future inspect -> create job workflow. "
                    "The inspector stays read-only in its own domain. The job UI below is a separate domain surface for compression and transcode work."
                }
            }

            MediaInspectorPage {}

            Card {
                CardHeader {
                    CardTitle { "Create Job" }
                    CardDescription {
                        "First target: a compact compression job UI, separate from media inspection."
                    }
                }
                CardContent { class: "space-y-4",
                    div { class: "grid gap-3 md:grid-cols-2 xl:grid-cols-4",
                        CompactField { label: "Action", value: "Compress" }
                        CompactField { label: "Container", value: "mp4" }
                        CompactField { label: "Video codec", value: "h264" }
                        CompactField { label: "Audio codec", value: "aac" }
                        CompactField { label: "Preset", value: "fast" }
                        CompactField { label: "Quality", value: "crf 23" }
                        CompactField { label: "Audio bitrate", value: "128k" }
                        CompactField { label: "Output", value: "input-compressed.mp4" }
                    }

                    div { class: "rounded-xl border bg-muted/20 p-4 text-sm leading-6",
                        p { class: "font-medium", "ASCII flow" }
                        pre { class: "mt-3 overflow-auto font-mono text-xs leading-5 whitespace-pre-wrap",
                            "input file\n  -> inspect with ffprobe\n  -> choose action: compress\n  -> create media job\n  -> ffmpeg runs on server\n  -> output artifact + logs + progress"
                        }
                    }
                }
            }

            Card {
                CardHeader {
                    CardTitle { "Planned Job Status" }
                    CardDescription {
                        "This is the future runtime panel once media_jobs gets real server logic."
                    }
                }
                CardContent { class: "grid gap-3 md:grid-cols-2 xl:grid-cols-4",
                    CompactField { label: "Status", value: "queued -> running -> done" }
                    CompactField { label: "Progress", value: "42%" }
                    CompactField { label: "Speed", value: "2.3x" }
                    CompactField { label: "ETA", value: "00:14" }
                    CompactField { label: "Before", value: "7.9 MB" }
                    CompactField { label: "After", value: "2.1 MB" }
                    CompactField { label: "Saved", value: "72.8%" }
                    CompactField { label: "Output", value: "download artifact" }
                }
            }
        }
    }
}

#[component]
fn CompactField(label: String, value: String) -> Element {
    rsx! {
        div { class: "rounded-lg border bg-background px-3 py-2",
            p { class: "text-[11px] uppercase tracking-wide text-muted-foreground", "{label}" }
            p { class: "mt-1 text-sm font-medium break-all", "{value}" }
        }
    }
}
