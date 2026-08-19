use std::path::{Path, PathBuf};

use proc_macro2::Span;
use std::fmt::Write as _;

#[derive(Debug, Clone)]
pub(super) struct RouteFieldMeta {
    pub(super) name: String,
    pub(super) ty: String,
}

#[derive(Debug, Clone)]
pub(super) struct RouteMeta {
    pub(super) kind: String,
    pub(super) rust_type: String,
    pub(super) path: Option<String>,
    pub(super) return_type: String,
    pub(super) extra_type: String,
    pub(super) fields: Vec<RouteFieldMeta>,
}

#[derive(Debug, Clone)]
pub(super) struct RouteMetadataFile {
    pub(super) crate_name: String,
    pub(super) routes: Vec<RouteMeta>,
}

fn push_json_string(out: &mut String, value: &str) {
    out.push('"');
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{08}' => out.push_str("\\b"),
            '\u{0C}' => out.push_str("\\f"),
            c if c <= '\u{1F}' => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out.push('"');
}

fn write_route_field_json(field: &RouteFieldMeta, out: &mut String) {
    out.push('{');
    push_json_string(out, "name");
    out.push(':');
    push_json_string(out, &field.name);
    out.push(',');
    push_json_string(out, "ty");
    out.push(':');
    push_json_string(out, &field.ty);
    out.push('}');
}

fn write_route_json(route: &RouteMeta, out: &mut String) {
    out.push('{');
    push_json_string(out, "kind");
    out.push(':');
    push_json_string(out, &route.kind);
    out.push(',');
    push_json_string(out, "rust_type");
    out.push(':');
    push_json_string(out, &route.rust_type);
    out.push(',');
    push_json_string(out, "path");
    out.push(':');
    match &route.path {
        Some(path) => push_json_string(out, path),
        None => out.push_str("null"),
    }
    out.push(',');
    push_json_string(out, "return_type");
    out.push(':');
    push_json_string(out, &route.return_type);
    out.push(',');
    push_json_string(out, "extra_type");
    out.push(':');
    push_json_string(out, &route.extra_type);
    out.push(',');
    push_json_string(out, "fields");
    out.push(':');
    out.push('[');
    for (idx, field) in route.fields.iter().enumerate() {
        if idx > 0 {
            out.push(',');
        }
        write_route_field_json(field, out);
    }
    out.push(']');
    out.push('}');
}

impl RouteMetadataFile {
    pub(super) fn to_json(&self) -> String {
        let mut out = String::new();
        out.push('{');
        push_json_string(&mut out, "crate_name");
        out.push(':');
        push_json_string(&mut out, &self.crate_name);
        out.push(',');
        push_json_string(&mut out, "routes");
        out.push(':');
        out.push('[');
        for (idx, route) in self.routes.iter().enumerate() {
            if idx > 0 {
                out.push(',');
            }
            write_route_json(route, &mut out);
        }
        out.push(']');
        out.push('}');
        out
    }
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
