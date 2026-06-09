use dioxus::prelude::*;

use crate::components::ui::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};
use crate::domain::media_read::routing::media_read_page::MediaReadPage;

#[component]
pub fn MediaWritePage() -> Element {
    rsx! {
        div { class: "max-w-[1100px] mx-auto w-full px-6 py-8 space-y-6",
            div { class: "space-y-2",
                h1 { class: "text-xl font-semibold", "Media Write Test" }
                p { class: "text-sm text-muted-foreground",
                    "Public test page for the future read -> write workflow. "
                    "The read domain stays focused on inspection. The write domain will own compression and artifact generation."
                }
            }

            MediaReadPage {}

            Card {
                CardHeader {
                    CardTitle { "Create Write Job" }
                    CardDescription {
                        "First target: compact server-side compression."
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
                            "input file\n  -> read with ffprobe\n  -> choose action: compress\n  -> create media_write job\n  -> ffmpeg runs on server\n  -> output artifact + logs + progress"
                        }
                    }
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
