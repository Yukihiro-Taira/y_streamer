#[cfg(feature = "server")]
use std::path::Path;

#[cfg(feature = "server")]
use crate::domain::media_read::data::media_probe_report::MediaSceneCut;

/// Detect likely scene cuts using ffmpeg scene score selection.
#[cfg(feature = "server")]
pub async fn detect_scenes(ffmpeg_bin: &str, input_path: &Path) -> Result<Vec<MediaSceneCut>, String> {
    use std::io::ErrorKind;

    use tokio::process::Command;

    let output = Command::new(ffmpeg_bin)
        .args(["-hide_banner", "-i"])
        .arg(input_path)
        .args([
            "-filter:v",
            "select='gt(scene,0.35)',showinfo",
            "-an",
            "-f",
            "null",
            "-",
        ])
        .output()
        .await
        .map_err(|err| {
            if err.kind() == ErrorKind::NotFound {
                format!("ffmpeg not found (bin: {ffmpeg_bin})")
            } else {
                format!("failed to launch ffmpeg for scene detection: {err}")
            }
        })?;

    let stderr = String::from_utf8_lossy(&output.stderr);
    Ok(parse_scene_cuts(&stderr))
}

#[cfg(feature = "server")]
fn parse_scene_cuts(stderr: &str) -> Vec<MediaSceneCut> {
    stderr
        .lines()
        .filter(|line| line.contains("showinfo"))
        .filter_map(|line| {
            let timestamp_secs = extract_token(line, "pts_time:")?.parse::<f64>().ok()?;
            let score = extract_token(line, "scene:").unwrap_or_default();
            Some(MediaSceneCut {
                timestamp_secs,
                score,
            })
        })
        .collect()
}

#[cfg(feature = "server")]
fn extract_token(line: &str, key: &str) -> Option<String> {
    let idx = line.find(key)?;
    let value = &line[idx + key.len()..];
    let token = value.split_whitespace().next()?.trim();
    if token.is_empty() {
        None
    } else {
        Some(token.to_string())
    }
}

#[cfg(all(test, feature = "server"))]
mod tests {
    use super::*;

    #[test]
    fn parses_showinfo_scene_lines() {
        let stderr = "[Parsed_showinfo_1 @ 0x0] n:   0 pts: 3003 pts_time:3.003 duration:1 duration_time:0.0417083 fmt:yuv420p sar:1/1 s:1920x1080 i:P iskey:1 type:I checksum:000 plane_checksum:[000] mean:[0] stdev:[0] scene:0.412\n";
        let cuts = parse_scene_cuts(stderr);
        assert_eq!(cuts.len(), 1);
        assert!((cuts[0].timestamp_secs - 3.003).abs() < 0.001);
        assert_eq!(cuts[0].score, "0.412");
    }
}
