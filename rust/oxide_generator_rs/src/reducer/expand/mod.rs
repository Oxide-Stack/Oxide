mod analysis;
mod emit;

use quote::quote;
use syn::ItemImpl;

use crate::reducer::args::ReducerArgs;
use crate::reducer::expand::analysis::analyze_reducer_impl;
use crate::reducer::expand::emit::{EmitArgs, emit_reducer_tokens};

pub(crate) fn expand_reducer_impl(
    args: ReducerArgs,
    mut item_impl: ItemImpl,
) -> proc_macro2::TokenStream {
    let ReducerArgs {
        engine_ident,
        snapshot_ident,
        initial_state,
        reducer_expr,
        include_frb,
        persist_key,
        persist_min_interval_ms,
    } = args;

    let analysis = match analyze_reducer_impl(&mut item_impl) {
        Ok(v) => v,
        Err(e) => return e.to_compile_error(),
    };

    if persist_key.is_some() && !cfg!(feature = "state-persistence") {
        return quote! {
            compile_error!("oxide_generator_rs: reducer persistence requires enabling the `state-persistence` feature on oxide_generator_rs and oxide_core");
        };
    }

    emit_reducer_tokens(
        item_impl,
        analysis,
        EmitArgs {
            engine_ident,
            snapshot_ident,
            initial_state,
            reducer_expr,
            include_frb,
            persist_key,
            persist_min_interval_ms,
        },
    )
}
