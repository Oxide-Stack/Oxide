use syn::parse::{Parse, ParseStream};
use syn::{Ident, Token};

// Attribute argument parsing for `#[reducer(...)]`.
//
// Why: a dedicated parse layer keeps the mini-language stable and isolates
// syntactic concerns from validation and token emission.
pub(crate) struct ReducerArgs {
    pub(crate) engine_ident: Ident,
    pub(crate) snapshot_ident: Ident,
    pub(crate) initial_state: syn::Expr,
    pub(crate) reducer_expr: Option<syn::Expr>,
    pub(crate) include_frb: bool,
    pub(crate) persist_key: Option<syn::LitStr>,
    pub(crate) persist_min_interval_ms: Option<u64>,
}

impl Parse for ReducerArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut engine_ident: Option<Ident> = None;
        let mut snapshot_ident: Option<Ident> = None;
        let mut initial_state: Option<syn::Expr> = None;
        let mut reducer_expr: Option<syn::Expr> = None;
        let mut include_frb = true;
        let mut persist_key: Option<syn::LitStr> = None;
        let mut persist_min_interval_ms: Option<u64> = None;

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            let key_string = key.to_string();

            match key_string.as_str() {
                "engine" => {
                    input.parse::<Token![=]>()?;
                    engine_ident = Some(input.parse()?);
                }
                "snapshot" => {
                    input.parse::<Token![=]>()?;
                    snapshot_ident = Some(input.parse()?);
                }
                "initial" => {
                    input.parse::<Token![=]>()?;
                    initial_state = Some(input.parse()?);
                }
                "reducer" => {
                    input.parse::<Token![=]>()?;
                    reducer_expr = Some(input.parse()?);
                }
                "no_frb" => {
                    include_frb = false;
                }
                "persist" => {
                    input.parse::<Token![=]>()?;
                    persist_key = Some(input.parse()?);
                }
                "persist_min_interval_ms" => {
                    input.parse::<Token![=]>()?;
                    let lit: syn::LitInt = input.parse()?;
                    persist_min_interval_ms = Some(lit.base10_parse()?);
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        key,
                        "unknown #[reducer] argument (expected `engine = Name`, `snapshot = Name`, `initial = <expr>`, optional `reducer = <expr>`, optional `no_frb`, optional `persist = \"key\"`, optional `persist_min_interval_ms = 200`)",
                    ));
                }
            }

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(Self {
            engine_ident: engine_ident.ok_or_else(|| {
                syn::Error::new(input.span(), "#[reducer] is missing `engine = <Name>`")
            })?,
            snapshot_ident: snapshot_ident.ok_or_else(|| {
                syn::Error::new(input.span(), "#[reducer] is missing `snapshot = <Name>`")
            })?,
            initial_state: initial_state.ok_or_else(|| {
                syn::Error::new(input.span(), "#[reducer] is missing `initial = <expr>`")
            })?,
            reducer_expr,
            include_frb,
            persist_key,
            persist_min_interval_ms,
        })
    }
}
