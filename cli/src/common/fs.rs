//! Atomic file writes shared by the commands that modify a project.

use std::io::Write as _;
use std::path::Path;

use anyhow::{Context, Result};

/// Write `content` to `path` atomically.
///
/// The content lands in a temp file in the same directory, is flushed to disk,
/// and is then renamed over the target. A crash or failure therefore leaves
/// either the old file or the new one, never a half-written file.
///
/// The temp name is derived from the *full* file name rather than the stem.
/// `Path::with_extension` replaces the extension, so `app.dart` and `app.rs`
/// would otherwise both map to `app.tmp-<pid>` and two writers on same-stem
/// siblings would interleave on a single temp file.
pub fn write_atomic(path: &Path, content: &str) -> Result<()> {
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;
    }

    let file_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("oxide-tmp");
    let tmp = path.with_file_name(format!(".{file_name}.{}.tmp", std::process::id()));

    // Write via a File handle so the data can be fsync'd before the rename. The
    // rename is atomic, but without the flush the rename can land before the
    // contents do, which on a power loss yields a zero-length file.
    let write_result = (|| -> std::io::Result<()> {
        let mut f = std::fs::File::create(&tmp)?;
        f.write_all(content.as_bytes())?;
        f.sync_all()?;
        Ok(())
    })();

    if let Err(e) = write_result {
        // Do not leave a partial temp file behind in the source tree.
        let _ = std::fs::remove_file(&tmp);
        return Err(e).with_context(|| format!("failed to write {}", tmp.display()));
    }

    if let Err(e) = std::fs::rename(&tmp, path) {
        let _ = std::fs::remove_file(&tmp);
        return Err(e)
            .with_context(|| format!("failed to rename {} to {}", tmp.display(), path.display()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_file_and_parent_directories() {
        let dir = tempfile::tempdir().unwrap();
        let target = dir.path().join("a/b/c/file.txt");
        write_atomic(&target, "hello").unwrap();
        assert_eq!(std::fs::read_to_string(&target).unwrap(), "hello");
    }

    #[test]
    fn overwrites_existing_file() {
        let dir = tempfile::tempdir().unwrap();
        let target = dir.path().join("f.txt");
        write_atomic(&target, "first").unwrap();
        write_atomic(&target, "second").unwrap();
        assert_eq!(std::fs::read_to_string(&target).unwrap(), "second");
    }

    #[test]
    fn leaves_no_temp_files_behind() {
        let dir = tempfile::tempdir().unwrap();
        let target = dir.path().join("f.txt");
        write_atomic(&target, "content").unwrap();
        let leftovers: Vec<String> = std::fs::read_dir(dir.path())
            .unwrap()
            .flatten()
            .map(|e| e.file_name().to_string_lossy().to_string())
            .filter(|n| n.contains(".tmp"))
            .collect();
        assert!(leftovers.is_empty(), "temp files left: {leftovers:?}");
    }

    #[test]
    fn same_stem_siblings_do_not_collide() {
        // Regression: with_extension() would map both of these to the same
        // "app.tmp-<pid>", so one writer could clobber the other's temp file.
        let dir = tempfile::tempdir().unwrap();
        let a = dir.path().join("app.dart");
        let b = dir.path().join("app.rs");
        write_atomic(&a, "dart").unwrap();
        write_atomic(&b, "rust").unwrap();
        assert_eq!(std::fs::read_to_string(&a).unwrap(), "dart");
        assert_eq!(std::fs::read_to_string(&b).unwrap(), "rust");
    }

    #[test]
    fn preserves_name_without_extension() {
        let dir = tempfile::tempdir().unwrap();
        let target = dir.path().join("LICENSE");
        write_atomic(&target, "text").unwrap();
        assert_eq!(std::fs::read_to_string(&target).unwrap(), "text");
    }
}
