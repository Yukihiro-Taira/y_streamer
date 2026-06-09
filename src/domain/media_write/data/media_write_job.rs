use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MediaWriteResult {
    pub trace_id: String,
    pub job_id: String,
    pub operation: String,
    pub output_file_name: String,
    pub download_url: String,
    pub output_container: String,
    pub video_codec: String,
    pub audio_codec: String,
    pub input_bytes: u64,
    pub output_bytes: u64,
    pub saved_bytes: u64,
    pub saved_percent: f64,
    pub elapsed_ms: u64,
    pub ffmpeg_timeout_secs: u64,
    pub command_summary: String,
    pub stderr_excerpt: String,
}

#[cfg_attr(not(any(feature = "server", target_arch = "wasm32")), allow(dead_code))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MediaWriteErrorResponse {
    pub trace_id: String,
    pub message: String,
}
