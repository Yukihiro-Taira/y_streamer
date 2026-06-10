#[cfg(feature = "server")]
use std::path::Path;

/// Extract 3 JPEG thumbnail frames from a video at 10%, 50%, 90% of duration.
/// Returns base64 data URLs. Silently skips failed frames.
#[cfg(feature = "server")]
pub async fn generate_thumbnails(
    ffmpeg_bin: &str,
    input_path: &Path,
    duration_secs: f64,
    trace_id: &str,
    temp_dir: &Path,
) -> Vec<String> {
    use base64::Engine;
    use tracing::{info, warn};

    if duration_secs <= 0.0 {
        return vec![];
    }

    let timestamps = [
        duration_secs * 0.10,
        duration_secs * 0.50,
        duration_secs * 0.90,
    ];

    let mut results = Vec::new();

    for (i, &ts) in timestamps.iter().enumerate() {
        let out_path = temp_dir.join(format!("thumb-{trace_id}-{i}.jpg"));

        let status = tokio::process::Command::new(ffmpeg_bin)
            .args([
                "-y",
                "-ss",
                &format!("{ts:.3}"),
                "-i",
                input_path.to_str().unwrap_or_default(),
                "-vframes",
                "1",
                "-q:v",
                "5",
                "-vf",
                "scale=320:-1",
                out_path.to_str().unwrap_or_default(),
            ])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .await;

        match status {
            Ok(s) if s.success() => match tokio::fs::read(&out_path).await {
                Ok(bytes) => {
                    let encoded = base64::engine::general_purpose::STANDARD.encode(&bytes);
                    results.push(format!("data:image/jpeg;base64,{encoded}"));
                    info!(trace_id, index = i, ts, "thumbnail generated");
                    let _ = tokio::fs::remove_file(&out_path).await;
                }
                Err(err) => warn!(trace_id, index = i, %err, "failed to read thumbnail file"),
            },
            Ok(s) => warn!(trace_id, index = i, code = ?s.code(), "ffmpeg thumbnail exit non-zero"),
            Err(err) => warn!(trace_id, index = i, %err, "ffmpeg thumbnail spawn failed"),
        }
    }

    results
}
