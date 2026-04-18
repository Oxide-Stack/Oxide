use std::path::{Path, PathBuf};

use proc_macro2::Span;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub(super) struct RouteFieldMeta {
    pub(super) name: String,
    pub(super) ty: String,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct RouteMeta {
    pub(super) kind: String,
    pub(super) rust_type: String,
    pub(super) path: Option<String>,
    pub(super) return_type: String,
    pub(super) extra_type: String,
    pub(super) fields: Vec<RouteFieldMeta>,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct RouteMetadataFile {
    pub(super) crate_name: String,
    pub(super) routes: Vec<RouteMeta>,
}

pub(super) trait SpanExt {
    fn span(&self) -> Span;
}

impl SpanExt for PathBuf {
    fn span(&self) -> Span {
        Span::call_site()
    }
}

impl SpanExt for &Path {
    fn span(&self) -> Span {
        Span::call_site()
    }
}
