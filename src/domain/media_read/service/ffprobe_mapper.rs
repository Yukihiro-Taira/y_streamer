#[cfg(feature = "server")]
use std::path::Path;

#[cfg(feature = "server")]
use serde::Deserialize;
#[cfg(feature = "server")]
use serde_json::Value;

#[cfg(feature = "server")]
use crate::domain::media_read::data::media_probe_report::{
    MediaChapterInfo, MediaKeyValue, MediaProbeReport, MediaStreamInfo,
};

#[cfg(feature = "server")]
pub(crate) fn map_ffprobe_report(
    trace_id: &str,
    path: &Path,
    file_name: &str,
    upload_bytes: u64,
    processing_time_ms: u64,
    ffprobe_timeout_secs: u64,
    parsed: FfprobeOutput,
    raw_json_pretty: String,
) -> MediaProbeReport {
    let streams = parsed
        .streams
        .into_iter()
        .map(map_stream_info)
        .collect::<Vec<_>>();

    let video_count = streams.iter().filter(|s| s.codec_type == "video").count();
    let audio_count = streams.iter().filter(|s| s.codec_type == "audio").count();
    let subtitle_count = streams
        .iter()
        .filter(|s| s.codec_type == "subtitle")
        .count();

    let chapters = parsed
        .chapters
        .into_iter()
        .map(map_chapter_info)
        .collect::<Vec<_>>();

    let program_count = parsed.programs.len();
    let stream_group_count = parsed.stream_groups.len();
    let format = parsed.format.unwrap_or_default();

    MediaProbeReport {
        trace_id: trace_id.to_string(),
        file_name: file_name.to_string(),
        path_hint: path.display().to_string(),
        upload_bytes,
        processing_time_ms,
        ffprobe_timeout_secs,
        format_name: format.format_name.unwrap_or_else(|| "unknown".into()),
        format_long_name: format.format_long_name.unwrap_or_default(),
        duration: display_numeric_option(format.duration.as_deref(), "s"),
        size: display_numeric_option(format.size.as_deref(), "bytes"),
        bit_rate: display_numeric_option(format.bit_rate.as_deref(), "bps"),
        probe_score: json_string_or_default(format.probe_score),
        start_time: format.start_time.unwrap_or_default(),
        program_count: if program_count > 0 {
            program_count
        } else {
            format.nb_programs.unwrap_or_default() as usize
        },
        stream_group_count: if stream_group_count > 0 {
            stream_group_count
        } else {
            format.nb_stream_groups.unwrap_or_default() as usize
        },
        stream_count: streams.len(),
        video_count,
        audio_count,
        subtitle_count,
        chapter_count: chapters.len(),
        format_tags: flatten_tags(format.tags),
        streams,
        chapters,
        raw_json_pretty,
    }
}

#[cfg(feature = "server")]
pub(crate) fn parse_ffprobe_output(stdout: &[u8]) -> anyhow::Result<(FfprobeOutput, String)> {
    use anyhow::Context;

    let raw_value: Value =
        serde_json::from_slice(stdout).context("failed to parse ffprobe json")?;
    let raw_json_pretty =
        serde_json::to_string_pretty(&raw_value).context("failed to pretty-print ffprobe json")?;
    let parsed: FfprobeOutput =
        serde_json::from_value(raw_value).context("failed to deserialize ffprobe json")?;
    Ok((parsed, raw_json_pretty))
}

#[cfg(feature = "server")]
fn map_stream_info(stream: FfprobeStream) -> MediaStreamInfo {
    MediaStreamInfo {
        index: stream.index.unwrap_or_default(),
        stream_id: stream.id.unwrap_or_default(),
        codec_type: stream.codec_type.unwrap_or_else(|| "unknown".into()),
        codec_name: stream.codec_name.unwrap_or_default(),
        codec_long_name: stream.codec_long_name.unwrap_or_default(),
        profile: stream.profile.unwrap_or_default(),
        codec_tag: join_non_empty([
            stream.codec_tag_string.unwrap_or_default(),
            stream.codec_tag.unwrap_or_default(),
        ]),
        duration: display_numeric_option(stream.duration.as_deref(), "s"),
        bit_rate: display_numeric_option(stream.bit_rate.as_deref(), "bps"),
        width: stream.width.map(|n| n.to_string()).unwrap_or_default(),
        height: stream.height.map(|n| n.to_string()).unwrap_or_default(),
        coded_width: stream
            .coded_width
            .map(|n| n.to_string())
            .unwrap_or_default(),
        coded_height: stream
            .coded_height
            .map(|n| n.to_string())
            .unwrap_or_default(),
        display_aspect_ratio: stream.display_aspect_ratio.unwrap_or_default(),
        sample_aspect_ratio: stream.sample_aspect_ratio.unwrap_or_default(),
        frame_rate: display_frame_rate(stream.avg_frame_rate, stream.r_frame_rate),
        pixel_format: stream.pix_fmt.unwrap_or_default(),
        sample_format: stream.sample_fmt.unwrap_or_default(),
        level: stream.level.map(|n| n.to_string()).unwrap_or_default(),
        field_order: stream.field_order.unwrap_or_default(),
        chroma_location: stream.chroma_location.unwrap_or_default(),
        color_range: stream.color_range.unwrap_or_default(),
        color_space: stream.color_space.unwrap_or_default(),
        color_transfer: stream.color_transfer.unwrap_or_default(),
        color_primaries: stream.color_primaries.unwrap_or_default(),
        sample_rate: stream.sample_rate.unwrap_or_default(),
        channels: stream.channels.map(|n| n.to_string()).unwrap_or_default(),
        channel_layout: stream.channel_layout.unwrap_or_default(),
        bits_per_sample: stream
            .bits_per_sample
            .map(|n| n.to_string())
            .unwrap_or_default(),
        bits_per_raw_sample: stream.bits_per_raw_sample.unwrap_or_default(),
        time_base: stream.time_base.unwrap_or_default(),
        start_time: stream.start_time.unwrap_or_default(),
        frame_count: stream.nb_frames.unwrap_or_default(),
        refs: stream.refs.map(|n| n.to_string()).unwrap_or_default(),
        has_b_frames: stream
            .has_b_frames
            .map(|n| n.to_string())
            .unwrap_or_default(),
        nal_length_size: stream.nal_length_size.unwrap_or_default(),
        is_avc: stream.is_avc.unwrap_or_default(),
        disposition: stream
            .disposition
            .map(flatten_object_pairs)
            .unwrap_or_default(),
        tags: flatten_tags(stream.tags),
    }
}

#[cfg(feature = "server")]
fn map_chapter_info(chapter: FfprobeChapter) -> MediaChapterInfo {
    MediaChapterInfo {
        id: chapter.id.unwrap_or_default(),
        start: display_numeric_option(chapter.start_time.as_deref(), "s"),
        end: display_numeric_option(chapter.end_time.as_deref(), "s"),
        time_base: chapter.time_base.unwrap_or_default(),
        tags: flatten_tags(chapter.tags),
    }
}

#[cfg(feature = "server")]
fn flatten_tags(value: Option<Value>) -> Vec<MediaKeyValue> {
    value.map(flatten_object_pairs).unwrap_or_default()
}

#[cfg(feature = "server")]
fn flatten_object_pairs(value: Value) -> Vec<MediaKeyValue> {
    match value {
        Value::Object(map) => map
            .into_iter()
            .map(|(key, value)| MediaKeyValue {
                key,
                value: json_value_to_string(value),
            })
            .collect(),
        _ => Vec::new(),
    }
}

#[cfg(feature = "server")]
fn json_value_to_string(value: Value) -> String {
    match value {
        Value::Null => String::new(),
        Value::Bool(v) => v.to_string(),
        Value::Number(v) => v.to_string(),
        Value::String(v) => v,
        Value::Array(values) => values
            .into_iter()
            .map(json_value_to_string)
            .collect::<Vec<_>>()
            .join(", "),
        Value::Object(map) => map
            .into_iter()
            .map(|(k, v)| format!("{k}={}", json_value_to_string(v)))
            .collect::<Vec<_>>()
            .join(", "),
    }
}

#[cfg(feature = "server")]
fn json_string_or_default(value: Option<Value>) -> String {
    value.map(json_value_to_string).unwrap_or_default()
}

#[cfg(feature = "server")]
fn display_numeric_option(raw: Option<&str>, suffix: &str) -> String {
    match raw.and_then(|value| value.parse::<f64>().ok()) {
        Some(number) if suffix == "s" => format!("{number:.3} s"),
        Some(number) if suffix == "bytes" => format!("{number:.0} bytes"),
        Some(number) if suffix == "bps" => format!("{number:.0} bps"),
        Some(number) => format!("{number} {suffix}"),
        None => raw.unwrap_or_default().to_string(),
    }
}

#[cfg(feature = "server")]
fn display_frame_rate(avg: Option<String>, raw: Option<String>) -> String {
    let value = avg.or(raw).unwrap_or_default();
    let parts = value.split('/').collect::<Vec<_>>();
    if parts.len() == 2 {
        if let (Ok(num), Ok(den)) = (parts[0].parse::<f64>(), parts[1].parse::<f64>()) {
            if den != 0.0 {
                return format!("{:.3} fps", num / den);
            }
        }
    }
    value
}

#[cfg(feature = "server")]
fn join_non_empty(values: [String; 2]) -> String {
    values
        .into_iter()
        .filter(|value| !value.trim().is_empty())
        .collect::<Vec<_>>()
        .join(" | ")
}

#[derive(Debug, Deserialize, Default)]
#[cfg(feature = "server")]
pub(crate) struct FfprobeOutput {
    #[serde(default)]
    streams: Vec<FfprobeStream>,
    #[serde(default)]
    chapters: Vec<FfprobeChapter>,
    #[serde(default)]
    programs: Vec<Value>,
    #[serde(default)]
    stream_groups: Vec<Value>,
    format: Option<FfprobeFormat>,
}

#[derive(Debug, Deserialize, Default)]
#[cfg(feature = "server")]
struct FfprobeFormat {
    nb_programs: Option<i64>,
    nb_stream_groups: Option<i64>,
    format_name: Option<String>,
    format_long_name: Option<String>,
    duration: Option<String>,
    size: Option<String>,
    bit_rate: Option<String>,
    probe_score: Option<Value>,
    start_time: Option<String>,
    tags: Option<Value>,
}

#[derive(Debug, Deserialize, Default)]
#[cfg(feature = "server")]
struct FfprobeStream {
    index: Option<i64>,
    codec_name: Option<String>,
    codec_long_name: Option<String>,
    profile: Option<String>,
    codec_type: Option<String>,
    codec_tag_string: Option<String>,
    codec_tag: Option<String>,
    id: Option<String>,
    width: Option<i64>,
    height: Option<i64>,
    coded_width: Option<i64>,
    coded_height: Option<i64>,
    pix_fmt: Option<String>,
    sample_fmt: Option<String>,
    level: Option<i64>,
    field_order: Option<String>,
    chroma_location: Option<String>,
    sample_aspect_ratio: Option<String>,
    display_aspect_ratio: Option<String>,
    color_range: Option<String>,
    color_space: Option<String>,
    color_transfer: Option<String>,
    color_primaries: Option<String>,
    avg_frame_rate: Option<String>,
    r_frame_rate: Option<String>,
    sample_rate: Option<String>,
    channels: Option<i64>,
    channel_layout: Option<String>,
    bits_per_raw_sample: Option<String>,
    bits_per_sample: Option<i64>,
    time_base: Option<String>,
    start_time: Option<String>,
    duration: Option<String>,
    bit_rate: Option<String>,
    nb_frames: Option<String>,
    refs: Option<i64>,
    has_b_frames: Option<i64>,
    is_avc: Option<String>,
    nal_length_size: Option<String>,
    disposition: Option<Value>,
    tags: Option<Value>,
}

#[derive(Debug, Deserialize, Default)]
#[cfg(feature = "server")]
struct FfprobeChapter {
    id: Option<i64>,
    time_base: Option<String>,
    start_time: Option<String>,
    end_time: Option<String>,
    tags: Option<Value>,
}
