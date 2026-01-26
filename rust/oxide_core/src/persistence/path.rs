use std::path::PathBuf;

#[cfg(feature = "persistence-json")]
const DEFAULT_PERSISTENCE_EXTENSION: &str = "json";

#[cfg(not(feature = "persistence-json"))]
const DEFAULT_PERSISTENCE_EXTENSION: &str = "bin";

/// Returns the default file path for persisted state.
///
/// The returned path is located under the process temporary directory and uses
/// an extension based on the selected persistence codec (`.json` or `.bin`).
///
/// # Returns
/// A fully-qualified file path.
pub fn default_persistence_path(key: &str) -> PathBuf {
    let file_name = format!("{key}.{DEFAULT_PERSISTENCE_EXTENSION}");
    let dir = std::env::temp_dir().join("oxide");
    dir.join(file_name)
}
