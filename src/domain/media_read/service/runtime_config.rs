#[cfg(feature = "server")]
use std::path::PathBuf;

#[cfg(feature = "server")]
#[derive(Clone, Debug)]
pub(crate) struct MediaReadRuntimeConfig {
    pub(crate) max_upload_bytes: usize,
    pub(crate) ffprobe_timeout_secs: u64,
    pub(crate) temp_dir: PathBuf,
    pub(crate) ffprobe_bin: String,
    pub(crate) ffmpeg_bin: String,
}

#[cfg(feature = "server")]
impl MediaReadRuntimeConfig {
    pub(crate) fn from_env() -> Self {
        Self {
            max_upload_bytes: read_usize_env("MEDIA_INSPECTOR_MAX_UPLOAD_BYTES")
                .unwrap_or(1024 * 1024 * 1024),
            ffprobe_timeout_secs: read_u64_env("MEDIA_INSPECTOR_FFPROBE_TIMEOUT_SECS")
                .unwrap_or(120),
            temp_dir: std::env::var("MEDIA_INSPECTOR_TEMP_DIR")
                .ok()
                .filter(|value| !value.trim().is_empty())
                .map(PathBuf::from)
                .unwrap_or_else(std::env::temp_dir),
            ffprobe_bin: std::env::var("MEDIA_INSPECTOR_FFPROBE_BIN")
                .ok()
                .filter(|value| !value.trim().is_empty())
                .unwrap_or_else(|| "ffprobe".into()),
            ffmpeg_bin: std::env::var("MEDIA_INSPECTOR_FFMPEG_BIN")
                .ok()
                .filter(|value| !value.trim().is_empty())
                .unwrap_or_else(|| "ffmpeg".into()),
        }
    }
}

#[cfg(feature = "server")]
fn read_usize_env(key: &str) -> Option<usize> {
    std::env::var(key)
        .ok()?
        .parse::<usize>()
        .ok()
        .filter(|value| *value > 0)
}

#[cfg(feature = "server")]
fn read_u64_env(key: &str) -> Option<u64> {
    std::env::var(key)
        .ok()?
        .parse::<u64>()
        .ok()
        .filter(|value| *value > 0)
}
