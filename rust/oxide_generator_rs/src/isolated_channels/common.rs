use syn::{ImplItem, ItemImpl, Type};

pub(super) fn find_assoc_type(item_impl: &ItemImpl, assoc: &str) -> syn::Result<Type> {
    item_impl
        .items
        .iter()
        .find_map(|item| match item {
            ImplItem::Type(t) if t.ident == assoc => Some(t.ty.clone()),
            _ => None,
        })
        .ok_or_else(|| {
            syn::Error::new_spanned(item_impl, format!("missing associated type `{assoc}`"))
        })
}

pub(super) fn impl_self_ident(ty: &Type) -> syn::Result<syn::Ident> {
    match ty {
        Type::Path(p) => p
            .path
            .segments
            .last()
            .map(|s| s.ident.clone())
            .ok_or_else(|| syn::Error::new_spanned(ty, "expected concrete impl type")),
        _ => Err(syn::Error::new_spanned(ty, "expected concrete impl type")),
    }
}
