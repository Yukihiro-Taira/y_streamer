#[cfg(feature = "server")]
use std::path::Path;

#[cfg(feature = "server")]
use crate::domain::media_read::data::media_probe_report::LoudnessReport;

/// Run ffmpeg ebur128 filter to measure R128 loudness.
/// Returns Err if no audio stream or if ffmpeg fails.
#[cfg(feature = "server")]
pub(crate) async fn run_loudness(ffmpeg_bin: &str, path: &Path) -> Result<LoudnessReport, String> {
    use std::io::ErrorKind;

    use tokio::process::Command;

    let output = Command::new(ffmpeg_bin)
        .args(["-hide_banner", "-i"])
        .arg(path)
        .args(["-af", "ebur128=peak=true", "-f", "null", "-"])
        .output()
        .await
        .map_err(|err| {
            if err.kind() == ErrorKind::NotFound {
                format!("ffmpeg not found (bin: {ffmpeg_bin})")
            } else {
                format!("failed to launch ffmpeg for loudness: {err}")
            }
        })?;

    // ebur128 summary goes to stderr regardless of exit code
    let stderr = String::from_utf8_lossy(&output.stderr);

    parse_ebur128_summary(&stderr).ok_or_else(|| {
        "no R128 summary found — file may have no audio or unsupported format".to_string()
    })
}

#[cfg(feature = "server")]
fn parse_ebur128_summary(stderr: &str) -> Option<LoudnessReport> {
    let extract = |label: &str| -> String {
        stderr
            .lines()
            .find(|l| l.trim_start().starts_with(label))
            .and_then(|l| l.split(':').nth(1))
            .map(|v| v.trim().split_whitespace().next().unwrap_or("").to_string())
            .unwrap_or_default()
    };

    let integrated = extract("I:");
    if integrated.is_empty() {
        return None;
    }

    Some(LoudnessReport {
        integrated_lufs: integrated,
        integrated_threshold: extract("Threshold:"),
        lra_lu: extract("LRA:"),
        lra_low: extract("LRA low:"),
        lra_high: extract("LRA high:"),
        true_peak_dbtp: extract("Peak:"),
    })
}
