// Reducer macro implementation boundary.
//
// Why: keep parsing, validation, and token emission separate so changes to one
// concern are less likely to introduce subtle codegen regressions.
mod args;
mod expand;
mod validate;

pub(crate) use args::ReducerArgs;
pub(crate) use expand::expand_reducer_impl;
