use crate::domain::diagnostic::data::diagnostic_report::{
    DiagnosticCheck, DiagnosticReport, DiagnosticStatus,
};
use crate::domain::media_read::data::media_probe_report::{
    MediaKeyValue, MediaProbeReport, MediaStreamInfo,
};

pub fn run(report: &MediaProbeReport) -> DiagnosticReport {
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

    let mut checks = vec![check_container(&report.format_name)];

    if let Some(vs) = video.first() {
        checks.push(check_video_codec(&vs.codec_name));
        checks.push(check_pixel_format(&vs.pixel_format));
        checks.push(check_color_depth(&vs.bits_per_raw_sample));
        checks.push(check_field_order(&vs.field_order));
        checks.push(check_frame_rate(&vs.frame_rate));
        checks.push(check_rotation(&vs.tags));
        if vs.codec_name == "h264" {
            checks.push(check_h264_profile(&vs.profile));
        }
    }

    checks.push(check_audio_present(report.audio_count));

    if let Some(a) = audio.first() {
        checks.push(check_audio_codec(&a.codec_name));
        checks.push(check_audio_channels(&a.channel_layout));
        checks.push(check_sample_rate(&a.sample_rate));
        checks.push(check_audio_bit_depth(&a.bits_per_sample));
    }

    checks.push(check_subtitles(report.subtitle_count));
    checks.push(check_extension(&report.file_name, &report.format_name));

    if let (Some(vs), Some(a)) = (video.first(), audio.first()) {
        checks.push(check_av_sync(&vs.start_time, &a.start_time));
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

// ── Rules ─────────────────────────────────────────────────────────────────────

fn check_container(format_name: &str) -> DiagnosticCheck {
    let label = "Container";
    let first = format_name.split(',').next().unwrap_or(format_name).trim();
    match first {
        "mp4" | "mov" => pass(label, format!("{first} — wide compatibility")),
        "matroska" | "mkv" => warn(label, "MKV — limited browser/mobile support"),
        "mxf" => pass(label, "MXF — broadcast standard"),
        _ => warn(label, format!("{format_name} — check platform support")),
    }
}

fn check_video_codec(codec_name: &str) -> DiagnosticCheck {
    let label = "Video codec";
    match codec_name {
        "h264" => pass(label, "H.264 — universal compatibility"),
        "hevc" | "h265" => warn(label, "HEVC — limited on older Android and browsers"),
        "av1" => warn(label, "AV1 — limited hardware decode support"),
        "vp9" => warn(label, "VP9 — web-only, limited device support"),
        "prores" | "dnxhd" | "dnxhr" | "cineform" => {
            warn(label, format!("{codec_name} — editing codec, not for delivery"))
        }
        "" => fail(label, "No video codec detected"),
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
        "0" | "" => pass(label, "8-bit (compressed — normal for H.264/AAC)"),
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
        warn(label, format!("{fps:.3} fps — non-standard, verify with platform"))
    }
}

fn parse_fps(s: &str) -> Option<f64> {
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

fn check_rotation(tags: &[MediaKeyValue]) -> DiagnosticCheck {
    let label = "Rotation";
    let rotate = tag_value(tags, "rotate").unwrap_or_default();
    match rotate.trim() {
        "" | "0" => pass(label, "No rotation — correct orientation"),
        r => warn(label, format!("rotate={r} — player must compensate")),
    }
}

fn check_h264_profile(profile: &str) -> DiagnosticCheck {
    let label = "H.264 profile";
    match profile {
        "Baseline" | "Constrained Baseline" => {
            pass(label, "Baseline — maximum mobile compatibility")
        }
        "Main" => pass(label, "Main — good compatibility"),
        "High" => warn(label, "High profile — may fail on older mobile/TV"),
        "" => warn(label, "Profile unknown"),
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

fn check_audio_codec(codec_name: &str) -> DiagnosticCheck {
    let label = "Audio codec";
    match codec_name {
        "aac" => pass(label, "AAC — universal support"),
        "mp3" => pass(label, "MP3 — widely supported"),
        "pcm_s16le" | "pcm_s24le" | "pcm_s32le" => {
            pass(label, format!("{codec_name} — broadcast PCM"))
        }
        "opus" => warn(label, "Opus — web-only, limited device support"),
        "vorbis" => warn(label, "Vorbis — web-only, not for broadcast"),
        "ac3" | "eac3" => warn(
            label,
            format!("{codec_name} — broadcast format, limited browser support"),
        ),
        "" => warn(label, "Audio codec unknown"),
        _ => warn(label, format!("{codec_name} — verify platform support")),
    }
}

fn check_audio_channels(channel_layout: &str) -> DiagnosticCheck {
    let label = "Audio channels";
    match channel_layout {
        "mono" => pass(label, "Mono — broadcast standard"),
        "stereo" => warn(label, "Stereo — use dual mono for broadcast delivery"),
        "" => warn(label, "Channel layout unknown"),
        l => warn(label, format!("{l} — verify with delivery spec")),
    }
}

fn check_sample_rate(sample_rate: &str) -> DiagnosticCheck {
    let label = "Sample rate";
    match sample_rate.trim() {
        "48000" => pass(label, "48 kHz — broadcast standard"),
        "44100" => warn(label, "44.1 kHz — resample to 48 kHz for broadcast"),
        "" => warn(label, "Sample rate unknown"),
        r => warn(label, format!("{r} Hz — non-standard, verify with platform")),
    }
}

fn check_audio_bit_depth(bits_per_sample: &str) -> DiagnosticCheck {
    let label = "Audio bit depth";
    match bits_per_sample.trim() {
        "16" => pass(label, "16-bit PCM — broadcast standard"),
        "24" => pass(label, "24-bit PCM — acceptable"),
        "0" | "" => pass(label, "Compressed audio (AAC/MP3) — bit depth not applicable"),
        b => warn(label, format!("{b}-bit — verify with delivery spec")),
    }
}

fn check_subtitles(subtitle_count: usize) -> DiagnosticCheck {
    let label = "Subtitles";
    if subtitle_count > 0 {
        pass(label, format!("{subtitle_count} stream(s) embedded"))
    } else {
        warn(label, "No embedded subtitles — check if sidecar .srt needed")
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
    let label = "A/V sync";
    let v: f64 = video_start.trim().parse().unwrap_or(0.0);
    let a: f64 = audio_start.trim().parse().unwrap_or(0.0);
    let diff_ms = ((v - a).abs() * 1000.0) as u64;
    if diff_ms < 100 {
        pass(label, format!("Drift {diff_ms} ms — within tolerance"))
    } else {
        warn(label, format!("Drift {diff_ms} ms — may cause sync issues"))
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::media_read::data::media_probe_report::{
        MediaProbeReport, MediaStreamInfo,
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
                    frame_rate: "25/1".into(),
                    profile: "Main".into(),
                    ..Default::default()
                },
                MediaStreamInfo {
                    codec_type: "audio".into(),
                    codec_name: "aac".into(),
                    channel_layout: "stereo".into(),
                    sample_rate: "48000".into(),
                    bits_per_sample: "0".into(),
                    ..Default::default()
                },
            ],
            ..Default::default()
        }
    }

    #[test]
    fn clean_file_has_no_fails() {
        let report = make_report();
        let diag = run(&report);
        assert_eq!(diag.fail_count(), 0);
    }

    #[test]
    fn no_audio_is_fail() {
        let mut report = make_report();
        report.audio_count = 0;
        report.streams.retain(|s| s.codec_type != "audio");
        let diag = run(&report);
        assert_eq!(diag.fail_count(), 1);
    }

    #[test]
    fn stereo_audio_is_warn() {
        let report = make_report();
        let diag = run(&report);
        let audio_ch = diag
            .checks
            .iter()
            .find(|c| c.label == "Audio channels")
            .unwrap();
        assert!(matches!(audio_ch.status, DiagnosticStatus::Warn));
    }
}
