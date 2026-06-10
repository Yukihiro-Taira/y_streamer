#[cfg(feature = "server")]
use std::path::Path;

#[cfg(feature = "server")]
use crate::domain::media_read::data::media_probe_report::{MediaSubtitle, MediaStreamInfo};

/// Extract all subtitle streams from a media file to SRT text.
#[cfg(feature = "server")]
pub async fn extract_subtitles(
    ffmpeg_bin: &str,
    input_path: &Path,
    streams: &[MediaStreamInfo],
    trace_id: &str,
    temp_dir: &Path,
) -> Vec<MediaSubtitle> {
    use tracing::{info, warn};

    let sub_streams: Vec<(usize, &MediaStreamInfo)> = streams
        .iter()
        .enumerate()
        .filter(|(_, s)| s.codec_type == "subtitle")
        .collect();

    if sub_streams.is_empty() {
        return vec![];
    }

    let mut results = Vec::new();

    for (stream_order, (_global_idx, stream)) in sub_streams.iter().enumerate() {
        let out_path = temp_dir.join(format!("sub-{trace_id}-{stream_order}.srt"));

        let status = tokio::process::Command::new(ffmpeg_bin)
            .args([
                "-y",
                "-i",
                input_path.to_str().unwrap_or_default(),
                "-map",
                &format!("0:s:{stream_order}"),
                "-f",
                "srt",
                out_path.to_str().unwrap_or_default(),
            ])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .await;

        match status {
            Ok(s) if s.success() => {
                match tokio::fs::read_to_string(&out_path).await {
                    Ok(content) if !content.trim().is_empty() => {
                        let language = stream
                            .tags
                            .iter()
                            .find(|t| t.key.to_lowercase() == "language")
                            .map(|t| t.value.clone())
                            .unwrap_or_else(|| format!("track {stream_order}"));

                        info!(trace_id, stream_order, language, "subtitle extracted");
                        results.push(MediaSubtitle {
                            stream_index: stream_order,
                            language,
                            content,
                        });
                    }
                    Ok(_) => warn!(trace_id, stream_order, "subtitle extracted but empty"),
                    Err(err) => warn!(trace_id, stream_order, %err, "failed to read subtitle file"),
                }
                let _ = tokio::fs::remove_file(&out_path).await;
            }
            Ok(s) => warn!(trace_id, stream_order, code = ?s.code(), "ffmpeg subtitle exit non-zero"),
            Err(err) => warn!(trace_id, stream_order, %err, "ffmpeg subtitle spawn failed"),
        }
    }

    results
}
