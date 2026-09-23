use quote::{format_ident, quote};
use syn::{ImplItem, ItemImpl};

use crate::reducer::sliced_usage::fn_uses_sliced_state_change;
use crate::reducer::validate::{
    find_impl_fn, impl_assoc_type, impl_reducer_ident, is_reducer_trait, validate_init_sig,
    validate_reduce_like_sig,
};

pub(super) struct ReducerAnalysis {
    pub(super) reducer_ident: syn::Ident,
    pub(super) state_ty: syn::Type,
    pub(super) action_ty: syn::Type,
    pub(super) uses_sliced_updates: bool,
    pub(super) state_slice_ty: Option<syn::Type>,
    pub(super) sliced_state_assert: proc_macro2::TokenStream,
}

pub(super) fn analyze_reducer_impl(item_impl: &mut ItemImpl) -> syn::Result<ReducerAnalysis> {
    let Some((_, trait_path, _)) = &item_impl.trait_ else {
        return Err(syn::Error::new_spanned(
            &item_impl.impl_token,
            "#[reducer(...)] must be applied to an `impl oxide_core::Reducer for <Type>` block",
        ));
    };
    if !is_reducer_trait(trait_path) {
        return Err(syn::Error::new_spanned(
            trait_path,
            "#[reducer(...)] must be applied to an `impl oxide_core::Reducer for <Type>` block",
        ));
    }

    let reducer_ident = impl_reducer_ident(item_impl)?;
    let state_ty = required_assoc_type(item_impl, "State")?;
    let action_ty = required_assoc_type(item_impl, "Action")?;
    let _sideeffect_ty = required_assoc_type(item_impl, "SideEffect")?;

    let init_fn = find_impl_fn(item_impl, "init").ok_or_else(|| {
        syn::Error::new_spanned(
            &item_impl.self_ty,
            "Reducer impl is missing `init` (expected `async fn init(&mut self, ctx: oxide_core::InitContext<Self::SideEffect>)` or `fn init(&mut self, ctx: oxide_core::InitContext<Self::SideEffect>) -> impl Future<Output = ()> + Send`)",
        )
    })?;
    validate_init_sig(init_fn)?;

    let reduce_fn = find_impl_fn(item_impl, "reduce").ok_or_else(|| {
        syn::Error::new_spanned(
            &item_impl.self_ty,
            "Reducer impl is missing `fn reduce(&mut self, state: &mut Self::State, ctx: oxide_core::ReducerCtx<'_, Self::Action, Self::State, ...>) -> CoreResult<StateChange<...>>`",
        )
    })?;
    validate_reduce_like_sig(reduce_fn, "reduce")?;

    let effect_fn = find_impl_fn(item_impl, "effect").ok_or_else(|| {
        syn::Error::new_spanned(
            &item_impl.self_ty,
            "Reducer impl is missing `fn effect(&mut self, state: &mut Self::State, ctx: oxide_core::ReducerCtx<'_, Self::SideEffect, Self::State, ...>) -> CoreResult<StateChange<...>>`",
        )
    })?;
    validate_reduce_like_sig(effect_fn, "effect")?;

    let reduce_uses_sliced = fn_uses_sliced_state_change(reduce_fn)?;
    let effect_uses_sliced = fn_uses_sliced_state_change(effect_fn)?;
    let uses_sliced_updates = reduce_uses_sliced || effect_uses_sliced;

    let sliced_state_assert = if uses_sliced_updates {
        quote! {
            const _: () = {
                assert!(#state_ty::__OXIDE_SLICED_STATE);
            };
        }
    } else {
        quote!()
    };

    let state_slice_ty = infer_state_slice_type(&state_ty, uses_sliced_updates)?;

    if let (true, Some(state_slice_ty)) = (uses_sliced_updates, state_slice_ty.as_ref()) {
        let Some((_, trait_path, _)) = item_impl.trait_.as_mut() else {
            return Err(syn::Error::new_spanned(
                &item_impl.impl_token,
                "#[reducer(...)] must be applied to an `impl oxide_core::Reducer for <Type>` block",
            ));
        };
        specialize_reducer_trait_generic(trait_path, state_slice_ty)?;
    }

    let has_infer_slices = item_impl.items.iter().any(|item| match item {
        ImplItem::Fn(f) if f.sig.ident == "infer_slices" => true,
        _ => false,
    });
    if let (true, Some(state_slice_ty)) = (uses_sliced_updates, state_slice_ty.as_ref()) {
        if !has_infer_slices {
            let body: syn::Expr = syn::parse_quote!(Self::State::infer_slices_impl(before, after));
            item_impl.items.push(syn::parse_quote!(
                fn infer_slices(
                    &self,
                    before: &Self::State,
                    after: &Self::State,
                ) -> ::std::vec::Vec<#state_slice_ty> {
                    #body
                }
            ));
        }
    }

    if let (true, Some(state_slice_ty)) = (uses_sliced_updates, state_slice_ty.as_ref()) {
        for item in item_impl.items.iter_mut() {
            let ImplItem::Fn(f) = item else {
                continue;
            };
            if f.sig.ident != "reduce" && f.sig.ident != "effect" {
                continue;
            }
            f.sig.output = syn::parse_quote!(
                -> ::oxide_core::CoreResult<::oxide_core::StateChange<#state_slice_ty>>
            );
        }
    }

    Ok(ReducerAnalysis {
        reducer_ident,
        state_ty,
        action_ty,
        uses_sliced_updates,
        state_slice_ty,
        sliced_state_assert,
    })
}

fn required_assoc_type(item_impl: &ItemImpl, assoc_name: &str) -> syn::Result<syn::Type> {
    impl_assoc_type(item_impl, assoc_name).ok_or_else(|| {
        syn::Error::new_spanned(
            &item_impl.self_ty,
            format!("Reducer impl is missing `type {assoc_name} = ...;`"),
        )
    })
}

fn infer_state_slice_type(
    state_ty: &syn::Type,
    uses_sliced_updates: bool,
) -> syn::Result<Option<syn::Type>> {
    if !uses_sliced_updates {
        return Ok(None);
    }

    let syn::Type::Path(state_path) = state_ty else {
        return Err(syn::Error::new_spanned(
            state_ty,
            "state type must be a path type to enable sliced updates",
        ));
    };
    let mut slice_path = state_path.clone();
    if let Some(last) = slice_path.path.segments.last_mut() {
        last.ident = format_ident!("{}Slice", last.ident);
        last.arguments = syn::PathArguments::None;
    }
    Ok(Some(syn::Type::Path(slice_path)))
}

/// Specializes the impl's `Reducer<...>` generic to the detected slice type.
///
/// Only elided (`Reducer`) or wildcard (`Reducer<_>`) generics are rewritten. An
/// explicit generic is kept as-is when it already matches the slice type and
/// otherwise produces a compile_error naming both types instead of being
/// silently overwritten.
fn specialize_reducer_trait_generic(
    trait_path: &mut syn::Path,
    state_slice_ty: &syn::Type,
) -> syn::Result<()> {
    enum Plan {
        Rewrite,
        AlreadyCorrect,
    }

    let Some(last) = trait_path.segments.last_mut() else {
        return Ok(());
    };

    let plan = match &last.arguments {
        // Elided: `impl Reducer for MyReducer`.
        syn::PathArguments::None => Plan::Rewrite,
        syn::PathArguments::AngleBracketed(args) => match explicit_single_type_arg(args) {
            // Wildcard: `impl Reducer<_> for MyReducer`.
            Some(syn::Type::Infer(_)) => Plan::Rewrite,
            // Already exactly what we would have written. Compare by final
            // segment ident so qualified spellings (`crate::MyStateSlice`) and
            // equivalent paths match instead of false-positiving on text.
            Some(ty) if types_match_by_final_segment(ty, state_slice_ty) => {
                Plan::AlreadyCorrect
            }
            Some(ty) => {
                return Err(syn::Error::new_spanned(
                    ty,
                    format!(
                        "reducer declares an explicit `Reducer<{explicit}>` but sliced updates imply `Reducer<{slice}>`; elide the trait generic or write `Reducer<{slice}>` so they agree",
                        explicit = quote!(#ty),
                        slice = quote!(#state_slice_ty),
                    ),
                ));
            }
            None => {
                return Err(syn::Error::new_spanned(
                    args,
                    "#[reducer] could not interpret the `Reducer<...>` trait generic for sliced updates; elide it, use `Reducer<_>`, or pass the slice type directly",
                ));
            }
        },
        syn::PathArguments::Parenthesized(args) => {
            return Err(syn::Error::new_spanned(
                args,
                "`Reducer<...>` must not use parenthesized (Fn-style) generics",
            ));
        }
    };

    if let Plan::Rewrite = plan {
        let args: syn::AngleBracketedGenericArguments = syn::parse_quote!(<#state_slice_ty>);
        last.arguments = syn::PathArguments::AngleBracketed(args);
    }
    Ok(())
}

fn explicit_single_type_arg(args: &syn::AngleBracketedGenericArguments) -> Option<&syn::Type> {
    if args.args.len() != 1 {
        return None;
    }
    match args.args.first() {
        Some(syn::GenericArgument::Type(ty)) => Some(ty),
        _ => None,
    }
}

/// Compares two types by their final path segment ident, so
/// `MyStateSlice`, `crate::MyStateSlice`, and `super::MyStateSlice` agree.
/// Falls back to token-string equality for non-path types.
fn types_match_by_final_segment(a: &syn::Type, b: &syn::Type) -> bool {
    match (a, b) {
        (syn::Type::Path(a_path), syn::Type::Path(b_path)) => {
            let a_last = a_path.path.segments.last().map(|s| s.ident.to_string());
            let b_last = b_path.path.segments.last().map(|s| s.ident.to_string());
            match (a_last, b_last) {
                (Some(a_ident), Some(b_ident)) => a_ident == b_ident,
                _ => quote!(#a).to_string() == quote!(#b).to_string(),
            }
        }
        _ => quote!(#a).to_string() == quote!(#b).to_string(),
    }
}
