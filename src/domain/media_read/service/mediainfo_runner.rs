#[cfg(feature = "server")]
use std::path::Path;

#[cfg(feature = "server")]
use crate::domain::media_read::data::media_probe_report::MediaInfoReport;

#[cfg(feature = "server")]
pub(crate) async fn run_mediainfo(bin: &str, path: &Path) -> Result<MediaInfoReport, String> {
    use std::io::ErrorKind;
    use serde_json::Value;
    use tokio::process::Command;

    let output = Command::new(bin)
        .args(["--Output=JSON", "--Full"])
        .arg(path)
        .output()
        .await
        .map_err(|err| {
            if err.kind() == ErrorKind::NotFound {
                format!("mediainfo not found — install with `brew install mediainfo` or `apt install mediainfo` (bin: {bin})")
            } else {
                format!("failed to launch mediainfo: {err}")
            }
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(format!("mediainfo exited non-zero: {stderr}"));
    }

    let raw_json = String::from_utf8_lossy(&output.stdout).to_string();
    let parsed: Value = serde_json::from_str(&raw_json)
        .map_err(|e| format!("failed to parse mediainfo JSON: {e}"))?;

    let tracks = parsed
        .pointer("/media/track")
        .and_then(|v| v.as_array())
        .ok_or_else(|| "mediainfo JSON missing /media/track".to_string())?;

    let general = tracks
        .iter()
        .find(|t| t.get("@type").and_then(|v| v.as_str()) == Some("General"));
    let video = tracks
        .iter()
        .find(|t| t.get("@type").and_then(|v| v.as_str()) == Some("Video"));
    let audio = tracks
        .iter()
        .find(|t| t.get("@type").and_then(|v| v.as_str()) == Some("Audio"));

    let str_field = |track: Option<&Value>, key: &str| -> String {
        track
            .and_then(|t| t.get(key))
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string()
    };

    Ok(MediaInfoReport {
        hdr_format: str_field(video, "HDR_Format"),
        hdr_format_compatibility: str_field(video, "HDR_Format_Compatibility"),
        writing_library: str_field(general, "Writing_Library"),
        encoded_application: str_field(general, "Encoded_Application"),
        audio_delay_ms: str_field(audio, "Delay"),
        standard: str_field(video, "Standard"),
        format_profile: {
            let profile = str_field(video, "Format_Profile");
            let level = str_field(video, "Format_Level");
            match (profile.is_empty(), level.is_empty()) {
                (false, false) => format!("{profile}@L{level}"),
                (false, true) => profile,
                _ => String::new(),
            }
        },
        frame_rate_num: str_field(video, "FrameRate_Num"),
        frame_rate_den: str_field(video, "FrameRate_Den"),
        scan_order: str_field(video, "ScanOrder"),
        bit_depth: str_field(video, "BitDepth"),
        raw_json,
    })
}
