use syn::parse::{Parse, ParseStream};
use syn::{Ident, LitBool, Token};

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct StateArgs {
    pub sliced: bool,
}

impl Parse for StateArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(Self::default());
        }

        let mut sliced: Option<bool> = None;
        while !input.is_empty() {
            let key: Ident = input.parse()?;
            input.parse::<Token![=]>()?;
            let value: LitBool = input.parse()?;
            match key.to_string().as_str() {
                "sliced" => sliced = Some(value.value),
                other => {
                    return Err(syn::Error::new_spanned(
                        key,
                        format!("unknown #[state] argument `{other}`"),
                    ));
                }
            }

            if input.peek(Token![,]) {
                let _ = input.parse::<Token![,]>()?;
            }
        }

        Ok(Self {
            sliced: sliced.unwrap_or(false),
        })
    }
}
