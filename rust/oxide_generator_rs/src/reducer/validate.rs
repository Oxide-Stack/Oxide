use syn::{Ident, ImplItem, ItemImpl};

// Structural validation helpers for reducer impl blocks.
//
// Why: codegen should fail fast with actionable errors when the user-written
// reducer signature doesn't match what Oxide needs to generate bindings.
pub(crate) fn type_path_last_segment(ty: &syn::Type) -> Option<String> {
    match ty {
        syn::Type::Path(p) => p
            .path
            .segments
            .last()
            .map(|segment| segment.ident.to_string()),
        _ => None,
    }
}

pub(crate) fn is_reducer_trait(trait_path: &syn::Path) -> bool {
    trait_path
        .segments
        .last()
        .is_some_and(|segment| segment.ident == "Reducer")
}

pub(crate) fn impl_reducer_ident(item_impl: &ItemImpl) -> syn::Result<Ident> {
    let self_ty = &*item_impl.self_ty;
    match self_ty {
        syn::Type::Path(p) => p
            .path
            .segments
            .last()
            .map(|segment| segment.ident.clone())
            .ok_or_else(|| syn::Error::new_spanned(self_ty, "reducer type must be a path type")),
        other => Err(syn::Error::new_spanned(
            other,
            "reducer type must be a path type",
        )),
    }
}

pub(crate) fn impl_assoc_type(item_impl: &ItemImpl, name: &str) -> Option<syn::Type> {
    item_impl.items.iter().find_map(|item| match item {
        ImplItem::Type(t) if t.ident == name => Some(t.ty.clone()),
        _ => None,
    })
}

pub(crate) fn find_impl_fn<'a>(item_impl: &'a ItemImpl, name: &str) -> Option<&'a syn::ImplItemFn> {
    item_impl.items.iter().find_map(|item| match item {
        ImplItem::Fn(f) if f.sig.ident == name => Some(f),
        _ => None,
    })
}

pub(crate) fn validate_init_sig(item_fn: &syn::ImplItemFn) -> syn::Result<()> {
    if item_fn.sig.asyncness.is_none() {
        return Err(syn::Error::new_spanned(
            &item_fn.sig.fn_token,
            "`init` must be async",
        ));
    }
    if item_fn.sig.inputs.len() != 2 {
        return Err(syn::Error::new_spanned(
            &item_fn.sig.inputs,
            "`init` must take exactly 2 arguments: `&mut self` and `oxide_core::InitContext<SideEffect>`",
        ));
    }
    let mut inputs = item_fn.sig.inputs.iter();
    let first = inputs.next().expect("first arg");
    let second = inputs.next().expect("second arg");

    match first {
        syn::FnArg::Receiver(r) if r.mutability.is_some() => {}
        other => {
            return Err(syn::Error::new_spanned(
                other,
                "`init` first argument must be `&mut self`",
            ));
        }
    }

    match second {
        syn::FnArg::Typed(_) => {}
        other => {
            return Err(syn::Error::new_spanned(
                other,
                "`init` second argument must be `oxide_core::InitContext<SideEffect>`",
            ));
        }
    }

    if !matches!(item_fn.sig.output, syn::ReturnType::Default) {
        return Err(syn::Error::new_spanned(
            &item_fn.sig.output,
            "`init` must not return a value",
        ));
    }

    Ok(())
}

fn return_is_core_result_state_change(output: &syn::ReturnType) -> bool {
    match output {
        syn::ReturnType::Default => false,
        syn::ReturnType::Type(_, ty) => match &**ty {
            syn::Type::Path(p) => match p.path.segments.last() {
                Some(last_segment) if last_segment.ident == "CoreResult" => {
                    match &last_segment.arguments {
                        syn::PathArguments::AngleBracketed(args) => {
                            if args.args.len() != 1 {
                                return false;
                            }
                            match args.args.first().unwrap() {
                                syn::GenericArgument::Type(syn::Type::Path(inner)) => inner
                                    .path
                                    .segments
                                    .last()
                                    .is_some_and(|seg| seg.ident == "StateChange"),
                                _ => false,
                            }
                        }
                        _ => false,
                    }
                }
                _ => false,
            },
            _ => false,
        },
    }
}

pub(crate) fn validate_reduce_like_sig(item_fn: &syn::ImplItemFn, name: &str) -> syn::Result<()> {
    if item_fn.sig.asyncness.is_some() {
        return Err(syn::Error::new_spanned(
            &item_fn.sig.fn_token,
            format!("`{name}` must not be async"),
        ));
    }
    if item_fn.sig.inputs.len() != 3 {
        return Err(syn::Error::new_spanned(
            &item_fn.sig.inputs,
            format!(
                "`{name}` must take exactly 3 arguments: `&mut self`, `&mut State`, and the input value"
            ),
        ));
    }

    let mut inputs = item_fn.sig.inputs.iter();
    let first = inputs.next().expect("first arg");
    let second = inputs.next().expect("second arg");
    let third = inputs.next().expect("third arg");

    match first {
        syn::FnArg::Receiver(r) if r.mutability.is_some() => {}
        other => {
            return Err(syn::Error::new_spanned(
                other,
                format!("`{name}` first argument must be `&mut self`"),
            ));
        }
    }

    match second {
        syn::FnArg::Typed(pat_ty) => match &*pat_ty.ty {
            syn::Type::Reference(r) if r.mutability.is_some() => {}
            other => {
                return Err(syn::Error::new_spanned(
                    other,
                    format!("`{name}` second argument must be `&mut State`"),
                ));
            }
        },
        other => {
            return Err(syn::Error::new_spanned(
                other,
                format!("`{name}` second argument must be `&mut State`"),
            ));
        }
    }

    match third {
        syn::FnArg::Typed(_) => {}
        other => {
            return Err(syn::Error::new_spanned(
                other,
                format!("`{name}` third argument must be the input value (not `self`)"),
            ));
        }
    }

    if !return_is_core_result_state_change(&item_fn.sig.output) {
        return Err(syn::Error::new_spanned(
            &item_fn.sig.output,
            format!("`{name}` must return `oxide_core::CoreResult<oxide_core::StateChange<...>>`"),
        ));
    }

    Ok(())
}
