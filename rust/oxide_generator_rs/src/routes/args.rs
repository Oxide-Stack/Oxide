use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, LitStr, Token, Type};

#[derive(Default)]
pub struct OxideRouteArgs {
    pub(super) path: Option<LitStr>,
    pub(super) return_type: Option<Type>,
    pub(super) extra_type: Option<Type>,
}

impl Parse for OxideRouteArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut args = OxideRouteArgs::default();
        while !input.is_empty() {
            let key: Ident = input.call(Ident::parse_any)?;
            input.parse::<Token![=]>()?;
            match key.to_string().as_str() {
                "path" => {
                    args.path = Some(input.parse()?);
                }
                "return" => {
                    args.return_type = Some(input.parse()?);
                }
                "extra" => {
                    args.extra_type = Some(input.parse()?);
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        key,
                        "unknown #[oxide_route] argument",
                    ));
                }
            }

            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
        }
        Ok(args)
    }
}

#[derive(Default)]
pub(super) struct RouteFieldArgs {
    pub(super) kind: Option<String>,
    pub(super) key: Option<LitStr>,
}

impl Parse for RouteFieldArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut args = RouteFieldArgs::default();
        while !input.is_empty() {
            let key: Ident = input.parse()?;
            input.parse::<Token![=]>()?;
            if key == "kind" {
                let v: LitStr = input.parse()?;
                args.kind = Some(v.value());
            } else if key == "key" {
                args.key = Some(input.parse()?);
            } else {
                return Err(syn::Error::new_spanned(key, "unknown #[route] argument"));
            }
            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
        }
        Ok(args)
    }
}
