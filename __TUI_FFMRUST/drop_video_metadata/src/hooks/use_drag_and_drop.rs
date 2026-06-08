use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result, anyhow};
use serde::Deserialize;
use serde_json::Value;

#[derive(Default)]
pub struct DragAndDropState {
    input: String,
    probe_state: ProbeState,
}

impl DragAndDropState {
    pub fn input(&self) -> &str {
        &self.input
    }

    pub fn probe_state(&self) -> &ProbeState {
        &self.probe_state
    }

    pub fn report_lines(&self) -> Vec<Line<'static>> {
        match &self.probe_state {
            ProbeState::Waiting => vec![line_gray(
                "Drop or paste a local video path to inspect container, streams, tags, and chapters.",
            )],
            ProbeState::Missing(path) => vec![
                line_label_value("Path", path.clone()),
                line_gray("The file does not exist on disk."),
            ],
            ProbeState::ProbeError(message) => vec![line_label_value("Error", message.clone())],
            ProbeState::Loaded(report) => report.to_lines(),
        }
    }

    pub fn commit_input(&mut self) {
        if let Some(path) = parse_first_path(&self.input) {
            self.probe_state = ProbeState::from_path(&path);
        }
        self.input.clear();
    }

    pub fn pop_input(&mut self) {
        self.input.pop();
        self.sync_input_preview();
    }

    pub fn push_input(&mut self, ch: char) {
        self.input.push(ch);
        self.sync_input_preview();
    }

    pub fn replace_input(&mut self, data: String) {
        self.input = data;
        self.sync_input_preview();
    }

    fn sync_input_preview(&mut self) {
        self.probe_state = match parse_first_path(&self.input) {
            Some(path) => ProbeState::from_path(&path),
            None => ProbeState::Waiting,
        };
    }
}

#[derive(Default)]
pub enum ProbeState {
    #[default]
    Waiting,
    Missing(String),
    ProbeError(String),
    Loaded(ProbeReport),
}

impl ProbeState {
    fn from_path(path: &Path) -> Self {
        if !path.exists() {
            return Self::Missing(path.display().to_string());
        }

        match ProbeReport::from_path(path) {
            Ok(report) => Self::Loaded(report),
            Err(error) => Self::ProbeError(error.to_string()),
        }
    }
}

#[derive(Clone, Default)]
pub struct ProbeReport {
    path: String,
    file_name: String,
    format_name: String,
    format_long_name: String,
    program_count: String,
    stream_group_count: String,
    duration: String,
    size: String,
    bit_rate: String,
    probe_score: String,
    start_time: String,
    stream_count: usize,
    video_count: usize,
    audio_count: usize,
    subtitle_count: usize,
    chapter_count: usize,
    format_tags: Vec<(String, String)>,
    streams: Vec<StreamSummary>,
    chapters: Vec<ChapterSummary>,
}

impl ProbeReport {
    fn from_path(path: &Path) -> Result<Self> {
        let output = Command::new("ffprobe")
            .args([
                "-v",
                "error",
                "-print_format",
                "json",
                "-show_format",
                "-show_streams",
                "-show_chapters",
            ])
            .arg(path)
            .output()
            .context("failed to launch ffprobe")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
            let message = if stderr.is_empty() {
                format!("ffprobe exited with status {}", output.status)
            } else {
                stderr
            };
            return Err(anyhow!(message));
        }

        let parsed: FfprobeOutput =
            serde_json::from_slice(&output.stdout).context("failed to parse ffprobe json")?;
        Ok(Self::from_ffprobe(path, parsed))
    }

    fn from_ffprobe(path: &Path, parsed: FfprobeOutput) -> Self {
        let streams = parsed
            .streams
            .into_iter()
            .map(StreamSummary::from_stream)
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
            .map(ChapterSummary::from_chapter)
            .collect::<Vec<_>>();

        let format = parsed.format.unwrap_or_default();

        Self {
            path: path.display().to_string(),
            file_name: file_name_label(path),
            format_name: format.format_name.unwrap_or_else(|| "unknown".into()),
            format_long_name: format.format_long_name.unwrap_or_default(),
            program_count: format
                .nb_programs
                .map(|n| n.to_string())
                .unwrap_or_default(),
            stream_group_count: format
                .nb_stream_groups
                .map(|n| n.to_string())
                .unwrap_or_default(),
            duration: display_numeric_option(format.duration.as_deref(), "s"),
            size: display_numeric_option(format.size.as_deref(), "bytes"),
            bit_rate: display_numeric_option(format.bit_rate.as_deref(), "bps"),
            probe_score: json_string_or_default(format.probe_score),
            start_time: format.start_time.unwrap_or_default(),
            stream_count: streams.len(),
            video_count,
            audio_count,
            subtitle_count,
            chapter_count: chapters.len(),
            format_tags: flatten_tags(format.tags),
            streams,
            chapters,
        }
    }

    pub fn display_name(&self) -> String {
        self.file_name.clone()
    }

    fn to_lines(&self) -> Vec<Line<'static>> {
        let mut lines = vec![
            line_label_value("Path", self.path.clone()),
            line_label_value("File", self.file_name.clone()),
            line_label_value(
                "Format",
                format!(
                    "{} ({})",
                    self.format_name,
                    fallback_empty(&self.format_long_name)
                ),
            ),
            line_label_value("Duration", fallback_empty(&self.duration)),
            line_label_value("Size", fallback_empty(&self.size)),
            line_label_value("Bit rate", fallback_empty(&self.bit_rate)),
            line_label_value("Start time", fallback_empty(&self.start_time)),
            line_label_value("Probe score", fallback_empty(&self.probe_score)),
            line_label_value("Programs", fallback_empty(&self.program_count)),
            line_label_value("Stream groups", fallback_empty(&self.stream_group_count)),
            line_label_value(
                "Streams",
                format!(
                    "total={} video={} audio={} subtitle={}",
                    self.stream_count, self.video_count, self.audio_count, self.subtitle_count
                ),
            ),
            line_label_value("Chapters", self.chapter_count.to_string()),
        ];

        if !self.format_tags.is_empty() {
            lines.push(line_section("Format tags"));
            for (key, value) in &self.format_tags {
                lines.push(line_bullet(key, value));
            }
        }

        if !self.streams.is_empty() {
            lines.push(line_section("Streams"));
            for stream in &self.streams {
                lines.extend(stream.to_lines());
            }
        }

        if !self.chapters.is_empty() {
            lines.push(line_section("Chapters"));
            for chapter in &self.chapters {
                lines.extend(chapter.to_lines());
            }
        }

        lines
    }
}

#[derive(Clone, Default)]
struct StreamSummary {
    index: i64,
    stream_id: String,
    codec_type: String,
    codec_name: String,
    codec_long_name: String,
    profile: String,
    codec_tag: String,
    duration: String,
    bit_rate: String,
    width: String,
    height: String,
    coded_width: String,
    coded_height: String,
    display_aspect_ratio: String,
    pixel_format: String,
    sample_format: String,
    level: String,
    field_order: String,
    frame_rate: String,
    sample_aspect_ratio: String,
    color_range: String,
    color_space: String,
    color_transfer: String,
    color_primaries: String,
    chroma_location: String,
    sample_rate: String,
    channels: String,
    channel_layout: String,
    bits_per_sample: String,
    bits_per_raw_sample: String,
    time_base: String,
    start_time: String,
    frame_count: String,
    refs: String,
    has_b_frames: String,
    nal_length_size: String,
    is_avc: String,
    disposition: Vec<(String, String)>,
    tags: Vec<(String, String)>,
}

impl StreamSummary {
    fn from_stream(stream: FfprobeStream) -> Self {
        let disposition = stream
            .disposition
            .map(flatten_object_pairs)
            .unwrap_or_default();

        Self {
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
            pixel_format: stream.pix_fmt.unwrap_or_default(),
            sample_format: stream.sample_fmt.unwrap_or_default(),
            level: stream.level.map(|n| n.to_string()).unwrap_or_default(),
            field_order: stream.field_order.unwrap_or_default(),
            frame_rate: display_frame_rate(stream.avg_frame_rate, stream.r_frame_rate),
            sample_aspect_ratio: stream.sample_aspect_ratio.unwrap_or_default(),
            color_range: stream.color_range.unwrap_or_default(),
            color_space: stream.color_space.unwrap_or_default(),
            color_transfer: stream.color_transfer.unwrap_or_default(),
            color_primaries: stream.color_primaries.unwrap_or_default(),
            chroma_location: stream.chroma_location.unwrap_or_default(),
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
            disposition,
            tags: flatten_tags(stream.tags),
        }
    }

    fn to_lines(&self) -> Vec<Line<'static>> {
        let mut lines = vec![
            line_subsection(format!("Stream #{} [{}]", self.index, self.codec_type)),
            line_bullet(
                "codec",
                join_non_empty([self.codec_name.clone(), self.codec_long_name.clone()]),
            ),
        ];

        push_if_present(&mut lines, "id", &self.stream_id);
        push_if_present(&mut lines, "profile", &self.profile);
        push_if_present(&mut lines, "codec tag", &self.codec_tag);
        push_if_present(&mut lines, "duration", &self.duration);
        push_if_present(&mut lines, "bit rate", &self.bit_rate);
        push_if_present(&mut lines, "time base", &self.time_base);
        push_if_present(&mut lines, "start time", &self.start_time);
        push_if_present(&mut lines, "frames", &self.frame_count);

        if self.codec_type == "video" {
            push_if_present(
                &mut lines,
                "resolution",
                &join_non_empty([self.width.clone(), self.height.clone()]).replace(' ', " x "),
            );
            push_if_present(
                &mut lines,
                "coded resolution",
                &join_non_empty([self.coded_width.clone(), self.coded_height.clone()])
                    .replace(' ', " x "),
            );
            push_if_present(&mut lines, "frame rate", &self.frame_rate);
            push_if_present(&mut lines, "pixel format", &self.pixel_format);
            push_if_present(&mut lines, "display aspect", &self.display_aspect_ratio);
            push_if_present(&mut lines, "sample aspect", &self.sample_aspect_ratio);
            push_if_present(&mut lines, "field order", &self.field_order);
            push_if_present(&mut lines, "level", &self.level);
            push_if_present(&mut lines, "chroma location", &self.chroma_location);
            push_if_present(&mut lines, "color range", &self.color_range);
            push_if_present(&mut lines, "color space", &self.color_space);
            push_if_present(&mut lines, "color transfer", &self.color_transfer);
            push_if_present(&mut lines, "color primaries", &self.color_primaries);
            push_if_present(&mut lines, "refs", &self.refs);
            push_if_present(&mut lines, "b-frames", &self.has_b_frames);
            push_if_present(&mut lines, "is avc", &self.is_avc);
            push_if_present(&mut lines, "nal length size", &self.nal_length_size);
            push_if_present(&mut lines, "bits/raw sample", &self.bits_per_raw_sample);
        }

        if self.codec_type == "audio" {
            push_if_present(&mut lines, "sample format", &self.sample_format);
            push_if_present(&mut lines, "sample rate", &self.sample_rate);
            push_if_present(&mut lines, "channels", &self.channels);
            push_if_present(&mut lines, "channel layout", &self.channel_layout);
            push_if_present(&mut lines, "bits/sample", &self.bits_per_sample);
        }

        if !self.disposition.is_empty() {
            lines.push(line_bullet(
                "disposition",
                summarize_pairs(&self.disposition),
            ));
        }

        if !self.tags.is_empty() {
            lines.push(line_bullet("tags", summarize_pairs(&self.tags)));
        }

        lines
    }
}

#[derive(Clone, Default)]
struct ChapterSummary {
    id: i64,
    start: String,
    end: String,
    time_base: String,
    tags: Vec<(String, String)>,
}

impl ChapterSummary {
    fn from_chapter(chapter: FfprobeChapter) -> Self {
        Self {
            id: chapter.id.unwrap_or_default(),
            start: display_numeric_option(chapter.start_time.as_deref(), "s"),
            end: display_numeric_option(chapter.end_time.as_deref(), "s"),
            time_base: chapter.time_base.unwrap_or_default(),
            tags: flatten_tags(chapter.tags),
        }
    }

    fn to_lines(&self) -> Vec<Line<'static>> {
        let mut lines = vec![line_subsection(format!(
            "Chapter #{} {} -> {}",
            self.id,
            fallback_empty(&self.start),
            fallback_empty(&self.end)
        ))];

        push_if_present(&mut lines, "time base", &self.time_base);
        if !self.tags.is_empty() {
            lines.push(line_bullet("tags", summarize_pairs(&self.tags)));
        }
        lines
    }
}

#[derive(Debug, Deserialize, Default)]
struct FfprobeOutput {
    #[serde(default)]
    streams: Vec<FfprobeStream>,
    #[serde(default)]
    chapters: Vec<FfprobeChapter>,
    format: Option<FfprobeFormat>,
}

#[derive(Debug, Deserialize, Default)]
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
struct FfprobeChapter {
    id: Option<i64>,
    time_base: Option<String>,
    start_time: Option<String>,
    end_time: Option<String>,
    tags: Option<Value>,
}

fn parse_first_path(input: &str) -> Option<PathBuf> {
    input
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .map(normalize_dropped_path)
        .filter(|path| !path.as_os_str().is_empty())
}

fn normalize_dropped_path(raw: &str) -> PathBuf {
    let trimmed = raw.trim().trim_matches(|c| c == '"' || c == '\'');
    let mut normalized = String::with_capacity(trimmed.len());
    let mut chars = trimmed.chars();

    while let Some(ch) = chars.next() {
        if ch == '\\' {
            if let Some(next) = chars.next() {
                normalized.push(next);
            }
        } else {
            normalized.push(ch);
        }
    }

    PathBuf::from(normalized)
}

fn file_name_label(path: &Path) -> String {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| path.display().to_string())
}

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

fn line_label_value(label: impl Into<String>, value: impl Into<String>) -> Line<'static> {
    Line::from(vec![
        Span::styled(
            format!("{}: ", label.into()),
            Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        ),
        Span::raw(value.into()),
    ])
}

fn line_gray(value: impl Into<String>) -> Line<'static> {
    Line::from(Span::styled(value.into(), Style::new().fg(Color::Gray)))
}

fn line_section(title: impl Into<String>) -> Line<'static> {
    Line::from(Span::styled(
        format!("-- {} --", title.into()),
        Style::new().fg(Color::Magenta).add_modifier(Modifier::BOLD),
    ))
}

fn line_subsection(title: impl Into<String>) -> Line<'static> {
    Line::from(Span::styled(
        title.into(),
        Style::new().fg(Color::Green).add_modifier(Modifier::BOLD),
    ))
}

fn line_bullet(label: impl Into<String>, value: impl Into<String>) -> Line<'static> {
    Line::from(vec![
        Span::raw("  - "),
        Span::styled(
            format!("{}: ", label.into()),
            Style::new().add_modifier(Modifier::BOLD),
        ),
        Span::raw(value.into()),
    ])
}

fn push_if_present(lines: &mut Vec<Line<'static>>, label: &str, value: &str) {
    if !value.trim().is_empty() {
        lines.push(line_bullet(label.to_string(), value.to_string()));
    }
}

fn flatten_tags(value: Option<Value>) -> Vec<(String, String)> {
    value.map(flatten_object_pairs).unwrap_or_default()
}

fn flatten_object_pairs(value: Value) -> Vec<(String, String)> {
    match value {
        Value::Object(map) => map
            .into_iter()
            .map(|(key, value)| (key, json_value_to_string(value)))
            .collect(),
        _ => Vec::new(),
    }
}

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

fn json_string_or_default(value: Option<Value>) -> String {
    value.map(json_value_to_string).unwrap_or_default()
}

fn summarize_pairs(pairs: &[(String, String)]) -> String {
    pairs
        .iter()
        .filter(|(_, value)| !value.trim().is_empty())
        .map(|(key, value)| format!("{key}={value}"))
        .collect::<Vec<_>>()
        .join(", ")
}

fn fallback_empty(value: &str) -> String {
    if value.trim().is_empty() {
        "n/a".into()
    } else {
        value.to_string()
    }
}

fn display_numeric_option(raw: Option<&str>, suffix: &str) -> String {
    match raw.and_then(|value| value.parse::<f64>().ok()) {
        Some(number) if suffix == "s" => format!("{number:.3} s"),
        Some(number) if suffix == "bytes" => format!("{number:.0} bytes"),
        Some(number) if suffix == "bps" => format!("{number:.0} bps"),
        Some(number) => format!("{number} {suffix}"),
        None => raw.unwrap_or_default().to_string(),
    }
}

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

fn join_non_empty(values: [String; 2]) -> String {
    values
        .into_iter()
        .filter(|value| !value.trim().is_empty())
        .collect::<Vec<_>>()
        .join(" | ")
}
