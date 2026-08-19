use syn::{Attribute, Token};

pub(super) fn ensure_required_derives(attrs: &mut Vec<Attribute>) {
    // These are the baseline derives Oxide expects for state and actions.
    let mut required: Vec<syn::Path> = vec![
        syn::parse_quote!(Debug),
        syn::parse_quote!(Clone),
        syn::parse_quote!(PartialEq),
        syn::parse_quote!(Eq),
    ];

    if cfg!(feature = "state-persistence") {
        // When persistence is enabled, we inject serde derives using oxide_core's re-export
        // so downstream crates do not need to depend on serde directly.
        required.push(syn::parse_quote!(::oxide_core::serde::Serialize));
        required.push(syn::parse_quote!(::oxide_core::serde::Deserialize));
    }

    ensure_derive(attrs, &required);
}

fn ensure_derive(attrs: &mut Vec<Attribute>, required: &[syn::Path]) {
    // We either update the first existing `#[derive(...)]` attribute, or insert a new one.
    let mut existing: Vec<syn::Path> = Vec::new();
    let mut derive_attr_idx: Option<usize> = None;

    for (idx, attr) in attrs.iter().enumerate() {
        if !attr.path().is_ident("derive") {
            continue;
        }
        derive_attr_idx = Some(idx);
        let parsed: syn::punctuated::Punctuated<syn::Path, Token![,]> =
            match attr.parse_args_with(syn::punctuated::Punctuated::parse_terminated) {
                Ok(v) => v,
                Err(_) => continue,
            };
        existing.extend(parsed.into_iter());
        break;
    }

    for req in required {
        // Compare by last path segment so `Serialize` and `oxide_core::serde::Serialize`
        // are treated as the same derive.
        let req_last = req.segments.last().map(|s| s.ident.to_string());
        let already = existing.iter().any(|p| {
            p.segments
                .last()
                .map(|s| s.ident.to_string())
                .is_some_and(|x| Some(x) == req_last)
        });
        if !already {
            existing.push(req.clone());
        }
    }

    let new_attr: Attribute = syn::parse_quote!(#[derive(#(#existing),*)]);
    match derive_attr_idx {
        Some(idx) => attrs[idx] = new_attr,
        None => attrs.insert(0, new_attr),
    }
}

fn has_serde_crate_attr(attrs: &[Attribute]) -> bool {
    // We only inject `#[serde(crate = "oxide_core::serde")]` if the user hasn't already
    // configured serde's crate path.
    for attr in attrs {
        if !attr.path().is_ident("serde") {
            continue;
        }
        let syn::Meta::List(list) = &attr.meta else {
            continue;
        };
        let parsed: syn::punctuated::Punctuated<syn::Meta, Token![,]> =
            match list.parse_args_with(syn::punctuated::Punctuated::parse_terminated) {
                Ok(v) => v,
                Err(_) => continue,
            };
        for meta in parsed {
            if meta.path().is_ident("crate") {
                return true;
            }
        }
    }
    false
}

pub(super) fn ensure_serde_crate_attr(attrs: &mut Vec<Attribute>) {
    if has_serde_crate_attr(attrs) {
        return;
    }
    attrs.push(syn::parse_quote!(#[serde(crate = "oxide_core::serde")]));
}
