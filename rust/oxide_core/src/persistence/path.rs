use std::path::PathBuf;

// Stable persistence addressing.
//
// persistence is keyed by a logical identifier, not an app-specific path,
// so multiple environments can share the same “where does my state live?” rule.
//
// derive a file name from the key and choose the final storage location
// based on the target (filesystem vs localStorage key).
const DEFAULT_PERSISTENCE_EXTENSION: &str = "bin";
const DEBUG_PERSISTENCE_EXTENSION: &str = "json";

/// Returns the default file path for persisted state.
///
/// The returned path is located under the process temporary directory and uses
/// an extension based on the selected persistence codec (`.json` or `.bin`).
///
/// On web targets, the returned path is not used as an actual filesystem path.
/// Instead, it is converted to a string and used as the storage key for
/// `window.localStorage`.
///
/// # Returns
/// A fully-qualified file path.
pub fn default_persistence_path(key: &str) -> PathBuf {
    persistence_path_with_extension(key, DEFAULT_PERSISTENCE_EXTENSION)
}

/// Returns the default debug JSON path for persisted state.
///
/// The returned path mirrors [`default_persistence_path`] but uses a `.json`
/// extension to store a human-readable copy of the persisted snapshot.
pub fn default_persistence_debug_json_path(key: &str) -> PathBuf {
    persistence_path_with_extension(key, DEBUG_PERSISTENCE_EXTENSION)
}

fn persistence_path_with_extension(key: &str, extension: &str) -> PathBuf {
    let file_name = format!("{key}.{extension}");
    #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
    {
        let dir = std::env::temp_dir().join("oxide");
        let _ = std::fs::create_dir_all(&dir);
        return dir.join(file_name);
    }

    #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
    {
        return PathBuf::from(format!("oxide/{file_name}"));
    }
}
