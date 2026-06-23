#[cfg(feature = "server")]
use std::path::Path;

#[cfg(feature = "server")]
use crate::domain::media_read::data::media_probe_report::MediaProbeReport;
#[cfg(feature = "server")]
use crate::domain::media_read::service::ffprobe_mapper::{
    map_ffprobe_report, parse_ffprobe_output,
};
#[cfg(feature = "server")]
use crate::domain::media_read::service::runtime_config::MediaReadRuntimeConfig;

#[cfg(feature = "server")]
pub(crate) async fn inspect_media_path(
    config: &MediaReadRuntimeConfig,
    path: &Path,
    file_name: &str,
    trace_id: &str,
    upload_bytes: u64,
    started_at: std::time::Instant,
) -> anyhow::Result<MediaProbeReport> {
    use anyhow::{Context, anyhow};
    use tokio::process::Command;
    use tokio::time::{Duration, timeout};
    use tracing::{info, instrument};

    #[instrument(skip_all, fields(path = %path.display(), ffprobe_bin = %config.ffprobe_bin))]
    async fn run_ffprobe(
        config: &MediaReadRuntimeConfig,
        path: &Path,
    ) -> anyhow::Result<std::process::Output> {
        let mut command = Command::new(&config.ffprobe_bin);
        command.kill_on_drop(true);
        command.args([
            "-v",
            "error",
            "-print_format",
            "json",
            "-show_format",
            "-show_streams",
            "-show_chapters",
            "-show_programs",
        ]);
        command.arg(path);

        let timeout_window = Duration::from_secs(config.ffprobe_timeout_secs);
        let output = timeout(timeout_window, command.output())
            .await
            .map_err(|_| {
                anyhow!(
                    "ffprobe timed out after {} seconds",
                    config.ffprobe_timeout_secs
                )
            })?
            .context("failed to launch ffprobe")?;

        Ok(output)
    }

    let output = run_ffprobe(config, path).await?;
    info!(
        stdout_bytes = output.stdout.len(),
        stderr_bytes = output.stderr.len(),
        "ffprobe finished"
    );

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        let message = if stderr.is_empty() {
            format!("ffprobe exited with status {}", output.status)
        } else {
            stderr
        };
        return Err(anyhow!(message));
    }

    let (parsed, raw_json_pretty) = parse_ffprobe_output(&output.stdout)?;
    let report = map_ffprobe_report(
        trace_id,
        path,
        file_name,
        upload_bytes,
        started_at.elapsed().as_millis() as u64,
        config.ffprobe_timeout_secs,
        parsed,
        raw_json_pretty,
    );
    info!(
        stream_count = report.stream_count,
        video_count = report.video_count,
        audio_count = report.audio_count,
        subtitle_count = report.subtitle_count,
        chapter_count = report.chapter_count,
        processing_time_ms = report.processing_time_ms,
        "ffprobe report mapped successfully"
    );

    Ok(report)
}
