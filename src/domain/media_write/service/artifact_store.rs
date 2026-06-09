#[cfg(feature = "server")]
use std::collections::HashMap;
#[cfg(feature = "server")]
use std::path::PathBuf;
#[cfg(feature = "server")]
use std::sync::{LazyLock, Mutex};

#[cfg(feature = "server")]
#[derive(Clone, Debug)]
pub(crate) struct MediaWriteArtifact {
    pub(crate) path: PathBuf,
    pub(crate) file_name: String,
    pub(crate) content_type: String,
}

#[cfg(feature = "server")]
static MEDIA_WRITE_ARTIFACTS: LazyLock<Mutex<HashMap<String, MediaWriteArtifact>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

#[cfg(feature = "server")]
pub(crate) fn insert_artifact(artifact_id: String, artifact: MediaWriteArtifact) {
    if let Ok(mut store) = MEDIA_WRITE_ARTIFACTS.lock() {
        store.insert(artifact_id, artifact);
    }
}

#[cfg(feature = "server")]
pub(crate) fn get_artifact(artifact_id: &str) -> Option<MediaWriteArtifact> {
    MEDIA_WRITE_ARTIFACTS
        .lock()
        .ok()
        .and_then(|store| store.get(artifact_id).cloned())
}
