use std::path::PathBuf;

use quote::quote;
use syn::{FnArg, Item, ItemFn, ItemMod, PatType, ReturnType, Type};

use crate::routes::args::RoutesArgs;
use crate::routes::codegen::{
    generate_navigation_bridge_module, generate_navigation_module, generate_oxide_init_module,
    generate_payload_helpers, generate_route_context_types, generate_route_kind_enum,
    generate_route_payload_enum,
};
use crate::routes::discovery::{collect_routes, discover_rs_files, parse_items_from_file};
use crate::routes::metadata::emit_metadata_json;

pub fn expand_routes_module(
    args: RoutesArgs,
    item_mod: ItemMod,
) -> syn::Result<proc_macro2::TokenStream> {
    let mod_ident = item_mod.ident.clone();

    let manifest_dir =
        PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".into()));
    let crate_name = std::env::var("CARGO_PKG_NAME").unwrap_or_else(|_| "unknown".into());

    let src_dir = manifest_dir.join("src");
    let routes_dir = src_dir.join(mod_ident.to_string());

    let mut files_to_scan = Vec::new();
    if routes_dir.is_dir() {
        files_to_scan.extend(discover_rs_files(&routes_dir)?);
    }

    let module_items = if let Some((_brace, inline_items)) = &item_mod.content {
        if inline_items.is_empty() {
            let mod_rs = routes_dir.join("mod.rs");
            let routes_rs = src_dir.join(format!("{mod_ident}.rs"));
            if mod_rs.is_file() {
                parse_items_from_file(&mod_rs)?
            } else if routes_rs.is_file() {
                parse_items_from_file(&routes_rs)?
            } else {
                Vec::new()
            }
        } else {
            inline_items.iter().cloned().collect::<Vec<_>>()
        }
    } else {
        let mod_rs = routes_dir.join("mod.rs");
        let routes_rs = src_dir.join(format!("{mod_ident}.rs"));
        if mod_rs.is_file() {
            parse_items_from_file(&mod_rs)?
        } else if routes_rs.is_file() {
            parse_items_from_file(&routes_rs)?
        } else {
            Vec::new()
        }
    };

    let mut items = module_items.clone();

    for file in files_to_scan {
        items.extend(parse_items_from_file(&file)?);
    }

    let routes = collect_routes(&items)?;
    validate_hook_functions(&module_items, &args)?;
    emit_metadata_json(&crate_name, &routes, &manifest_dir)?;

    let kind_enum = generate_route_kind_enum(&routes)?;
    let payload_enum = generate_route_payload_enum(&routes)?;
    let payload_helpers = generate_payload_helpers(&routes)?;
    let route_context_types = generate_route_context_types()?;
    // pass routes list to generation functions that may need metadata
    let navigation_module = generate_navigation_module(&routes)?;
    let init_module = generate_oxide_init_module()?;
    let navigation_bridge_module = generate_navigation_bridge_module(&args)?;

    let mut out_mod = item_mod;
    if out_mod.content.is_none() {
        out_mod.semi = None;
        out_mod.content = Some((syn::token::Brace::default(), module_items.clone()));
    }

    let Some((_brace, mod_items)) = &mut out_mod.content else {
        unreachable!("checked content is_some above")
    };
    if mod_items.is_empty() && !module_items.is_empty() {
        mod_items.extend(module_items);
    }

    mod_items.push(Item::Verbatim(kind_enum));
    mod_items.push(Item::Verbatim(payload_enum));
    mod_items.push(Item::Verbatim(payload_helpers));
    mod_items.push(Item::Verbatim(route_context_types));
    mod_items.push(Item::Verbatim(navigation_bridge_module));

    Ok(quote! {
        #out_mod
        #navigation_module
        #init_module
    })
}

fn validate_hook_functions(module_items: &[Item], args: &RoutesArgs) -> syn::Result<()> {
    if let Some(init_path) = &args.init {
        let ident = init_path.segments.last().expect("validated path").ident.clone();
        let Some(item_fn) = find_fn(module_items, &ident) else {
            return Err(syn::Error::new_spanned(
                init_path,
                format!(
                    "routes hook `init = {ident}` not found in this routes module; define `fn {ident}(ctx: RouteInitContext) -> oxide_core::CoreResult<()>`"
                ),
            ));
        };
        validate_hook_signature(item_fn, "RouteInitContext", "init")?;
    }

    if let Some(on_change_path) = &args.on_route_change {
        let ident = on_change_path
            .segments
            .last()
            .expect("validated path")
            .ident
            .clone();
        let Some(item_fn) = find_fn(module_items, &ident) else {
            return Err(syn::Error::new_spanned(
                on_change_path,
                format!(
                    "routes hook `on_route_change = {ident}` not found in this routes module; define `fn {ident}(ctx: RouteUpdateContext) -> oxide_core::CoreResult<()>`"
                ),
            ));
        };
        validate_hook_signature(item_fn, "RouteUpdateContext", "on_route_change")?;
    }

    Ok(())
}

fn find_fn<'a>(module_items: &'a [Item], ident: &syn::Ident) -> Option<&'a ItemFn> {
    module_items.iter().find_map(|item| match item {
        Item::Fn(item_fn) if item_fn.sig.ident == *ident => Some(item_fn),
        _ => None,
    })
}

fn validate_hook_signature(
    item_fn: &ItemFn,
    expected_ctx_type: &str,
    hook_name: &str,
) -> syn::Result<()> {
    if item_fn.sig.asyncness.is_some() {
        return Err(syn::Error::new_spanned(
            &item_fn.sig.asyncness,
            format!(
                "routes hook `{}` must be synchronous and return oxide_core::CoreResult<()>",
                hook_name
            ),
        ));
    }
    if item_fn.sig.inputs.len() != 1 {
        return Err(syn::Error::new_spanned(
            &item_fn.sig.inputs,
            format!(
                "routes hook `{}` must take exactly one argument of type {}",
                hook_name, expected_ctx_type
            ),
        ));
    }

    let FnArg::Typed(PatType { ty, .. }) = item_fn.sig.inputs.first().expect("len checked") else {
        return Err(syn::Error::new_spanned(
            &item_fn.sig.inputs,
            format!(
                "routes hook `{}` argument must be typed as {}",
                hook_name, expected_ctx_type
            ),
        ));
    };
    if !type_last_ident_is(ty, expected_ctx_type) {
        return Err(syn::Error::new_spanned(
            ty,
            format!(
                "routes hook `{}` must accept {} as its only argument",
                hook_name, expected_ctx_type
            ),
        ));
    }

    match &item_fn.sig.output {
        ReturnType::Type(_, ty) if type_last_ident_is(ty, "CoreResult") => Ok(()),
        _ => Err(syn::Error::new_spanned(
            &item_fn.sig.output,
            format!(
                "routes hook `{}` must return oxide_core::CoreResult<()>",
                hook_name
            ),
        )),
    }
}

fn type_last_ident_is(ty: &Type, expected: &str) -> bool {
    let Type::Path(path) = ty else {
        return false;
    };
    path.path
        .segments
        .last()
        .map(|segment| segment.ident == expected)
        .unwrap_or(false)
}
