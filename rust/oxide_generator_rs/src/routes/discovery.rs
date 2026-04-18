use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use quote::ToTokens;
use syn::{Attribute, Item, ItemImpl, ItemStruct, Type};

use crate::routes::args::OxideRouteArgs;
use crate::routes::model::{RouteFieldMeta, RouteMeta, SpanExt};

pub(super) fn discover_rs_files(dir: &Path) -> syn::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir).map_err(|e| syn::Error::new(dir.span(), e.to_string()))? {
        let entry = entry.map_err(|e| syn::Error::new(dir.span(), e.to_string()))?;
        let path = entry.path();
        if path.file_name().and_then(|n| n.to_str()) == Some("mod.rs") {
            continue;
        }
        if path.extension().and_then(|e| e.to_str()) == Some("rs") {
            files.push(path);
        }
    }
    Ok(files)
}

pub(super) fn parse_items_from_file(path: &Path) -> syn::Result<Vec<Item>> {
    let src = fs::read_to_string(path).map_err(|e| syn::Error::new(path.span(), e.to_string()))?;
    let file = syn::parse_file(&src)?;
    Ok(file.items)
}

pub(super) fn collect_routes(items: &[Item]) -> syn::Result<Vec<RouteMeta>> {
    let mut impls_by_type: BTreeMap<String, (&ItemImpl, String, String, Option<String>)> =
        BTreeMap::new();
    let mut annotated: BTreeMap<String, (OxideRouteArgs, Vec<RouteFieldMeta>)> = BTreeMap::new();

    for item in items {
        match item {
            Item::Struct(item_struct) => {
                if let Some(attr) = find_oxide_route_attr(&item_struct.attrs) {
                    let args = parse_oxide_route_args(attr)?;
                    let ident = item_struct.ident.to_string();
                    let fields = extract_struct_fields(item_struct);
                    annotated.insert(ident, (args, fields));
                }
            }
            Item::Impl(item_impl) => {
                let Some((_, trait_path, _)) = &item_impl.trait_ else {
                    continue;
                };
                let trait_ident = trait_path.segments.last().map(|s| s.ident.to_string());
                if trait_ident.as_deref() != Some("Route") {
                    continue;
                }

                let Some(type_ident) = impl_self_ident(&item_impl.self_ty) else {
                    continue;
                };

                let (return_type, extra_type) = extract_associated_types(item_impl);
                let path = extract_path(item_impl);

                impls_by_type.insert(
                    type_ident.clone(),
                    (item_impl, return_type, extra_type, path),
                );
            }
            _ => {}
        }
    }

    let mut struct_fields: BTreeMap<String, Vec<RouteFieldMeta>> = BTreeMap::new();
    for item in items {
        let Item::Struct(item_struct) = item else {
            continue;
        };
        let ident = item_struct.ident.to_string();
        let fields = extract_struct_fields(item_struct);
        struct_fields.insert(ident, fields);
    }

    let mut routes = Vec::new();
    for (type_name, (args, fields)) in annotated {
        let kind = type_to_kind(&type_name);
        let return_type = args
            .return_type
            .map(|t| t.to_token_stream().to_string())
            .unwrap_or_else(|| "oxide_core::navigation::NoReturn".to_string());
        let extra_type = args
            .extra_type
            .map(|t| t.to_token_stream().to_string())
            .unwrap_or_else(|| "oxide_core::navigation::NoExtra".to_string());
        let path = args.path.as_ref().map(|s| s.value());
        routes.push(RouteMeta {
            kind,
            rust_type: type_name.clone(),
            path,
            return_type,
            extra_type,
            fields,
        });
        struct_fields.remove(&type_name);
    }

    for (type_name, (_impl_item, return_type, extra_type, path)) in impls_by_type {
        if routes.iter().any(|r| r.rust_type == type_name) {
            continue;
        }
        let kind = type_to_kind(&type_name);
        let fields = struct_fields.remove(&type_name).unwrap_or_default();
        routes.push(RouteMeta {
            kind,
            rust_type: type_name.clone(),
            path,
            return_type,
            extra_type,
            fields,
        });
    }

    Ok(routes)
}

pub(super) fn find_oxide_route_attr(attrs: &[Attribute]) -> Option<&Attribute> {
    attrs.iter().find(|a| {
        a.path().segments.last().map(|s| s.ident.to_string()) == Some("oxide_route".to_string())
    })
}

pub(super) fn parse_oxide_route_args(attr: &Attribute) -> syn::Result<OxideRouteArgs> {
    match &attr.meta {
        syn::Meta::Path(_) => Ok(OxideRouteArgs::default()),
        syn::Meta::List(_) => attr.parse_args(),
        syn::Meta::NameValue(_) => Err(syn::Error::new_spanned(
            attr,
            "expected attribute arguments in parentheses: #[oxide_route(...)]",
        )),
    }
}

fn impl_self_ident(ty: &Type) -> Option<String> {
    match ty {
        Type::Path(p) => p.path.segments.last().map(|s| s.ident.to_string()),
        _ => None,
    }
}

fn extract_associated_types(item_impl: &ItemImpl) -> (String, String) {
    let mut return_type = "oxide_core::navigation::NoReturn".to_string();
    let mut extra_type = "oxide_core::navigation::NoExtra".to_string();

    for it in &item_impl.items {
        let syn::ImplItem::Type(ty) = it else {
            continue;
        };
        let name = ty.ident.to_string();
        if name == "Return" {
            return_type = ty.ty.to_token_stream().to_string();
        } else if name == "Extra" {
            extra_type = ty.ty.to_token_stream().to_string();
        }
    }

    (return_type, extra_type)
}

pub(super) fn extract_path(item_impl: &ItemImpl) -> Option<String> {
    for it in &item_impl.items {
        let syn::ImplItem::Fn(f) = it else { continue };
        if f.sig.ident != "path" {
            continue;
        }
        let block = &f.block;
        if block.stmts.len() != 1 {
            continue;
        }
        let syn::Stmt::Expr(expr, _) = &block.stmts[0] else {
            continue;
        };

        match expr {
            syn::Expr::Call(call) => {
                if let syn::Expr::Path(p) = &*call.func {
                    if p.path.segments.last().map(|s| s.ident.to_string()) != Some("Some".into()) {
                        continue;
                    }
                    if call.args.len() != 1 {
                        continue;
                    }
                    if let Some(syn::Expr::Lit(syn::ExprLit {
                        lit: syn::Lit::Str(s),
                        ..
                    })) = call.args.first()
                    {
                        return Some(s.value());
                    }
                }
            }
            syn::Expr::Path(p) => {
                if p.path.segments.last().map(|s| s.ident.to_string()) == Some("None".into()) {
                    return None;
                }
            }
            _ => {}
        }
    }
    None
}

fn extract_struct_fields(item_struct: &ItemStruct) -> Vec<RouteFieldMeta> {
    match &item_struct.fields {
        syn::Fields::Named(named) => named
            .named
            .iter()
            .filter_map(|f| {
                let name = f.ident.as_ref()?.to_string();
                let ty = f.ty.to_token_stream().to_string();
                Some(RouteFieldMeta { name, ty })
            })
            .collect(),
        syn::Fields::Unnamed(_) | syn::Fields::Unit => Vec::new(),
    }
}

pub(super) fn type_to_kind(type_name: &str) -> String {
    type_name
        .strip_suffix("Route")
        .filter(|s| !s.is_empty())
        .unwrap_or(type_name)
        .to_string()
}
