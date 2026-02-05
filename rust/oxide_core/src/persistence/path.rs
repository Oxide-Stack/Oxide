use std::path::PathBuf;

// Stable persistence addressing.
//
// Why: persistence is keyed by a logical identifier, not an app-specific path,
// so multiple environments can share the same “where does my state live?” rule.
//
// How: derive a file name from the key and choose the final storage location
// based on the target (filesystem vs localStorage key).
#[cfg(feature = "persistence-json")]
const DEFAULT_PERSISTENCE_EXTENSION: &str = "json";

#[cfg(not(feature = "persistence-json"))]
const DEFAULT_PERSISTENCE_EXTENSION: &str = "bin";

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
    let file_name = format!("{key}.{DEFAULT_PERSISTENCE_EXTENSION}");
    #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
    {
        let dir = std::env::temp_dir().join("oxide");
        return dir.join(file_name);
    }

    #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
    {
        return PathBuf::from(format!("oxide/{file_name}"));
    }
}
