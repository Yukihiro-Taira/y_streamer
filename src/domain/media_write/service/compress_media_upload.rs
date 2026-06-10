#[cfg(feature = "server")]
use std::path::{Path, PathBuf};

#[cfg(feature = "server")]
use crate::domain::media_write::data::media_write_job::{
    MediaWriteErrorResponse, MediaWriteResult,
};
#[cfg(feature = "server")]
use crate::domain::media_write::service::artifact_store::{
    MediaWriteArtifact, get_artifact, insert_artifact,
};
#[cfg(feature = "server")]
use crate::domain::media_write::service::runtime_config::MediaWriteRuntimeConfig;
#[cfg(feature = "server")]
use crate::domain::observability::error_reporter::report_server_error;

#[cfg(feature = "server")]
#[derive(Clone, Copy, Debug)]
enum MediaWriteOperation {
    Compress,
    Transcode,
}

#[cfg(feature = "server")]
struct MediaWriteForm {
    file_name: String,
    input_path: PathBuf,
    input_bytes: u64,
    output_container: String,
    video_codec: String,
    audio_codec: String,
    crf: String,
    preset: String,
    audio_bitrate: String,
}

#[cfg(feature = "server")]
pub fn media_write_upload_limit_bytes() -> usize {
    MediaWriteRuntimeConfig::from_env().max_upload_bytes
}

#[cfg(feature = "server")]
pub async fn media_write_compress_handler(
    dioxus::server::axum::Extension(pool): dioxus::server::axum::Extension<sqlx::PgPool>,
    multipart: axum::extract::Multipart,
) -> Result<
    axum::Json<MediaWriteResult>,
    (axum::http::StatusCode, axum::Json<MediaWriteErrorResponse>),
> {
    run_media_write_job(MediaWriteOperation::Compress, pool, multipart)
        .await
        .map(axum::Json)
}

#[cfg(feature = "server")]
pub async fn media_write_transcode_handler(
    dioxus::server::axum::Extension(pool): dioxus::server::axum::Extension<sqlx::PgPool>,
    multipart: axum::extract::Multipart,
) -> Result<
    axum::Json<MediaWriteResult>,
    (axum::http::StatusCode, axum::Json<MediaWriteErrorResponse>),
> {
    run_media_write_job(MediaWriteOperation::Transcode, pool, multipart)
        .await
        .map(axum::Json)
}

#[cfg(feature = "server")]
pub async fn media_write_artifact_download_handler(
    axum::extract::Path(artifact_id): axum::extract::Path<String>,
) -> Result<
    impl axum::response::IntoResponse,
    (axum::http::StatusCode, axum::Json<MediaWriteErrorResponse>),
> {
    use axum::http::header::{CONTENT_DISPOSITION, CONTENT_TYPE};

    let trace_id = uuid::Uuid::new_v4().to_string();
    let artifact = get_artifact(&artifact_id)
        .ok_or_else(|| bad_request_error(&trace_id, "unknown artifact id".to_string()))?;
    let bytes = tokio::fs::read(&artifact.path).await.map_err(|err| {
        internal_error(
            &trace_id,
            format!("failed to read artifact file for download: {err}"),
        )
    })?;

    Ok((
        [
            (CONTENT_TYPE, artifact.content_type),
            (
                CONTENT_DISPOSITION,
                format!("attachment; filename=\"{}\"", artifact.file_name),
            ),
        ],
        bytes,
    ))
}

#[cfg(feature = "server")]
async fn run_media_write_job(
    operation: MediaWriteOperation,
    pool: sqlx::PgPool,
    mut multipart: axum::extract::Multipart,
) -> Result<MediaWriteResult, (axum::http::StatusCode, axum::Json<MediaWriteErrorResponse>)> {
    use tokio::process::Command;
    use tokio::time::{Duration, timeout};
    use tracing::{Instrument, error, info, info_span};

    let config = MediaWriteRuntimeConfig::from_env();
    let started_at = std::time::Instant::now();
    let trace_id = uuid::Uuid::new_v4().to_string();
    let span = info_span!(
        "media_write_job",
        trace_id = %trace_id,
        operation = %operation_name(operation),
        ffmpeg_timeout_secs = config.ffmpeg_timeout_secs,
        max_upload_bytes = config.max_upload_bytes
    );

    async move {
        let form = parse_media_write_form(&config, &trace_id, operation, &mut multipart).await?;
        let output_file_name = derive_output_file_name(
            &form.file_name,
            &form.output_container,
            operation_name(operation),
        );
        let output_path = config.temp_dir.join(format!(
            "media-write-output-{trace_id}-{}",
            sanitize_file_component(&output_file_name)
        ));
        let command_summary = build_command_summary(operation, &config, &form);

        info!(
            input_file_name = %form.file_name,
            input_bytes = form.input_bytes,
            output_file_name = %output_file_name,
            video_codec = %form.video_codec,
            audio_codec = %form.audio_codec,
            output_container = %form.output_container,
            "starting media_write job"
        );

        let mut command = Command::new(&config.ffmpeg_bin);
        command.kill_on_drop(true);
        command.args(["-y", "-i"]).arg(&form.input_path);
        apply_ffmpeg_args(&mut command, operation, &form);
        command.arg(&output_path);

        let output = timeout(
            Duration::from_secs(config.ffmpeg_timeout_secs),
            command.output(),
        )
        .await
        .map_err(|_| {
            unprocessable_error(
                &trace_id,
                format!(
                    "ffmpeg timed out after {} seconds",
                    config.ffmpeg_timeout_secs
                ),
            )
        })?
        .map_err(|err| internal_error(&trace_id, format!("failed to launch ffmpeg: {err}")))?;

        let _ = tokio::fs::remove_file(&form.input_path).await;

        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        if !output.status.success() {
            let message = if stderr.is_empty() {
                format!("ffmpeg exited with status {}", output.status)
            } else {
                stderr
            };
            let _ = tokio::fs::remove_file(&output_path).await;
            error!(message = %message, "media_write job failed");
            report_server_error(
                pool,
                format!("media_write.{}_failed", operation_name(operation)),
                format!("[trace_id={trace_id}]: {message}"),
            );
            return Err(unprocessable_error(&trace_id, message));
        }

        let output_meta = tokio::fs::metadata(&output_path).await.map_err(|err| {
            internal_error(&trace_id, format!("failed to stat output artifact: {err}"))
        })?;
        let output_bytes = output_meta.len();
        let saved_bytes = form.input_bytes.saturating_sub(output_bytes);
        let saved_percent = if form.input_bytes == 0 {
            0.0
        } else {
            (saved_bytes as f64 / form.input_bytes as f64) * 100.0
        };
        let job_id = uuid::Uuid::new_v4().to_string();
        let artifact_id = job_id.clone();
        insert_artifact(
            artifact_id.clone(),
            MediaWriteArtifact {
                path: output_path,
                file_name: output_file_name.clone(),
                content_type: content_type_for_container(&form.output_container).to_string(),
            },
        );

        Ok(MediaWriteResult {
            trace_id,
            job_id,
            operation: operation_name(operation).to_string(),
            output_file_name,
            download_url: format!("/api/media-write/artifacts/{artifact_id}"),
            output_container: form.output_container,
            video_codec: form.video_codec,
            audio_codec: form.audio_codec,
            input_bytes: form.input_bytes,
            output_bytes,
            saved_bytes,
            saved_percent,
            elapsed_ms: started_at.elapsed().as_millis() as u64,
            ffmpeg_timeout_secs: config.ffmpeg_timeout_secs,
            command_summary,
            stderr_excerpt: tail_excerpt(&stderr, 4000),
        })
    }
    .instrument(span)
    .await
}

#[cfg(feature = "server")]
async fn parse_media_write_form(
    config: &MediaWriteRuntimeConfig,
    trace_id: &str,
    operation: MediaWriteOperation,
    multipart: &mut axum::extract::Multipart,
) -> Result<MediaWriteForm, (axum::http::StatusCode, axum::Json<MediaWriteErrorResponse>)> {
    use tokio::fs::File;
    use tokio::io::AsyncWriteExt;
    use tracing::warn;

    let mut file_name: Option<String> = None;
    let mut input_path: Option<PathBuf> = None;
    let mut input_bytes: u64 = 0;
    let mut output_container = default_output_container(operation).to_string();
    let mut video_codec = default_video_codec(operation).to_string();
    let mut audio_codec = default_audio_codec(operation).to_string();
    let mut crf = "23".to_string();
    let mut preset = "fast".to_string();
    let mut audio_bitrate = "128k".to_string();

    tokio::fs::create_dir_all(&config.temp_dir)
        .await
        .map_err(|err| internal_error(trace_id, format!("failed to create temp dir: {err}")))?;

    while let Some(field) = multipart.next_field().await.map_err(|err| {
        bad_request_error(trace_id, format!("failed to read multipart field: {err}"))
    })? {
        let field_name = field.name().map(str::to_owned).unwrap_or_default();

        match field_name.as_str() {
            "file" => {
                let original_file_name = field
                    .file_name()
                    .map(str::to_owned)
                    .filter(|value| !value.is_empty())
                    .unwrap_or_else(|| "upload.bin".into());
                let temp_path =
                    make_temp_media_path(&config.temp_dir, &original_file_name, trace_id);
                let mut output = File::create(&temp_path).await.map_err(|err| {
                    internal_error(
                        trace_id,
                        format!("failed to create temp upload file: {err}"),
                    )
                })?;
                let mut field = field;
                while let Some(chunk) = field.chunk().await.map_err(|err| {
                    bad_request_error(trace_id, format!("failed to read upload chunk: {err}"))
                })? {
                    input_bytes += chunk.len() as u64;
                    output.write_all(&chunk).await.map_err(|err| {
                        internal_error(trace_id, format!("failed to write upload chunk: {err}"))
                    })?;
                }
                output.flush().await.map_err(|err| {
                    internal_error(trace_id, format!("failed to flush temp upload file: {err}"))
                })?;
                drop(output);
                file_name = Some(original_file_name);
                input_path = Some(temp_path);
            }
            "output_container" => {
                if let Some(value) = field.text().await.ok().filter(|v| !v.trim().is_empty()) {
                    output_container = sanitize_output_container(&value);
                }
            }
            "video_codec" => {
                if let Some(value) = field.text().await.ok().filter(|v| !v.trim().is_empty()) {
                    video_codec = sanitize_video_codec(&value, operation);
                }
            }
            "audio_codec" => {
                if let Some(value) = field.text().await.ok().filter(|v| !v.trim().is_empty()) {
                    audio_codec = sanitize_audio_codec(&value, operation);
                }
            }
            "crf" => {
                if let Some(value) = field.text().await.ok().filter(|v| !v.trim().is_empty()) {
                    crf = value;
                }
            }
            "preset" => {
                if let Some(value) = field.text().await.ok().filter(|v| !v.trim().is_empty()) {
                    preset = value;
                }
            }
            "audio_bitrate" => {
                if let Some(value) = field.text().await.ok().filter(|v| !v.trim().is_empty()) {
                    audio_bitrate = value;
                }
            }
            _ => warn!(field_name = %field_name, "ignoring unknown media_write field"),
        }
    }

    let input_path = input_path
        .ok_or_else(|| bad_request_error(trace_id, "missing multipart field `file`".to_string()))?;
    let file_name = file_name.unwrap_or_else(|| "upload.bin".into());

    if input_bytes == 0 {
        let _ = tokio::fs::remove_file(&input_path).await;
        return Err(bad_request_error(
            trace_id,
            "uploaded file is empty".to_string(),
        ));
    }

    Ok(MediaWriteForm {
        file_name,
        input_path,
        input_bytes,
        output_container,
        video_codec,
        audio_codec,
        crf,
        preset,
        audio_bitrate,
    })
}

#[cfg(feature = "server")]
fn apply_ffmpeg_args(
    command: &mut tokio::process::Command,
    operation: MediaWriteOperation,
    form: &MediaWriteForm,
) {
    match operation {
        MediaWriteOperation::Compress => {
            command.args([
                "-c:v",
                ffmpeg_video_codec(&form.video_codec),
                "-preset",
                &form.preset,
                "-crf",
                &form.crf,
                "-c:a",
                ffmpeg_audio_codec(&form.audio_codec),
                "-b:a",
                &form.audio_bitrate,
            ]);
        }
        MediaWriteOperation::Transcode => {
            command.args([
                "-c:v",
                ffmpeg_video_codec(&form.video_codec),
                "-c:a",
                ffmpeg_audio_codec(&form.audio_codec),
            ]);
            if form.audio_codec != "copy" {
                command.args(["-b:a", &form.audio_bitrate]);
            }
        }
    }
}

#[cfg(feature = "server")]
fn build_command_summary(
    operation: MediaWriteOperation,
    config: &MediaWriteRuntimeConfig,
    form: &MediaWriteForm,
) -> String {
    match operation {
        MediaWriteOperation::Compress => format!(
            "{} -y -i <input> -c:v {} -preset {} -crf {} -c:a {} -b:a {} <output.{}>",
            config.ffmpeg_bin,
            ffmpeg_video_codec(&form.video_codec),
            form.preset,
            form.crf,
            ffmpeg_audio_codec(&form.audio_codec),
            form.audio_bitrate,
            form.output_container
        ),
        MediaWriteOperation::Transcode => format!(
            "{} -y -i <input> -c:v {} -c:a {}{} <output.{}>",
            config.ffmpeg_bin,
            ffmpeg_video_codec(&form.video_codec),
            ffmpeg_audio_codec(&form.audio_codec),
            if form.audio_codec == "copy" {
                String::new()
            } else {
                format!(" -b:a {}", form.audio_bitrate)
            },
            form.output_container
        ),
    }
}

#[cfg(feature = "server")]
fn operation_name(operation: MediaWriteOperation) -> &'static str {
    match operation {
        MediaWriteOperation::Compress => "compress",
        MediaWriteOperation::Transcode => "transcode",
    }
}

#[cfg(feature = "server")]
fn default_output_container(operation: MediaWriteOperation) -> &'static str {
    match operation {
        MediaWriteOperation::Compress => "mp4",
        MediaWriteOperation::Transcode => "mp4",
    }
}

#[cfg(feature = "server")]
fn default_video_codec(operation: MediaWriteOperation) -> &'static str {
    match operation {
        MediaWriteOperation::Compress => "h264",
        MediaWriteOperation::Transcode => "h264",
    }
}

#[cfg(feature = "server")]
fn default_audio_codec(operation: MediaWriteOperation) -> &'static str {
    match operation {
        MediaWriteOperation::Compress => "aac",
        MediaWriteOperation::Transcode => "aac",
    }
}

#[cfg(feature = "server")]
fn sanitize_output_container(value: &str) -> String {
    match value {
        "mp4" | "mov" | "mkv" | "webm" => value.to_string(),
        _ => "mp4".to_string(),
    }
}

#[cfg(feature = "server")]
fn sanitize_video_codec(value: &str, operation: MediaWriteOperation) -> String {
    match value {
        "h264" | "hevc" | "vp9" => value.to_string(),
        "copy" if matches!(operation, MediaWriteOperation::Transcode) => "copy".to_string(),
        _ => default_video_codec(operation).to_string(),
    }
}

#[cfg(feature = "server")]
fn sanitize_audio_codec(value: &str, operation: MediaWriteOperation) -> String {
    match value {
        "aac" | "opus" | "mp3" => value.to_string(),
        "copy" if matches!(operation, MediaWriteOperation::Transcode) => "copy".to_string(),
        _ => default_audio_codec(operation).to_string(),
    }
}

#[cfg(feature = "server")]
fn ffmpeg_video_codec(codec: &str) -> &str {
    match codec {
        "h264" => "libx264",
        "hevc" => "libx265",
        "vp9" => "libvpx-vp9",
        "copy" => "copy",
        _ => "libx264",
    }
}

#[cfg(feature = "server")]
fn ffmpeg_audio_codec(codec: &str) -> &str {
    match codec {
        "aac" => "aac",
        "opus" => "libopus",
        "mp3" => "libmp3lame",
        "copy" => "copy",
        _ => "aac",
    }
}

#[cfg(feature = "server")]
fn content_type_for_container(container: &str) -> &'static str {
    match container {
        "mov" => "video/quicktime",
        "mkv" => "video/x-matroska",
        "webm" => "video/webm",
        _ => "video/mp4",
    }
}

#[cfg(feature = "server")]
fn derive_output_file_name(file_name: &str, container: &str, operation: &str) -> String {
    let stem = Path::new(file_name)
        .file_stem()
        .and_then(|value| value.to_str())
        .filter(|value| !value.trim().is_empty())
        .unwrap_or("output");
    format!("{stem}-{operation}.{container}")
}

#[cfg(feature = "server")]
fn make_temp_media_path(temp_dir: &Path, file_name: &str, trace_id: &str) -> PathBuf {
    let extension = Path::new(file_name)
        .extension()
        .and_then(|value| value.to_str())
        .filter(|value| !value.is_empty())
        .unwrap_or("bin");
    temp_dir.join(format!(
        "media-write-input-{trace_id}-{}.{}",
        sanitize_file_component(file_name),
        extension
    ))
}

#[cfg(feature = "server")]
fn sanitize_file_component(raw: &str) -> String {
    raw.chars()
        .map(|ch| match ch {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' | '.' => ch,
            _ => '-',
        })
        .collect()
}

#[cfg(feature = "server")]
fn tail_excerpt(value: &str, max_chars: usize) -> String {
    if value.len() <= max_chars {
        value.to_string()
    } else {
        value[value.len() - max_chars..].to_string()
    }
}

#[cfg(feature = "server")]
fn bad_request_error(
    trace_id: &str,
    message: String,
) -> (axum::http::StatusCode, axum::Json<MediaWriteErrorResponse>) {
    error_response(axum::http::StatusCode::BAD_REQUEST, trace_id, message)
}

#[cfg(feature = "server")]
fn unprocessable_error(
    trace_id: &str,
    message: String,
) -> (axum::http::StatusCode, axum::Json<MediaWriteErrorResponse>) {
    error_response(
        axum::http::StatusCode::UNPROCESSABLE_ENTITY,
        trace_id,
        message,
    )
}

#[cfg(feature = "server")]
fn internal_error(
    trace_id: &str,
    message: String,
) -> (axum::http::StatusCode, axum::Json<MediaWriteErrorResponse>) {
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
) -> (axum::http::StatusCode, axum::Json<MediaWriteErrorResponse>) {
    (
        status,
        axum::Json(MediaWriteErrorResponse {
            trace_id: trace_id.to_string(),
            message,
        }),
    )
}
