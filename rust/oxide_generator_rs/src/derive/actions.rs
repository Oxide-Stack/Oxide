use quote::quote;
use syn::ItemEnum;

use crate::derive::common::{ensure_required_derives, ensure_serde_crate_attr};
use crate::meta::{ActionsMeta, collect_doc_lines, enum_variants, push_meta_doc};

pub(crate) fn expand_actions_enum(mut item: ItemEnum) -> proc_macro2::TokenStream {
    let name = item.ident.to_string();
    let ident = item.ident.clone();
    ensure_required_derives(&mut item.attrs);
    if cfg!(feature = "state-persistence") {
        ensure_serde_crate_attr(&mut item.attrs);
    }
    let docs = collect_doc_lines(&item.attrs);
    let variants = enum_variants(&item);
    item.attrs.push(syn::parse_quote!(#[doc = "oxide:actions"]));
    push_meta_doc(
        &mut item.attrs,
        &ActionsMeta {
            kind: "actions",
            name,
            docs,
            variants,
        },
    );
    let persistence_bounds = if cfg!(feature = "state-persistence") {
        quote!(+ ::oxide_core::serde::Serialize + for<'de> ::oxide_core::serde::Deserialize<'de>)
    } else {
        quote!()
    };

    quote!(
        #item
        const _: () = {
            fn _oxide_require_actions_traits<T>()
            where
                T: ::core::fmt::Debug + Clone + PartialEq + Eq #persistence_bounds,
            {
            }
            let _ = _oxide_require_actions_traits::<#ident>;
        };
    )
}
