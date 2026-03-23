use syn::{GenericArgument, Ident, ItemEnum, PathArguments, Type};

/// Extracts a simple identifier from a type used in an associated type position.
///
/// Accepted forms:
/// - `MyEnum`
/// - `crate::path::MyEnum`
///
/// Rejected forms include generics, references, trait objects, and qualified `Self::`.
pub fn type_to_simple_ident(ty: &Type) -> syn::Result<Ident> {
    let Type::Path(type_path) = ty else {
        return Err(syn::Error::new_spanned(
            ty,
            "expected a concrete type path (no references, no generics)",
        ));
    };
    if type_path.qself.is_some() {
        return Err(syn::Error::new_spanned(
            ty,
            "expected a concrete type path (no qualified Self types)",
        ));
    }
    let Some(seg) = type_path.path.segments.last() else {
        return Err(syn::Error::new_spanned(ty, "expected a concrete type path"));
    };
    if !matches!(seg.arguments, PathArguments::None) {
        return Err(syn::Error::new_spanned(
            ty,
            "generic arguments are not allowed on channel payload types",
        ));
    }
    Ok(seg.ident.clone())
}

/// Validates that an enum is suitable for use as an FFI-safe channel payload (best-effort).
pub fn validate_enum_payload(item_enum: &ItemEnum) -> syn::Result<()> {
    if !item_enum.generics.params.is_empty() {
        return Err(syn::Error::new_spanned(
            &item_enum.generics,
            "generics are not allowed on channel enums",
        ));
    }
    for variant in &item_enum.variants {
        match &variant.fields {
            syn::Fields::Named(fields) => {
                for field in &fields.named {
                    validate_payload_type(&field.ty)?;
                }
            }
            syn::Fields::Unnamed(fields) => {
                for field in &fields.unnamed {
                    validate_payload_type(&field.ty)?;
                }
            }
            syn::Fields::Unit => {}
        }
    }
    Ok(())
}

fn validate_payload_type(ty: &Type) -> syn::Result<()> {
    if let Some(message) = find_forbidden_type_shape(ty) {
        return Err(syn::Error::new_spanned(ty, message));
    }
    Ok(())
}

fn find_forbidden_type_shape(ty: &Type) -> Option<&'static str> {
    match ty {
        Type::Reference(_) => Some("references are not allowed in channel payloads"),
        Type::TraitObject(_) => Some("trait objects are not allowed in channel payloads"),
        Type::ImplTrait(_) => Some("impl Trait is not allowed in channel payloads"),
        Type::BareFn(_) => Some("bare function types are not allowed in channel payloads"),
        Type::Path(type_path) => {
            for seg in &type_path.path.segments {
                match &seg.arguments {
                    PathArguments::None => continue,
                    PathArguments::AngleBracketed(args) => {
                        for arg in &args.args {
                            if matches!(arg, GenericArgument::Lifetime(_)) {
                                return Some(
                                    "lifetime parameters are not allowed in channel payloads",
                                );
                            }
                            return Some(
                                "generic type arguments are not allowed in channel payloads",
                            );
                        }
                    }
                    PathArguments::Parenthesized(_) => {
                        return Some(
                            "parenthesized path arguments are not allowed in channel payloads",
                        );
                    }
                }
            }
            None
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn type_to_simple_ident_accepts_plain_and_path_types() {
        let plain: Type = syn::parse_str("DialogRequest").unwrap();
        assert_eq!(
            type_to_simple_ident(&plain).unwrap().to_string(),
            "DialogRequest"
        );

        let path: Type = syn::parse_str("crate::channels::DialogResponse").unwrap();
        assert_eq!(
            type_to_simple_ident(&path).unwrap().to_string(),
            "DialogResponse"
        );
    }

    #[test]
    fn type_to_simple_ident_rejects_non_path_and_generic_types() {
        let reference: Type = syn::parse_str("&DialogRequest").unwrap();
        let err = type_to_simple_ident(&reference).unwrap_err().to_string();
        assert!(err.contains("expected a concrete type path"));

        let generic: Type = syn::parse_str("DialogRequest<String>").unwrap();
        let err = type_to_simple_ident(&generic).unwrap_err().to_string();
        assert!(err.contains("generic arguments are not allowed"));
    }

    #[test]
    fn validate_enum_payload_accepts_unit_and_simple_fields() {
        let item_enum: ItemEnum =
            syn::parse_str("enum Events { Ping, Track { id: u64 }, Error(String) }").unwrap();
        validate_enum_payload(&item_enum).unwrap();
    }

    #[test]
    fn validate_enum_payload_rejects_generics_and_forbidden_shapes() {
        let generic_enum: ItemEnum = syn::parse_str("enum Generic<T> { V(T) }").unwrap();
        let err = validate_enum_payload(&generic_enum)
            .unwrap_err()
            .to_string();
        assert!(err.contains("generics are not allowed"));

        let ref_enum: ItemEnum = syn::parse_str("enum E { V(&'static str) }").unwrap();
        let err = validate_enum_payload(&ref_enum).unwrap_err().to_string();
        assert!(err.contains("references are not allowed"));

        let trait_obj_enum: ItemEnum = syn::parse_str("enum E { V(Box<dyn Send>) }").unwrap();
        let err = validate_enum_payload(&trait_obj_enum)
            .unwrap_err()
            .to_string();
        assert!(
            err.contains("generic type arguments are not allowed")
                || err.contains("trait objects are not allowed")
        );
    }
}
