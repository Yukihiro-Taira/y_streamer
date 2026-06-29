#[cfg(feature = "server")]
use std::path::Path;

/// Extract a PNG waveform preview from the audio track and return it as a data URL.
#[cfg(feature = "server")]
pub async fn generate_waveform(
    ffmpeg_bin: &str,
    input_path: &Path,
    trace_id: &str,
    temp_dir: &Path,
) -> String {
    use base64::Engine;
    use tracing::{info, warn};

    let out_path = temp_dir.join(format!("waveform-{trace_id}.png"));

    let status = tokio::process::Command::new(ffmpeg_bin)
        .args([
            "-y",
            "-i",
            input_path.to_str().unwrap_or_default(),
            "-frames:v",
            "1",
            "-filter_complex",
            "showwavespic=s=1400x280:colors=0x60a5fa:scale=sqrt:draw=full:filter=peak",
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
                let _ = tokio::fs::remove_file(&out_path).await;
                info!(trace_id, "waveform generated");
                format!("data:image/png;base64,{encoded}")
            }
            Err(err) => {
                warn!(trace_id, %err, "failed to read waveform image");
                String::new()
            }
        },
        Ok(s) => {
            warn!(trace_id, code = ?s.code(), "ffmpeg waveform exit non-zero");
            String::new()
        }
        Err(err) => {
            warn!(trace_id, %err, "ffmpeg waveform spawn failed");
            String::new()
        }
    }
}
