use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, LitStr, Path, Token, Type};

#[derive(Default, Clone)]
pub struct RoutesArgs {
    pub init: Option<Path>,
    pub on_route_change: Option<Path>,
}

impl Parse for RoutesArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(Self::default());
        }

        let mut args = RoutesArgs::default();
        while !input.is_empty() {
            let key: Ident = input.call(Ident::parse_any)?;
            input.parse::<Token![=]>()?;

            match key.to_string().as_str() {
                "init" => {
                    if args.init.is_some() {
                        return Err(syn::Error::new_spanned(
                            key,
                            "duplicate #[routes] argument `init`",
                        ));
                    }
                    let path: Path = input.parse()?;
                    validate_hook_path(&path, "init")?;
                    args.init = Some(path);
                }
                "on_route_change" => {
                    if args.on_route_change.is_some() {
                        return Err(syn::Error::new_spanned(
                            key,
                            "duplicate #[routes] argument `on_route_change`",
                        ));
                    }
                    let path: Path = input.parse()?;
                    validate_hook_path(&path, "on_route_change")?;
                    args.on_route_change = Some(path);
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        key,
                        "unknown #[routes] argument; supported: init = hook_fn, on_route_change = hook_fn",
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

fn validate_hook_path(path: &Path, arg_name: &str) -> syn::Result<()> {
    if path.leading_colon.is_some() || path.segments.len() != 1 {
        return Err(syn::Error::new_spanned(
            path,
            format!(
                "#[routes] argument `{arg_name}` must reference a function inside the annotated routes module using a bare identifier (e.g. {arg_name} = my_hook)"
            ),
        ));
    }
    Ok(())
}

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
