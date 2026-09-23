use syn::visit::Visit;

// Detection for whether a reducer impl uses sliced updates.
//
// The `#[reducer]` macro needs to know whether it should:
// - Require `State: SlicedState`
// - Specialize the reducer impl to `Reducer<<State as SlicedState>::StateSlice>`
// - Generate snapshot wrappers that expose `slices` metadata
//
// Only clearly-qualified paths count as evidence (e.g. `StateChange::Infer`,
// `crate::...::StateChange::Slices`). A bare glob-imported `Infer`/`Slices`
// identifier is ambiguous (any enum could export such a variant), so it
// produces a compile_error! diagnostic instead of being silently interpreted.
// Migration: qualify as `StateChange::Infer` / `StateChange::Slices`.
//
// Limitations (syntactic detection only):
// - The `StateChange` segment must be spelled literally; `use ...::StateChange
//   as SC; SC::Infer` is treated as unrelated and sliced mode stays off.
// - Detection runs on every `reduce`/`effect`, including non-sliced reducers.
//   An unrelated `OtherEnum::Infer` is ignored, but a bare `Infer` from any
//   import errors even if you never intended sliced updates; qualify or rename
//   the unrelated import in that case.

pub(crate) fn fn_uses_sliced_state_change(item_fn: &syn::ImplItemFn) -> syn::Result<bool> {
    let mut detector = SlicedUsageDetector {
        found: false,
        ambiguous: None,
    };
    detector.visit_impl_item_fn(item_fn);
    match detector.ambiguous {
        Some(err) => Err(err),
        None => Ok(detector.found),
    }
}

struct SlicedUsageDetector {
    found: bool,
    ambiguous: Option<syn::Error>,
}

enum SlicedPathKind {
    /// Multi-segment path ending in `StateChange::Infer` / `StateChange::Slices`.
    Qualified,
    /// Single-segment bare `Infer` / `Slices` identifier.
    Bare,
    Unrelated,
}

fn classify_sliced_path(path: &syn::Path) -> SlicedPathKind {
    let Some(last) = path.segments.last() else {
        return SlicedPathKind::Unrelated;
    };

    let last_ident = last.ident.to_string();
    if last_ident != "Infer" && last_ident != "Slices" {
        return SlicedPathKind::Unrelated;
    }

    if path.segments.len() == 1 {
        return SlicedPathKind::Bare;
    }

    match path.segments.iter().rev().nth(1) {
        Some(prev) if prev.ident == "StateChange" => SlicedPathKind::Qualified,
        _ => SlicedPathKind::Unrelated,
    }
}

impl SlicedUsageDetector {
    fn record_path(&mut self, path: &syn::Path) {
        match classify_sliced_path(path) {
            SlicedPathKind::Qualified => self.found = true,
            SlicedPathKind::Bare => {
                if self.ambiguous.is_none() {
                    self.ambiguous = Some(syn::Error::new_spanned(
                        path,
                        "ambiguous bare `Infer`/`Slices`; qualify the path (e.g. `StateChange::Infer`) so sliced-update detection is explicit. If you did not intend sliced updates, qualify or rename the unrelated import instead. Note: `StateChange` must be spelled literally (`SC::Infer` via an alias is not detected)",
                    ));
                }
            }
            SlicedPathKind::Unrelated => {}
        }
    }
}

impl<'ast> Visit<'ast> for SlicedUsageDetector {
    fn visit_path(&mut self, node: &'ast syn::Path) {
        self.record_path(node);
        if self.ambiguous.is_none() {
            syn::visit::visit_path(self, node);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn detect(body: &str) -> syn::Result<bool> {
        let src = format!(
            "fn reduce(&mut self, _state: &mut S, _ctx: C) -> oxide_core::CoreResult<oxide_core::StateChange> {{ {body} }}"
        );
        let item_fn: syn::ImplItemFn = syn::parse_str(&src).unwrap();
        fn_uses_sliced_state_change(&item_fn)
    }

    #[test]
    fn qualified_state_change_paths_are_detected() {
        assert!(detect("let _ = StateChange::Infer;").unwrap());
        assert!(
            detect("let _ = oxide_core::StateChange::Infer;")
                .unwrap()
        );
        assert!(
            detect(
                "let _ = crate::deep::inner::StateChange::Slices(&[]);"
            )
            .unwrap()
        );
        // Pattern position (`match` arms) is detected too, including tuple-struct
        // patterns like `StateChange::Slices(s)` which parse as Pat::TupleStruct.
        assert!(
            detect("match r { StateChange::Infer => {} StateChange::None => {} }").unwrap()
        );
        assert!(
            detect("match r { StateChange::Slices(s) => { let _ = s; } _ => {} }").unwrap()
        );
        assert!(
            detect("let StateChange::Slices(s) = r;").unwrap()
        );
    }

    #[test]
    fn bare_infer_or_slices_idents_produce_diagnostic() {
        let err = detect("let _ = Infer;").unwrap_err();
        assert!(err.to_string().contains("ambiguous bare `Infer`/`Slices`"));
        assert!(err.to_string().contains("qualify the path"));

        let err = detect("let _ = Slices(&[]);").unwrap_err();
        assert!(err.to_string().contains("ambiguous bare `Infer`/`Slices`"));
    }

    #[test]
    fn unrelated_identifiers_are_ignored() {
        assert!(!detect("let infer = 1; let _ = OtherEnum::Infer;").unwrap());
        assert!(!detect("let slices = vec![1]; let _ = config.slices;").unwrap());
        assert!(!detect("Ok(StateChange::None)").unwrap());
    }
}
