use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MediaProbeReport {
    pub trace_id: String,
    pub file_name: String,
    pub path_hint: String,
    pub upload_bytes: u64,
    pub processing_time_ms: u64,
    pub ffprobe_timeout_secs: u64,
    pub format_name: String,
    pub format_long_name: String,
    pub duration: String,
    pub size: String,
    pub bit_rate: String,
    pub probe_score: String,
    pub start_time: String,
    pub program_count: usize,
    pub stream_group_count: usize,
    pub stream_count: usize,
    pub video_count: usize,
    pub audio_count: usize,
    pub subtitle_count: usize,
    pub chapter_count: usize,
    pub format_tags: Vec<MediaKeyValue>,
    pub streams: Vec<MediaStreamInfo>,
    pub chapters: Vec<MediaChapterInfo>,
    pub raw_json_pretty: String,
    /// base64 data URLs (data:image/jpeg;base64,...), empty if not a video or generation failed
    pub thumbnails: Vec<String>,
}

#[cfg_attr(not(any(feature = "server", target_arch = "wasm32")), allow(dead_code))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MediaProbeErrorResponse {
    pub trace_id: String,
    pub message: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MediaStreamInfo {
    pub index: i64,
    pub stream_id: String,
    pub codec_type: String,
    pub codec_name: String,
    pub codec_long_name: String,
    pub profile: String,
    pub codec_tag: String,
    pub duration: String,
    pub bit_rate: String,
    pub width: String,
    pub height: String,
    pub coded_width: String,
    pub coded_height: String,
    pub display_aspect_ratio: String,
    pub sample_aspect_ratio: String,
    pub frame_rate: String,
    pub pixel_format: String,
    pub sample_format: String,
    pub level: String,
    pub field_order: String,
    pub chroma_location: String,
    pub color_range: String,
    pub color_space: String,
    pub color_transfer: String,
    pub color_primaries: String,
    pub sample_rate: String,
    pub channels: String,
    pub channel_layout: String,
    pub bits_per_sample: String,
    pub bits_per_raw_sample: String,
    pub time_base: String,
    pub start_time: String,
    pub frame_count: String,
    pub refs: String,
    pub has_b_frames: String,
    pub nal_length_size: String,
    pub is_avc: String,
    pub disposition: Vec<MediaKeyValue>,
    pub tags: Vec<MediaKeyValue>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MediaChapterInfo {
    pub id: i64,
    pub start: String,
    pub end: String,
    pub time_base: String,
    pub tags: Vec<MediaKeyValue>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MediaKeyValue {
    pub key: String,
    pub value: String,
}
