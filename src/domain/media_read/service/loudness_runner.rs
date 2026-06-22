#[cfg(feature = "server")]
use std::path::Path;

#[cfg(feature = "server")]
use crate::domain::media_read::data::media_probe_report::LoudnessReport;

/// Run ffmpeg ebur128 filter to measure R128 loudness.
/// Returns None if no audio stream or if ffmpeg fails.
#[cfg(feature = "server")]
pub(crate) async fn run_loudness(ffmpeg_bin: &str, path: &Path) -> Option<LoudnessReport> {
    use tokio::process::Command;
    use tracing::warn;

    let output = Command::new(ffmpeg_bin)
        .args(["-hide_banner", "-i"])
        .arg(path)
        .args(["-af", "ebur128=peak=true", "-f", "null", "-"])
        .output()
        .await
        .ok()?;

    // ebur128 summary goes to stderr regardless of exit code
    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() && !stderr.contains("Integrated loudness") {
        warn!(
            ffmpeg_bin,
            path = %path.display(),
            "loudness measurement failed"
        );
        return None;
    }

    parse_ebur128_summary(&stderr)
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
