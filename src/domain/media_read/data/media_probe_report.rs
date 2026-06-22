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
    /// Extracted subtitle streams, empty if none found
    pub subtitles: Vec<MediaSubtitle>,
    /// mediainfo report, None if not installed or failed (see mediainfo_error)
    pub mediainfo: Option<MediaInfoReport>,
    /// set when mediainfo is None due to an error ("not installed", etc.)
    pub mediainfo_error: Option<String>,
    /// R128 loudness analysis, None if no audio or failed (see loudness_error)
    pub loudness: Option<LoudnessReport>,
    /// set when loudness is None due to an error
    pub loudness_error: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MediaSubtitle {
    pub stream_index: usize,
    pub language: String,
    pub content: String,
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
    pub avg_frame_rate: String,
    pub r_frame_rate: String,
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
    pub codec_time_base: String,
    pub max_bit_rate: String,
    pub nb_read_frames: String,
    pub extradata_size: String,
    pub initial_padding: String,
    pub closed_captions: String,
    pub disposition: Vec<MediaKeyValue>,
    pub tags: Vec<MediaKeyValue>,
    pub side_data: Vec<MediaKeyValue>,
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MediaInfoReport {
    /// e.g. "HDR10", "Dolby Vision", "HLG", "" if SDR
    pub hdr_format: String,
    /// e.g. "HDR10 / Dolby Vision" compatibility string
    pub hdr_format_compatibility: String,
    /// e.g. "x264 core 164 r3095", "DaVinci Resolve Studio 18.6"
    pub writing_library: String,
    /// NLE / encoder application name
    pub encoded_application: String,
    /// Audio delay relative to video in ms
    pub audio_delay_ms: String,
    /// "NTSC", "PAL", "" if unknown
    pub standard: String,
    /// e.g. "High@L4.0"
    pub format_profile: String,
    /// Frame rate numerator (e.g. "24000")
    pub frame_rate_num: String,
    /// Frame rate denominator (e.g. "1001")
    pub frame_rate_den: String,
    /// Interlace scan order: "TFF", "BFF", "" if progressive
    pub scan_order: String,
    /// Bit depth from mediainfo (sometimes more accurate than ffprobe)
    pub bit_depth: String,
    /// Full mediainfo JSON output
    pub raw_json: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LoudnessReport {
    /// Integrated loudness in LUFS (e.g. "-23.0")
    pub integrated_lufs: String,
    /// Loudness threshold used for integrated measurement
    pub integrated_threshold: String,
    /// Loudness range in LU (e.g. "7.2")
    pub lra_lu: String,
    /// LRA low in LUFS
    pub lra_low: String,
    /// LRA high in LUFS
    pub lra_high: String,
    /// True peak in dBTP (e.g. "-1.2")
    pub true_peak_dbtp: String,
}
