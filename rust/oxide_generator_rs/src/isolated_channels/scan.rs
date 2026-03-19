use std::fs;
use std::path::{Path, PathBuf};

use syn::{Item, ItemEnum};

/// Attempts to locate and parse an enum definition by name from the consuming crate sources.
///
/// This is a best-effort strategy used to provide actionable compile-time errors
/// (for example: "Request must be an enum" or "variants must not contain references").
pub fn find_enum_in_crate_src(enum_ident: &str) -> syn::Result<Option<ItemEnum>> {
    let manifest_dir =
        PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".into()));
    let cwd = std::env::current_dir().unwrap_or_else(|_| manifest_dir.clone());
    let mut roots = vec![manifest_dir, cwd];
    roots.dedup();

    for root in roots {
        let files = discover_rs_files_recursive(&root)?;
        for file in files {
            let src = match fs::read_to_string(&file) {
                Ok(src) => src,
                Err(_) => continue,
            };
            let parsed = match syn::parse_file(&src) {
                Ok(parsed) => parsed,
                Err(_) => continue,
            };

            for item in parsed.items {
                if let Item::Enum(item_enum) = item {
                    if item_enum.ident == enum_ident {
                        return Ok(Some(item_enum));
                    }
                }
            }
        }
    }

    Ok(None)
}

fn discover_rs_files_recursive(root: &Path) -> syn::Result<Vec<PathBuf>> {
    let mut out = Vec::new();
    visit_dir(root, &mut out)?;
    Ok(out)
}

fn visit_dir(dir: &Path, out: &mut Vec<PathBuf>) -> syn::Result<()> {
    for entry in fs::read_dir(dir).map_err(|e| syn::Error::new(dir.span(), e.to_string()))? {
        let entry = entry.map_err(|e| syn::Error::new(dir.span(), e.to_string()))?;
        let path = entry.path();
        if path.is_dir() {
            if path.file_name().and_then(|n| n.to_str()) == Some("target") {
                continue;
            }
            visit_dir(&path, out)?;
            continue;
        }

        if path.extension().and_then(|e| e.to_str()) == Some("rs") {
            out.push(path);
        }
    }
    Ok(())
}

trait SpanPath {
    fn span(&self) -> proc_macro2::Span;
}

impl SpanPath for &Path {
    fn span(&self) -> proc_macro2::Span {
        proc_macro2::Span::call_site()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TEST_ENV_LOCK;

    fn temp_manifest(prefix: &str) -> PathBuf {
        let mut dir = std::env::temp_dir();
        dir.push(format!(
            "{}_{}_{}",
            prefix,
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join("src")).unwrap();
        dir
    }

    #[test]
    fn discovers_enum_in_src_and_skips_target_tree() {
        let _guard = TEST_ENV_LOCK
            .get_or_init(|| std::sync::Mutex::new(()))
            .lock()
            .unwrap();

        let dir = temp_manifest("oxide_scan");
        fs::write(
            dir.join("src").join("lib.rs"),
            "pub enum DialogRequest { Confirm }",
        )
        .unwrap();
        fs::create_dir_all(dir.join("target").join("nested")).unwrap();
        fs::write(
            dir.join("target").join("nested").join("ignored.rs"),
            "pub enum ShouldNotBeScanned { V }",
        )
        .unwrap();

        unsafe { std::env::set_var("CARGO_MANIFEST_DIR", &dir) };
        let found = find_enum_in_crate_src("DialogRequest").unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().ident.to_string(), "DialogRequest");

        let missing = find_enum_in_crate_src("ShouldNotBeScanned").unwrap();
        assert!(missing.is_none());

        let _ = fs::remove_dir_all(dir);
    }
}
