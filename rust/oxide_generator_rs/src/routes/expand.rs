use std::path::PathBuf;

use quote::quote;
use syn::{Item, ItemMod};

use crate::routes::codegen::{
    generate_navigation_bridge_module, generate_navigation_module, generate_oxide_init_module,
    generate_payload_helpers, generate_route_kind_enum, generate_route_payload_enum,
};
use crate::routes::discovery::{collect_routes, discover_rs_files, parse_items_from_file};
use crate::routes::metadata::emit_metadata_json;

pub fn expand_routes_module(item_mod: ItemMod) -> syn::Result<proc_macro2::TokenStream> {
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
    emit_metadata_json(&crate_name, &routes, &manifest_dir)?;

    let kind_enum = generate_route_kind_enum(&routes)?;
    let payload_enum = generate_route_payload_enum(&routes)?;
    let payload_helpers = generate_payload_helpers(&routes)?;
    // pass routes list to generation functions that may need metadata
    let navigation_module = generate_navigation_module(&routes)?;
    let init_module = generate_oxide_init_module()?;
    let navigation_bridge_module = generate_navigation_bridge_module()?;

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
    mod_items.push(Item::Verbatim(navigation_bridge_module));

    Ok(quote! {
        #out_mod
        #navigation_module
        #init_module
    })
}
