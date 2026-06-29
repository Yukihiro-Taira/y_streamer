use crate::domain::media_read::data::media_probe_report::{
    MediaKeyValue, MediaProbeReport, MediaStreamInfo,
};

#[derive(Clone, Debug, PartialEq)]
pub struct CompareFieldRow {
    pub label: String,
    pub left_value: String,
    pub right_value: String,
    pub same: bool,
}

pub fn build_compare_rows(left: &MediaProbeReport, right: &MediaProbeReport) -> Vec<CompareFieldRow> {
    let left_video = left.streams.iter().find(|stream| stream.codec_type == "video");
    let right_video = right.streams.iter().find(|stream| stream.codec_type == "video");
    let left_audio = left.streams.iter().find(|stream| stream.codec_type == "audio");
    let right_audio = right.streams.iter().find(|stream| stream.codec_type == "audio");

    vec![
        compare_row("Container", &left.format_name, &right.format_name),
        compare_row(
            "Video codec",
            &stream_value(left_video, |stream| stream.codec_name.clone()),
            &stream_value(right_video, |stream| stream.codec_name.clone()),
        ),
        compare_row(
            "Audio codec",
            &stream_value(left_audio, |stream| stream.codec_name.clone()),
            &stream_value(right_audio, |stream| stream.codec_name.clone()),
        ),
        compare_row(
            "Resolution",
            &stream_value(left_video, video_resolution),
            &stream_value(right_video, video_resolution),
        ),
        compare_row(
            "Frame rate",
            &stream_value(left_video, |stream| stream.frame_rate.clone()),
            &stream_value(right_video, |stream| stream.frame_rate.clone()),
        ),
        compare_row(
            "Pixel format",
            &stream_value(left_video, |stream| stream.pixel_format.clone()),
            &stream_value(right_video, |stream| stream.pixel_format.clone()),
        ),
        compare_row(
            "Color depth",
            &stream_value(left_video, |stream| normalize_empty(&stream.bits_per_raw_sample, "8")),
            &stream_value(right_video, |stream| normalize_empty(&stream.bits_per_raw_sample, "8")),
        ),
        compare_row(
            "Audio channels",
            &stream_value(left_audio, |stream| stream.channel_layout.clone()),
            &stream_value(right_audio, |stream| stream.channel_layout.clone()),
        ),
        compare_row(
            "Sample rate",
            &stream_value(left_audio, |stream| stream.sample_rate.clone()),
            &stream_value(right_audio, |stream| stream.sample_rate.clone()),
        ),
        compare_row(
            "Subtitles",
            &left.subtitle_count.to_string(),
            &right.subtitle_count.to_string(),
        ),
        compare_row(
            "Timecode",
            &report_timecode(left, left_video),
            &report_timecode(right, right_video),
        ),
        compare_row(
            "Encoder",
            &report_tag(left, "encoder", "encoding_tool"),
            &report_tag(right, "encoder", "encoding_tool"),
        ),
        compare_row(
            "Creation time",
            &report_tag(left, "creation_time", ""),
            &report_tag(right, "creation_time", ""),
        ),
        compare_row(
            "Writing library",
            &mediainfo_value(left, |mi| mi.writing_library.clone()),
            &mediainfo_value(right, |mi| mi.writing_library.clone()),
        ),
        compare_row(
            "Encoded application",
            &mediainfo_value(left, |mi| mi.encoded_application.clone()),
            &mediainfo_value(right, |mi| mi.encoded_application.clone()),
        ),
        compare_row(
            "HDR format",
            &mediainfo_value(left, |mi| mi.hdr_format.clone()),
            &mediainfo_value(right, |mi| mi.hdr_format.clone()),
        ),
        compare_row(
            "Audio language tags",
            &stream_language_summary(left, "audio"),
            &stream_language_summary(right, "audio"),
        ),
        compare_row(
            "Subtitle language tags",
            &stream_language_summary(left, "subtitle"),
            &stream_language_summary(right, "subtitle"),
        ),
    ]
}

fn compare_row(label: &str, left_value: &str, right_value: &str) -> CompareFieldRow {
    let left_value = display_compare_value(left_value);
    let right_value = display_compare_value(right_value);
    let same = left_value == right_value;
    CompareFieldRow {
        label: label.to_string(),
        left_value,
        right_value,
        same,
    }
}

fn display_compare_value(value: &str) -> String {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        "missing".to_string()
    } else {
        trimmed.to_string()
    }
}

fn stream_value(
    stream: Option<&MediaStreamInfo>,
    map: impl FnOnce(&MediaStreamInfo) -> String,
) -> String {
    stream.map(map).unwrap_or_default()
}

fn video_resolution(stream: &MediaStreamInfo) -> String {
    if !stream.width.is_empty() && !stream.height.is_empty() {
        format!("{}x{}", stream.width, stream.height)
    } else {
        String::new()
    }
}

fn report_timecode(report: &MediaProbeReport, video: Option<&MediaStreamInfo>) -> String {
    find_report_tag(report, "timecode")
        .or_else(|| {
            video.and_then(|stream| {
                stream
                    .tags
                    .iter()
                    .find(|tag| tag.key.eq_ignore_ascii_case("timecode"))
                    .map(|tag| tag.value.clone())
            })
        })
        .unwrap_or_default()
}

fn report_tag(report: &MediaProbeReport, primary: &str, fallback: &str) -> String {
    find_report_tag(report, primary)
        .or_else(|| {
            if fallback.is_empty() {
                None
            } else {
                find_report_tag(report, fallback)
            }
        })
        .unwrap_or_default()
}

fn mediainfo_value(
    report: &MediaProbeReport,
    map: impl FnOnce(&crate::domain::media_read::data::media_probe_report::MediaInfoReport) -> String,
) -> String {
    report.mediainfo.as_ref().map(map).unwrap_or_default()
}

fn find_report_tag(report: &MediaProbeReport, key: &str) -> Option<String> {
    find_tag_value(&report.format_tags, key)
}

fn find_tag_value(tags: &[MediaKeyValue], key: &str) -> Option<String> {
    tags.iter()
        .find(|tag| tag.key.eq_ignore_ascii_case(key) && !tag.value.trim().is_empty())
        .map(|tag| tag.value.clone())
}

fn normalize_empty(value: &str, fallback: &str) -> String {
    if value.trim().is_empty() {
        fallback.to_string()
    } else {
        value.to_string()
    }
}

fn stream_language_summary(report: &MediaProbeReport, codec_type: &str) -> String {
    let values = report
        .streams
        .iter()
        .filter(|stream| stream.codec_type == codec_type)
        .map(|stream| {
            find_tag_value(&stream.tags, "language").unwrap_or_else(|| "missing".to_string())
        })
        .collect::<Vec<_>>();

    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(", ")
    }
}
