#[cfg(feature = "server")]
use std::path::{Path, PathBuf};

#[cfg(feature = "server")]
use crate::domain::media_read::data::media_probe_report::{
    MediaProbeErrorResponse, MediaProbeReport,
};
#[cfg(feature = "server")]
use crate::domain::media_read::service::ffprobe_runner::inspect_media_path;
#[cfg(feature = "server")]
use crate::domain::media_read::service::loudness_runner::run_loudness;
#[cfg(feature = "server")]
use crate::domain::media_read::service::mediainfo_runner::run_mediainfo;
#[cfg(feature = "server")]
use crate::domain::media_read::service::runtime_config::MediaReadRuntimeConfig;
#[cfg(feature = "server")]
use crate::domain::media_read::service::subtitle_extractor::extract_subtitles;
#[cfg(feature = "server")]
use crate::domain::media_read::service::thumbnail_generator::generate_thumbnails;
#[cfg(feature = "server")]
use crate::domain::observability::error_reporter::report_server_error;

#[cfg(feature = "server")]
pub fn media_read_upload_limit_bytes() -> usize {
    MediaReadRuntimeConfig::from_env().max_upload_bytes
}

#[cfg(feature = "server")]
pub async fn media_read_upload_handler(
    dioxus::server::axum::Extension(pool): dioxus::server::axum::Extension<sqlx::PgPool>,
    mut multipart: axum::extract::Multipart,
) -> Result<
    axum::Json<MediaProbeReport>,
    (axum::http::StatusCode, axum::Json<MediaProbeErrorResponse>),
> {
    use tokio::fs::File;
    use tokio::io::AsyncWriteExt;
    use tracing::{Instrument, error, info, info_span, warn};

    let config = MediaReadRuntimeConfig::from_env();
    let started_at = std::time::Instant::now();
    let trace_id = uuid::Uuid::new_v4().to_string();
    let span = info_span!(
        "media_read_upload",
        trace_id = %trace_id,
        ffprobe_timeout_secs = config.ffprobe_timeout_secs,
        max_upload_bytes = config.max_upload_bytes
    );

    async move {
        info!(
            temp_dir = %config.temp_dir.display(),
            ffprobe_bin = %config.ffprobe_bin,
            "upload started"
        );

        while let Some(field) = multipart.next_field().await.map_err(|err| {
            bad_request_error(&trace_id, format!("failed to read multipart field: {err}"))
        })? {
            let field_name = field.name().map(str::to_owned).unwrap_or_default();
            if field_name != "file" {
                warn!(field_name = %field_name, "ignoring non-file multipart field");
                continue;
            }

            let original_file_name = field
                .file_name()
                .map(str::to_owned)
                .filter(|value| !value.is_empty())
                .unwrap_or_else(|| "upload.bin".into());
            let content_type = field
                .content_type()
                .map(str::to_owned)
                .unwrap_or_else(|| "application/octet-stream".into());

            tokio::fs::create_dir_all(&config.temp_dir)
                .await
                .map_err(|err| {
                    internal_error_response(
                        &trace_id,
                        format!("failed to create media temp dir: {err}"),
                    )
                })?;

            let temp_path = make_temp_media_path(&config.temp_dir, &original_file_name, &trace_id);
            let mut output = File::create(&temp_path).await.map_err(|err| {
                internal_error_response(
                    &trace_id,
                    format!("failed to create temp upload file: {err}"),
                )
            })?;

            let mut field = field;
            let mut stored_bytes: u64 = 0;
            let mut chunk_count: u64 = 0;

            info!(
                file_name = %original_file_name,
                content_type = %content_type,
                temp_path = %temp_path.display(),
                "streaming upload into temp file"
            );

            while let Some(chunk) = field.chunk().await.map_err(|err| {
                bad_request_error(&trace_id, format!("failed to read upload chunk: {err}"))
            })? {
                stored_bytes += chunk.len() as u64;
                chunk_count += 1;
                output.write_all(&chunk).await.map_err(|err| {
                    internal_error_response(
                        &trace_id,
                        format!("failed to write upload chunk: {err}"),
                    )
                })?;
            }

            output.flush().await.map_err(|err| {
                internal_error_response(
                    &trace_id,
                    format!("failed to flush temp upload file: {err}"),
                )
            })?;
            drop(output);

            if stored_bytes == 0 {
                let _ = tokio::fs::remove_file(&temp_path).await;
                return Err(bad_request_error(
                    &trace_id,
                    "uploaded file is empty".to_string(),
                ));
            }

            info!(
                file_name = %original_file_name,
                content_type = %content_type,
                stored_bytes,
                chunk_count,
                elapsed_ms = started_at.elapsed().as_millis() as u64,
                "upload fully written; starting ffprobe"
            );

            let inspection = inspect_media_path(
                &config,
                &temp_path,
                &original_file_name,
                &trace_id,
                stored_bytes,
                started_at,
            )
            .await
            .map_err(|err| {
                error!(
                    file_name = %original_file_name,
                    stored_bytes,
                    error = %err,
                    "ffprobe inspection failed"
                );
                report_server_error(
                    pool.clone(),
                    "media_read.ffprobe_failed",
                    format!("[trace_id={trace_id}] file={original_file_name}: {err}"),
                );
                unprocessable_error(&trace_id, err.to_string())
            });

            let mut report = match inspection {
                Ok(r) => r,
                Err(err) => {
                    let _ = tokio::fs::remove_file(&temp_path).await;
                    return Err(err);
                }
            };

            // Run thumbnails, subtitles, mediainfo, loudness in parallel
            let duration_secs: f64 = report
                .duration
                .split_whitespace()
                .next()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0.0);
            let has_video = report.video_count > 0;
            let has_audio = report.audio_count > 0;
            let streams_clone = report.streams.clone();

            let (thumbnails, subtitles, mediainfo, loudness) = tokio::join!(
                async {
                    if has_video {
                        generate_thumbnails(
                            &config.ffmpeg_bin,
                            &temp_path,
                            duration_secs,
                            &trace_id,
                            &config.temp_dir,
                        )
                        .await
                    } else {
                        vec![]
                    }
                },
                async {
                    if report.subtitle_count > 0 {
                        extract_subtitles(
                            &config.ffmpeg_bin,
                            &temp_path,
                            &streams_clone,
                            &trace_id,
                            &config.temp_dir,
                        )
                        .await
                    } else {
                        vec![]
                    }
                },
                run_mediainfo(&config.mediainfo_bin, &temp_path),
                async {
                    if has_audio {
                        run_loudness(&config.ffmpeg_bin, &temp_path).await
                    } else {
                        Err("no audio stream".to_string())
                    }
                },
            );

            report.thumbnails = thumbnails;
            report.subtitles = subtitles;
            match mediainfo {
                Ok(r) => report.mediainfo = Some(r),
                Err(e) => report.mediainfo_error = Some(e),
            }
            match loudness {
                Ok(r) => report.loudness = Some(r),
                Err(e) => report.loudness_error = Some(e),
            }

            if let Err(err) = tokio::fs::remove_file(&temp_path).await {
                warn!(temp_path = %temp_path.display(), error = %err, "temp file cleanup failed");
            } else {
                info!(
                    file_name = %original_file_name,
                    stored_bytes,
                    elapsed_ms = started_at.elapsed().as_millis() as u64,
                    "temp file cleanup completed"
                );
            }

            return Ok(axum::Json(report));
        }

        Err(bad_request_error(
            &trace_id,
            "missing multipart field `file`".to_string(),
        ))
    }
    .instrument(span)
    .await
}

#[cfg(feature = "server")]
fn make_temp_media_path(temp_dir: &Path, file_name: &str, trace_id: &str) -> PathBuf {
    let extension = Path::new(file_name)
        .extension()
        .and_then(|value| value.to_str())
        .filter(|value| !value.is_empty())
        .unwrap_or("bin");
    let safe_stem = Path::new(file_name)
        .file_stem()
        .and_then(|value| value.to_str())
        .map(sanitize_file_component)
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "upload".into());

    temp_dir.join(format!("media-read-{trace_id}-{safe_stem}.{extension}"))
}

#[cfg(feature = "server")]
fn sanitize_file_component(raw: &str) -> String {
    raw.chars()
        .map(|ch| match ch {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' => ch,
            _ => '-',
        })
        .collect()
}

#[cfg(feature = "server")]
fn bad_request_error(
    trace_id: &str,
    message: String,
) -> (axum::http::StatusCode, axum::Json<MediaProbeErrorResponse>) {
    error_response(axum::http::StatusCode::BAD_REQUEST, trace_id, message)
}

#[cfg(feature = "server")]
fn unprocessable_error(
    trace_id: &str,
    message: String,
) -> (axum::http::StatusCode, axum::Json<MediaProbeErrorResponse>) {
    error_response(
        axum::http::StatusCode::UNPROCESSABLE_ENTITY,
        trace_id,
        message,
    )
}

#[cfg(feature = "server")]
fn internal_error_response(
    trace_id: &str,
    message: String,
) -> (axum::http::StatusCode, axum::Json<MediaProbeErrorResponse>) {
    error_response(
        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        trace_id,
        message,
    )
}

#[cfg(feature = "server")]
fn error_response(
    status: axum::http::StatusCode,
    trace_id: &str,
    message: String,
) -> (axum::http::StatusCode, axum::Json<MediaProbeErrorResponse>) {
    (
        status,
        axum::Json(MediaProbeErrorResponse {
            trace_id: trace_id.to_string(),
            message,
        }),
    )
}
