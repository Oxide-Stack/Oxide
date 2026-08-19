use std::fs;
use std::path::Path;

use crate::routes::model::{RouteMeta, RouteMetadataFile, SpanExt};

pub(super) fn emit_metadata_json(
    crate_name: &str,
    routes: &[RouteMeta],
    manifest_dir: &Path,
) -> syn::Result<()> {
    let target_dir = manifest_dir.join("target").join("oxide_routes");
    fs::create_dir_all(&target_dir)
        .map_err(|e| syn::Error::new(manifest_dir.span(), e.to_string()))?;
    let file_path = target_dir.join(format!("{crate_name}.json"));
    let json = RouteMetadataFile {
        crate_name: crate_name.to_string(),
        routes: routes.to_vec(),
    }
    .to_json();
    fs::write(&file_path, json).map_err(|e| syn::Error::new(file_path.span(), e.to_string()))?;
    Ok(())
}
