use serde::{Deserialize, Serialize};

use crate::domain::media_read::data::media_probe_report::MediaProbeReport;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiagnosticProgress {
    pub trace_id: String,
    pub file_name: String,
    pub file_bytes: u64,
    pub elapsed_ms: u64,
    pub stage: ProgressStage,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum ProgressStage {
    Uploading,
    RunningFfprobe {
        upload_ms: u64,
    },
    RunningEnrichment {
        ffprobe_ms: u64,
        stream_count: u32,
        video_codec: Option<String>,
        resolution: Option<String>,
        audio_codec: Option<String>,
        duration_label: Option<String>,
        mediainfo_done: bool,
        loudness_done: bool,
        thumbnails_done: bool,
        subtitles_done: bool,
    },
    Done {
        report: Box<MediaProbeReport>,
    },
    Failed {
        message: String,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StartUploadResponse {
    pub trace_id: String,
}
