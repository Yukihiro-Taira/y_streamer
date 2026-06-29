use crate::domain::diagnostic::data::diagnostic_report::{
    DiagnosticCheck, DiagnosticReport, DiagnosticStatus,
};
use crate::domain::diagnostic::data::platform_profile::PlatformProfile;
use crate::domain::media_read::data::media_probe_report::{
    MediaKeyValue, MediaProbeReport, MediaStreamInfo,
};

pub fn run(report: &MediaProbeReport, profile: &PlatformProfile) -> DiagnosticReport {
    let video: Vec<&MediaStreamInfo> = report
        .streams
        .iter()
        .filter(|s| s.codec_type == "video" && !has_disposition(s, "attached_pic"))
        .collect();
    let audio: Vec<&MediaStreamInfo> = report
        .streams
        .iter()
        .filter(|s| s.codec_type == "audio")
        .collect();
    let subtitles: Vec<&MediaStreamInfo> = report
        .streams
        .iter()
        .filter(|s| s.codec_type == "subtitle")
        .collect();

    let mut checks = vec![check_container(&report.format_name, profile)];

    if let Some(vs) = video.first() {
        checks.push(check_video_codec(&vs.codec_name, profile));
        checks.push(check_pixel_format(&vs.pixel_format));
        checks.push(check_color_depth(&vs.bits_per_raw_sample));
        checks.push(check_field_order(&vs.field_order));
        checks.push(check_frame_rate(&vs.frame_rate));
        checks.push(check_vfr(&vs.avg_frame_rate, &vs.r_frame_rate));
        checks.push(check_rotation(&vs.tags));
        checks.push(check_sample_aspect_ratio(&vs.sample_aspect_ratio));
        checks.push(check_color_space(
            &vs.color_primaries,
            &vs.color_transfer,
            &vs.color_space,
            &vs.color_range,
        ));
        checks.push(check_b_frames(&vs.has_b_frames));
        checks.push(check_chroma_location(&vs.chroma_location));
        checks.push(check_closed_captions(&vs.closed_captions));
        if !vs.side_data.is_empty() {
            checks.extend(check_hdr_side_data(&vs.side_data));
        }
        if vs.codec_name == "h264" {
            checks.push(check_h264_profile(&vs.profile, profile));
        }
    }

    checks.push(check_audio_present(report.audio_count));

    if let Some(a) = audio.first() {
        checks.push(check_audio_codec(&a.codec_name, profile));
        checks.push(check_audio_channels(&a.channel_layout, profile));
        checks.push(check_sample_rate(&a.sample_rate, profile));
        checks.push(check_audio_bit_depth(&a.bits_per_sample, profile));
        checks.push(check_default_audio_stream(a));
    }
    checks.push(check_audio_language_tags(&audio));

    checks.push(check_subtitles(report.subtitle_count));
    checks.push(check_subtitle_codecs(&subtitles));
    checks.push(check_subtitle_language_tags(&subtitles));
    checks.push(check_forced_subtitles(&subtitles));
    checks.push(check_extension(&report.file_name, &report.format_name));
    checks.push(check_encoder_tag(&report.format_tags));
    checks.push(check_creation_time_tag(&report.format_tags));
    checks.push(check_timecode_tag(report, &video, profile));

    if let (Some(vs), Some(a)) = (video.first(), audio.first()) {
        checks.push(check_av_sync(&vs.start_time, &a.start_time));
        checks.push(check_av_duration_mismatch(&vs.duration, &a.duration));
    }

    DiagnosticReport { checks }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn has_disposition(s: &MediaStreamInfo, key: &str) -> bool {
    s.disposition
        .iter()
        .any(|kv| kv.key == key && kv.value == "1")
}

fn tag_value(tags: &[MediaKeyValue], key: &str) -> Option<String> {
    tags.iter()
        .find(|kv| kv.key.to_lowercase() == key.to_lowercase())
        .map(|kv| kv.value.clone())
}

fn pass(label: &str, detail: impl Into<String>) -> DiagnosticCheck {
    DiagnosticCheck {
        label: label.into(),
        status: DiagnosticStatus::Pass,
        detail: detail.into(),
    }
}

fn warn(label: &str, detail: impl Into<String>) -> DiagnosticCheck {
    DiagnosticCheck {
        label: label.into(),
        status: DiagnosticStatus::Warn,
        detail: detail.into(),
    }
}

fn fail(label: &str, detail: impl Into<String>) -> DiagnosticCheck {
    DiagnosticCheck {
        label: label.into(),
        status: DiagnosticStatus::Fail,
        detail: detail.into(),
    }
}

fn parse_fps(s: &str) -> Option<f64> {
    let s = s.trim();
    // handle "25.000 fps" display format
    let s = s.trim_end_matches(" fps").trim_end_matches("fps");
    let parts: Vec<&str> = s.split('/').collect();
    if parts.len() == 2 {
        let num: f64 = parts[0].trim().parse().ok()?;
        let den: f64 = parts[1].trim().parse().ok()?;
        if den == 0.0 {
            return None;
        }
        Some(num / den)
    } else {
        s.trim().parse().ok()
    }
}

fn parse_duration_secs(s: &str) -> Option<f64> {
    // strip suffix like " s"
    s.trim().trim_end_matches(" s").trim().parse().ok()
}

// ── Rules ─────────────────────────────────────────────────────────────────────

fn check_container(format_name: &str, profile: &PlatformProfile) -> DiagnosticCheck {
    let label = "Container";
    let first = format_name.split(',').next().unwrap_or(format_name).trim();
    match (first, profile) {
        ("mp4", _) => pass(label, "mp4 — wide compatibility"),
        ("mov", PlatformProfile::Broadcast) => pass(label, "mov — broadcast standard"),
        ("mov", _) => pass(label, "mov — good compatibility"),
        ("mxf", PlatformProfile::Broadcast) => pass(label, "MXF — broadcast standard"),
        ("mxf", _) => warn(label, "MXF — broadcast only, not web/mobile"),
        ("matroska" | "mkv", PlatformProfile::Web) => {
            warn(label, "MKV — limited browser/device support")
        }
        ("matroska" | "mkv", _) => warn(label, "MKV — not recommended for delivery"),
        _ => warn(label, format!("{format_name} — check platform support")),
    }
}

fn check_video_codec(codec_name: &str, profile: &PlatformProfile) -> DiagnosticCheck {
    let label = "Video codec";
    match (codec_name, profile) {
        ("h264", _) => pass(label, "H.264 — universal compatibility"),
        ("hevc" | "h265", PlatformProfile::Broadcast) => {
            pass(label, "HEVC — acceptable for broadcast")
        }
        ("hevc" | "h265", _) => warn(label, "HEVC — limited on older Android and browsers"),
        ("av1", PlatformProfile::Web) => {
            warn(label, "AV1 — growing web support, not universal yet")
        }
        ("av1", _) => warn(label, "AV1 — limited hardware decode support"),
        ("vp9", PlatformProfile::Web) => warn(label, "VP9 — web-only"),
        ("vp9", _) => fail(label, "VP9 — not supported on mobile/broadcast"),
        ("prores" | "dnxhd" | "dnxhr" | "cineform", PlatformProfile::Broadcast) => warn(
            label,
            format!("{codec_name} — editing codec, acceptable for archival but not delivery"),
        ),
        ("prores" | "dnxhd" | "dnxhr" | "cineform", _) => fail(
            label,
            format!("{codec_name} — editing codec, not for delivery"),
        ),
        ("", _) => fail(label, "No video codec detected"),
        _ => warn(label, format!("{codec_name} — verify platform support")),
    }
}

fn check_pixel_format(pix_fmt: &str) -> DiagnosticCheck {
    let label = "Pixel format";
    match pix_fmt {
        "yuv420p" => pass(label, "yuv420p — maximum compatibility"),
        "yuv422p" => warn(label, "yuv422p — editing format, may fail on mobile"),
        "yuv444p" => warn(label, "yuv444p — high quality, limited support"),
        "yuv420p10le" | "yuv420p10be" => warn(label, "10-bit 4:2:0 — HDR, limited web support"),
        "" => warn(label, "Pixel format unknown"),
        _ => warn(label, format!("{pix_fmt} — check device support")),
    }
}

fn check_color_depth(bits_per_raw_sample: &str) -> DiagnosticCheck {
    let label = "Video color depth";
    match bits_per_raw_sample.trim() {
        "8" => pass(label, "8-bit — maximum compatibility"),
        "0" | "" => pass(label, "8-bit (compressed — normal for H.264)"),
        "10" => warn(label, "10-bit HDR — may fail on older web players"),
        "12" => warn(label, "12-bit — cinema/HDR, not web delivery"),
        b => warn(label, format!("{b}-bit — verify platform support")),
    }
}

fn check_field_order(field_order: &str) -> DiagnosticCheck {
    let label = "Scan type";
    match field_order {
        "progressive" | "" => pass(label, "Progressive — no deinterlacing needed"),
        order => warn(
            label,
            format!("Interlaced ({order}) — deinterlace before web delivery"),
        ),
    }
}

fn check_frame_rate(frame_rate: &str) -> DiagnosticCheck {
    let label = "Frame rate";
    let Some(fps) = parse_fps(frame_rate) else {
        return warn(label, format!("Cannot parse: {frame_rate}"));
    };
    if (fps - 25.0).abs() < 0.1 {
        pass(label, "25 fps — broadcast standard")
    } else if (fps - 30.0).abs() < 0.1 || (fps - 29.97).abs() < 0.02 {
        pass(label, format!("{fps:.2} fps — standard"))
    } else if (fps - 24.0).abs() < 0.1 || (fps - 23.976).abs() < 0.01 {
        pass(label, format!("{fps:.3} fps — cinema standard"))
    } else {
        warn(
            label,
            format!("{fps:.3} fps — non-standard, verify with platform"),
        )
    }
}

fn check_vfr(avg_frame_rate: &str, r_frame_rate: &str) -> DiagnosticCheck {
    let label = "Variable frame rate";
    if avg_frame_rate.is_empty() || r_frame_rate.is_empty() {
        return warn(label, "Cannot determine — frame rate data missing");
    }
    match (parse_fps(avg_frame_rate), parse_fps(r_frame_rate)) {
        (Some(avg), Some(rfr)) if (avg - rfr).abs() > 0.01 => warn(
            label,
            format!(
                "VFR detected — avg {avg:.3} vs container {rfr:.3} fps — may cause sync issues"
            ),
        ),
        (Some(avg), Some(_)) => pass(label, format!("CFR — {avg:.3} fps constant")),
        _ => warn(label, "Cannot determine frame rate consistency"),
    }
}

fn check_rotation(tags: &[MediaKeyValue]) -> DiagnosticCheck {
    let label = "Rotation";
    let rotate = tag_value(tags, "rotate").unwrap_or_default();
    match rotate.trim() {
        "" | "0" => pass(label, "No rotation — correct orientation"),
        r => warn(label, format!("rotate={r} — player must compensate")),
    }
}

fn check_h264_profile(profile: &str, platform: &PlatformProfile) -> DiagnosticCheck {
    let label = "H.264 profile";
    match (profile, platform) {
        ("Baseline" | "Constrained Baseline", _) => {
            pass(label, "Baseline — maximum mobile compatibility")
        }
        ("Main", _) => pass(label, "Main — good compatibility"),
        ("High", PlatformProfile::Mobile) => {
            warn(label, "High profile — may fail on older mobile devices")
        }
        ("High", _) => pass(label, "High profile — fine for web/broadcast"),
        ("", _) => warn(label, "Profile unknown"),
        _ => warn(label, format!("{profile} — verify device support")),
    }
}

fn check_audio_present(audio_count: usize) -> DiagnosticCheck {
    let label = "Audio track";
    if audio_count > 0 {
        pass(label, format!("{audio_count} track(s) found"))
    } else {
        fail(label, "No audio track — video is silent")
    }
}

fn check_audio_codec(codec_name: &str, profile: &PlatformProfile) -> DiagnosticCheck {
    let label = "Audio codec";
    match (codec_name, profile) {
        ("aac", _) => pass(label, "AAC — universal support"),
        ("mp3", PlatformProfile::Broadcast) => {
            fail(label, "MP3 — not acceptable for broadcast, use PCM")
        }
        ("mp3", _) => pass(label, "MP3 — widely supported"),
        ("pcm_s16le" | "pcm_s24le" | "pcm_s32le", _) => {
            pass(label, format!("{codec_name} — broadcast PCM"))
        }
        ("opus", PlatformProfile::Web) => pass(label, "Opus — web supported"),
        ("opus", _) => warn(label, "Opus — web-only, not for mobile/broadcast"),
        ("vorbis", _) => warn(label, "Vorbis — web-only, not for broadcast"),
        ("ac3" | "eac3", PlatformProfile::Broadcast) => {
            pass(label, format!("{codec_name} — broadcast standard"))
        }
        ("ac3" | "eac3", _) => warn(
            label,
            format!("{codec_name} — broadcast format, limited browser support"),
        ),
        ("", _) => warn(label, "Audio codec unknown"),
        _ => warn(label, format!("{codec_name} — verify platform support")),
    }
}

fn check_audio_channels(channel_layout: &str, profile: &PlatformProfile) -> DiagnosticCheck {
    let label = "Audio channels";
    match (channel_layout, profile) {
        ("mono", _) => pass(label, "Mono — broadcast standard"),
        ("stereo", PlatformProfile::Broadcast) => fail(
            label,
            "Stereo — broadcast requires dual mono tracks, not stereo",
        ),
        ("stereo", _) => pass(label, "Stereo — fine for web/mobile"),
        ("", _) => warn(label, "Channel layout unknown"),
        (l, PlatformProfile::Broadcast) => {
            fail(label, format!("{l} — broadcast requires mono tracks"))
        }
        (l, _) => warn(label, format!("{l} — verify with delivery spec")),
    }
}

fn check_sample_rate(sample_rate: &str, profile: &PlatformProfile) -> DiagnosticCheck {
    let label = "Sample rate";
    match (sample_rate.trim(), profile) {
        ("48000", _) => pass(label, "48 kHz — broadcast standard"),
        ("44100", PlatformProfile::Broadcast) => {
            fail(label, "44.1 kHz — broadcast requires 48 kHz")
        }
        ("44100", _) => warn(label, "44.1 kHz — resample to 48 kHz for broadcast"),
        ("", _) => warn(label, "Sample rate unknown"),
        (r, _) => warn(label, format!("{r} Hz — non-standard")),
    }
}

fn check_audio_bit_depth(bits_per_sample: &str, profile: &PlatformProfile) -> DiagnosticCheck {
    let label = "Audio bit depth";
    match (bits_per_sample.trim(), profile) {
        ("16", _) => pass(label, "16-bit PCM — broadcast standard"),
        ("24", _) => pass(label, "24-bit PCM — acceptable"),
        ("0" | "", _) => pass(
            label,
            "Compressed audio (AAC/MP3) — bit depth not applicable",
        ),
        (b, PlatformProfile::Broadcast) => warn(
            label,
            format!("{b}-bit — broadcast typically requires 16-bit PCM"),
        ),
        (b, _) => warn(label, format!("{b}-bit — verify with delivery spec")),
    }
}

fn check_default_audio_stream(stream: &MediaStreamInfo) -> DiagnosticCheck {
    let label = "Default audio stream";
    let is_default = stream
        .disposition
        .iter()
        .any(|kv| kv.key == "default" && kv.value == "1");
    if is_default {
        pass(label, "Default flag set — players will select this track")
    } else {
        warn(
            label,
            "No default flag — players may not auto-select this track",
        )
    }
}

fn check_audio_language_tags(audio: &[&MediaStreamInfo]) -> DiagnosticCheck {
    let label = "Audio language tags";
    if audio.is_empty() {
        return pass(label, "No audio stream to validate");
    }

    let missing = audio
        .iter()
        .filter(|stream| tag_value(&stream.tags, "language").unwrap_or_default().trim().is_empty())
        .count();

    if missing == 0 {
        pass(label, "All audio streams have language tags")
    } else {
        warn(
            label,
            format!("{missing} audio stream(s) missing language metadata"),
        )
    }
}

fn check_subtitles(subtitle_count: usize) -> DiagnosticCheck {
    let label = "Subtitles";
    if subtitle_count > 0 {
        pass(label, format!("{subtitle_count} stream(s) embedded"))
    } else {
        warn(
            label,
            "No embedded subtitles — check if sidecar .srt needed",
        )
    }
}

fn check_subtitle_language_tags(subtitles: &[&MediaStreamInfo]) -> DiagnosticCheck {
    let label = "Subtitle language tags";
    if subtitles.is_empty() {
        return pass(label, "No subtitle stream to validate");
    }

    let missing = subtitles
        .iter()
        .filter(|stream| tag_value(&stream.tags, "language").unwrap_or_default().trim().is_empty())
        .count();

    if missing == 0 {
        pass(label, "All subtitle streams have language tags")
    } else {
        warn(
            label,
            format!("{missing} subtitle stream(s) missing language metadata"),
        )
    }
}

fn check_subtitle_codecs(subtitles: &[&MediaStreamInfo]) -> DiagnosticCheck {
    let label = "Subtitle codec";
    if subtitles.is_empty() {
        return pass(label, "No subtitle stream to validate");
    }

    let codecs = subtitles
        .iter()
        .map(|stream| stream.codec_name.trim())
        .collect::<Vec<_>>();

    let unsupported = codecs
        .iter()
        .copied()
        .filter(|codec| {
            !matches!(
                *codec,
                "mov_text" | "srt" | "subrip" | "webvtt" | "ass" | "ssa"
            )
        })
        .collect::<Vec<_>>();

    if unsupported.is_empty() {
        pass(
            label,
            format!("Supported subtitle codec(s): {}", codecs.join(", ")),
        )
    } else {
        warn(
            label,
            format!(
                "Unsupported or platform-sensitive subtitle codec(s): {}",
                unsupported.join(", ")
            ),
        )
    }
}

fn check_extension(file_name: &str, format_name: &str) -> DiagnosticCheck {
    let label = "File extension";
    let ext = file_name.rsplit('.').next().unwrap_or("").to_lowercase();
    let matches = match ext.as_str() {
        "mp4" | "m4v" => format_name.contains("mp4") || format_name.contains("mov"),
        "mov" => format_name.contains("mov"),
        "mkv" => format_name.contains("matroska"),
        "mxf" => format_name.contains("mxf"),
        "webm" => format_name.contains("webm") || format_name.contains("matroska"),
        _ => true,
    };
    if matches {
        pass(label, format!(".{ext} matches container"))
    } else {
        warn(
            label,
            format!(".{ext} does not match container — actual format is {format_name}"),
        )
    }
}

fn check_av_sync(video_start: &str, audio_start: &str) -> DiagnosticCheck {
    let label = "A/V start sync";
    let v: f64 = video_start
        .trim()
        .trim_end_matches(" s")
        .parse()
        .unwrap_or(0.0);
    let a: f64 = audio_start
        .trim()
        .trim_end_matches(" s")
        .parse()
        .unwrap_or(0.0);
    let diff_ms = ((v - a).abs() * 1000.0) as u64;
    if diff_ms < 100 {
        pass(label, format!("Drift {diff_ms} ms — within tolerance"))
    } else {
        warn(label, format!("Drift {diff_ms} ms — may cause sync issues"))
    }
}

fn check_av_duration_mismatch(video_duration: &str, audio_duration: &str) -> DiagnosticCheck {
    let label = "A/V duration";
    match (
        parse_duration_secs(video_duration),
        parse_duration_secs(audio_duration),
    ) {
        (Some(v), Some(a)) => {
            let diff_ms = ((v - a).abs() * 1000.0) as u64;
            if diff_ms < 500 {
                pass(label, format!("Match within {diff_ms} ms"))
            } else {
                warn(
                    label,
                    format!("Mismatch {diff_ms} ms — audio/video length differ"),
                )
            }
        }
        _ => warn(
            label,
            "Cannot compare — duration missing from one or both streams",
        ),
    }
}

fn check_sample_aspect_ratio(sar: &str) -> DiagnosticCheck {
    let label = "Sample aspect ratio";
    match sar.trim() {
        "" | "0:1" | "1:1" => pass(label, "Square pixels (1:1)"),
        r => warn(
            label,
            format!("Anamorphic SAR {r} — verify display scaling"),
        ),
    }
}

fn check_color_space(primaries: &str, transfer: &str, space: &str, range: &str) -> DiagnosticCheck {
    let label = "Color space";
    if transfer == "smpte2084" {
        return warn(label, "PQ / HDR10 — verify player supports HDR");
    }
    if transfer == "arib-std-b67" {
        return warn(label, "HLG HDR — broadcast HDR format");
    }
    if primaries == "bt2020" || space == "bt2020nc" || space == "bt2020c" {
        return warn(
            label,
            "BT.2020 wide gamut — HDR content, limited display support",
        );
    }
    if range == "pc" {
        return warn(label, "Full range (pc) — may crush blacks on TV displays");
    }
    if primaries.is_empty() && transfer.is_empty() {
        return warn(label, "Color space untagged — player assumes BT.709");
    }
    pass(
        label,
        format!("BT.709 — primaries={primaries} transfer={transfer}"),
    )
}

fn check_b_frames(has_b_frames: &str) -> DiagnosticCheck {
    let label = "B-frames";
    match has_b_frames.trim() {
        "" | "0" => pass(label, "No B-frames — maximum decoder compatibility"),
        "1" => pass(label, "B-frames: 1 — standard, widely supported"),
        "2" => warn(label, "B-frames: 2 — verify low-power device support"),
        n => warn(
            label,
            format!("B-frames: {n} — may cause issues on low-power decoders"),
        ),
    }
}

fn check_chroma_location(chroma_location: &str) -> DiagnosticCheck {
    let label = "Chroma location";
    match chroma_location.trim() {
        "" | "left" => pass(label, "Chroma left — H.264/H.265 standard"),
        "center" => warn(
            label,
            "Chroma center — unusual for H.264, verify encoder settings",
        ),
        "topleft" => pass(label, "Chroma topleft — interlaced convention, acceptable"),
        loc => warn(label, format!("{loc} — non-standard chroma location")),
    }
}

fn check_closed_captions(closed_captions: &str) -> DiagnosticCheck {
    let label = "Closed captions";
    match closed_captions.trim() {
        "" | "0" => pass(label, "No embedded CEA-608/708 captions flagged"),
        "1" => warn(
            label,
            "Embedded closed captions detected — separate from subtitle streams",
        ),
        value => warn(
            label,
            format!("{value} caption flag(s) detected — verify CEA-608/708 handling"),
        ),
    }
}

fn check_hdr_side_data(side_data: &[MediaKeyValue]) -> Vec<DiagnosticCheck> {
    let mut checks = vec![];
    for entry in side_data {
        let check = match entry.key.as_str() {
            "Mastering display metadata" => {
                let max_lum = entry
                    .value
                    .split(", ")
                    .find(|p| p.starts_with("max_luminance="))
                    .and_then(|p| p.strip_prefix("max_luminance="))
                    .unwrap_or("?");
                warn(
                    "HDR mastering display",
                    format!("HDR10 master display present — max luminance: {max_lum}"),
                )
            }
            "Content light level metadata" => {
                let max_cll = entry
                    .value
                    .split(", ")
                    .find(|p| p.starts_with("max_content="))
                    .and_then(|p| p.strip_prefix("max_content="))
                    .unwrap_or("?");
                let max_fall = entry
                    .value
                    .split(", ")
                    .find(|p| p.starts_with("max_average="))
                    .and_then(|p| p.strip_prefix("max_average="))
                    .unwrap_or("?");
                warn(
                    "HDR content light level",
                    format!("MaxCLL={max_cll}nit MaxFALL={max_fall}nit"),
                )
            }
            "DOVI configuration record" => {
                let profile = entry
                    .value
                    .split(", ")
                    .find(|p| p.starts_with("dv_profile="))
                    .and_then(|p| p.strip_prefix("dv_profile="))
                    .unwrap_or("?");
                warn(
                    "Dolby Vision",
                    format!("Dolby Vision profile {profile} — verify player support"),
                )
            }
            "Spherical Mapping" => warn(
                "360° video",
                "Spherical mapping detected — verify 360° player support",
            ),
            "Stereo 3D" => warn("Stereo 3D", "3D video detected — verify player support"),
            _ => continue,
        };
        checks.push(check);
    }
    checks
}

fn check_forced_subtitles(subtitles: &[&MediaStreamInfo]) -> DiagnosticCheck {
    let label = "Forced subtitles";
    let forced_count = subtitles
        .iter()
        .filter(|stream| has_disposition(stream, "forced"))
        .count();
    match forced_count {
        0 => pass(label, "No forced subtitle stream flagged"),
        1 => warn(
            label,
            "1 forced subtitle stream present — verify target player behavior",
        ),
        count => warn(
            label,
            format!("{count} forced subtitle streams present — verify delivery intent"),
        ),
    }
}

fn check_encoder_tag(format_tags: &[MediaKeyValue]) -> DiagnosticCheck {
    let label = "Encoder";
    match tag_value(format_tags, "encoder") {
        Some(enc) if !enc.is_empty() => pass(label, format!("Encoded with: {enc}")),
        _ => match tag_value(format_tags, "encoding_tool") {
            Some(tool) if !tool.is_empty() => pass(label, format!("Encoded with: {tool}")),
            _ => warn(label, "No encoder tag — origin NLE unknown"),
        },
    }
}

fn check_creation_time_tag(format_tags: &[MediaKeyValue]) -> DiagnosticCheck {
    let label = "Creation time";
    match tag_value(format_tags, "creation_time") {
        Some(value) if !value.is_empty() => pass(label, format!("Tagged: {value}")),
        _ => warn(label, "No creation_time tag found"),
    }
}

fn check_timecode_tag(
    report: &MediaProbeReport,
    video: &[&MediaStreamInfo],
    profile: &PlatformProfile,
) -> DiagnosticCheck {
    let label = "Timecode";
    let format_timecode = tag_value(&report.format_tags, "timecode");
    let stream_timecode = video.iter().find_map(|stream| tag_value(&stream.tags, "timecode"));
    let value = format_timecode.or(stream_timecode).unwrap_or_default();

    if !value.is_empty() {
        return pass(label, format!("Timecode present: {value}"));
    }

    match profile {
        PlatformProfile::Broadcast => warn(label, "No timecode tag found for broadcast profile"),
        _ => pass(label, "No explicit timecode tag — acceptable for web/mobile"),
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::media_read::data::media_probe_report::{
        MediaKeyValue, MediaProbeReport, MediaStreamInfo,
    };

    fn make_report() -> MediaProbeReport {
        MediaProbeReport {
            file_name: "test.mp4".into(),
            format_name: "mov,mp4,m4a,3gp,3g2,mj2".into(),
            audio_count: 1,
            video_count: 1,
            subtitle_count: 0,
            streams: vec![
                MediaStreamInfo {
                    codec_type: "video".into(),
                    codec_name: "h264".into(),
                    pixel_format: "yuv420p".into(),
                    bits_per_raw_sample: "8".into(),
                    field_order: "progressive".into(),
                    frame_rate: "25.000 fps".into(),
                    avg_frame_rate: "25/1".into(),
                    r_frame_rate: "25/1".into(),
                    profile: "Main".into(),
                    disposition: vec![MediaKeyValue {
                        key: "default".into(),
                        value: "1".into(),
                    }],
                    ..Default::default()
                },
                MediaStreamInfo {
                    codec_type: "audio".into(),
                    codec_name: "aac".into(),
                    channel_layout: "stereo".into(),
                    sample_rate: "48000".into(),
                    bits_per_sample: "0".into(),
                    disposition: vec![MediaKeyValue {
                        key: "default".into(),
                        value: "1".into(),
                    }],
                    ..Default::default()
                },
            ],
            ..Default::default()
        }
    }

    #[test]
    fn web_stereo_is_pass() {
        let diag = run(&make_report(), &PlatformProfile::Web);
        let ch = diag
            .checks
            .iter()
            .find(|c| c.label == "Audio channels")
            .unwrap();
        assert!(matches!(ch.status, DiagnosticStatus::Pass));
    }

    #[test]
    fn broadcast_stereo_is_fail() {
        let diag = run(&make_report(), &PlatformProfile::Broadcast);
        let ch = diag
            .checks
            .iter()
            .find(|c| c.label == "Audio channels")
            .unwrap();
        assert!(matches!(ch.status, DiagnosticStatus::Fail));
    }

    #[test]
    fn no_audio_is_fail_all_profiles() {
        let mut report = make_report();
        report.audio_count = 0;
        report.streams.retain(|s| s.codec_type != "audio");
        for profile in [
            PlatformProfile::Web,
            PlatformProfile::Broadcast,
            PlatformProfile::Mobile,
        ] {
            let diag = run(&report, &profile);
            assert_eq!(
                diag.fail_count(),
                1,
                "expected 1 fail for {}",
                profile.label()
            );
        }
    }

    #[test]
    fn cfr_is_pass() {
        let diag = run(&make_report(), &PlatformProfile::Web);
        let vfr = diag
            .checks
            .iter()
            .find(|c| c.label == "Variable frame rate")
            .unwrap();
        assert!(matches!(vfr.status, DiagnosticStatus::Pass));
    }

    #[test]
    fn vfr_is_warn() {
        let mut report = make_report();
        report.streams[0].avg_frame_rate = "30000/1001".into();
        report.streams[0].r_frame_rate = "60000/1001".into();
        let diag = run(&report, &PlatformProfile::Web);
        let vfr = diag
            .checks
            .iter()
            .find(|c| c.label == "Variable frame rate")
            .unwrap();
        assert!(matches!(vfr.status, DiagnosticStatus::Warn));
    }

    #[test]
    fn broadcast_44100_is_fail() {
        let mut report = make_report();
        report.streams[1].sample_rate = "44100".into();
        let diag = run(&report, &PlatformProfile::Broadcast);
        let sr = diag
            .checks
            .iter()
            .find(|c| c.label == "Sample rate")
            .unwrap();
        assert!(matches!(sr.status, DiagnosticStatus::Fail));
    }

    #[test]
    fn closed_captions_flag_is_warn() {
        let mut report = make_report();
        report.streams[0].closed_captions = "1".into();
        let diag = run(&report, &PlatformProfile::Web);
        let cc = diag
            .checks
            .iter()
            .find(|c| c.label == "Closed captions")
            .unwrap();
        assert!(matches!(cc.status, DiagnosticStatus::Warn));
    }

    #[test]
    fn forced_subtitle_stream_is_warn() {
        let mut report = make_report();
        report.subtitle_count = 1;
        report.streams.push(MediaStreamInfo {
            codec_type: "subtitle".into(),
            codec_name: "mov_text".into(),
            disposition: vec![MediaKeyValue {
                key: "forced".into(),
                value: "1".into(),
            }],
            ..Default::default()
        });
        let diag = run(&report, &PlatformProfile::Web);
        let forced = diag
            .checks
            .iter()
            .find(|c| c.label == "Forced subtitles")
            .unwrap();
        assert!(matches!(forced.status, DiagnosticStatus::Warn));
    }

    #[test]
    fn supported_subtitle_codec_is_pass() {
        let mut report = make_report();
        report.subtitle_count = 1;
        report.streams.push(MediaStreamInfo {
            codec_type: "subtitle".into(),
            codec_name: "mov_text".into(),
            ..Default::default()
        });
        let diag = run(&report, &PlatformProfile::Web);
        let codec = diag
            .checks
            .iter()
            .find(|c| c.label == "Subtitle codec")
            .unwrap();
        assert!(matches!(codec.status, DiagnosticStatus::Pass));
    }

    #[test]
    fn unsupported_subtitle_codec_is_warn() {
        let mut report = make_report();
        report.subtitle_count = 1;
        report.streams.push(MediaStreamInfo {
            codec_type: "subtitle".into(),
            codec_name: "dvd_subtitle".into(),
            ..Default::default()
        });
        let diag = run(&report, &PlatformProfile::Web);
        let codec = diag
            .checks
            .iter()
            .find(|c| c.label == "Subtitle codec")
            .unwrap();
        assert!(matches!(codec.status, DiagnosticStatus::Warn));
    }

    #[test]
    fn missing_audio_language_is_warn() {
        let diag = run(&make_report(), &PlatformProfile::Web);
        let check = diag
            .checks
            .iter()
            .find(|c| c.label == "Audio language tags")
            .unwrap();
        assert!(matches!(check.status, DiagnosticStatus::Warn));
    }

    #[test]
    fn timecode_missing_is_warn_for_broadcast() {
        let diag = run(&make_report(), &PlatformProfile::Broadcast);
        let check = diag.checks.iter().find(|c| c.label == "Timecode").unwrap();
        assert!(matches!(check.status, DiagnosticStatus::Warn));
    }

    #[test]
    fn creation_time_present_is_pass() {
        let mut report = make_report();
        report.format_tags.push(MediaKeyValue {
            key: "creation_time".into(),
            value: "2026-06-29T10:00:00Z".into(),
        });
        let diag = run(&report, &PlatformProfile::Web);
        let check = diag
            .checks
            .iter()
            .find(|c| c.label == "Creation time")
            .unwrap();
        assert!(matches!(check.status, DiagnosticStatus::Pass));
    }
}
