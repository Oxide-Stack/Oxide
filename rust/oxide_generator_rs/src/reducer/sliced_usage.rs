use syn::visit::Visit;

// Detection for whether a reducer impl uses sliced updates.
//
// Why: The `#[reducer]` macro needs to know whether it should:
// - Require `State: SlicedState`
// - Specialize the reducer impl to `Reducer<<State as SlicedState>::StateSlice>`
// - Generate snapshot wrappers that expose `slices` metadata
//
// How: Walk the function AST and look for references to `Infer`/`Slices` either
// as `StateChange::Infer` / `StateChange::Slices` or via glob imports like
// `use StateChange::*; Ok(Infer)`.

pub(crate) fn fn_uses_sliced_state_change(item_fn: &syn::ImplItemFn) -> bool {
    let mut detector = SlicedUsageDetector { found: false };
    detector.visit_impl_item_fn(item_fn);
    detector.found
}

struct SlicedUsageDetector {
    found: bool,
}

impl<'ast> Visit<'ast> for SlicedUsageDetector {
    fn visit_expr_path(&mut self, node: &'ast syn::ExprPath) {
        if path_looks_like_sliced_state_change(&node.path) {
            self.found = true;
            return;
        }
        syn::visit::visit_expr_path(self, node);
    }

    fn visit_pat(&mut self, node: &'ast syn::Pat) {
        if let syn::Pat::Path(path) = node {
            if path_looks_like_sliced_state_change(&path.path) {
                self.found = true;
                return;
            }
        }
        syn::visit::visit_pat(self, node);
    }
}

fn path_looks_like_sliced_state_change(path: &syn::Path) -> bool {
    let Some(last) = path.segments.last() else {
        return false;
    };

    let last_ident = last.ident.to_string();
    if last_ident != "Infer" && last_ident != "Slices" {
        return false;
    }

    // Accept both `StateChange::Infer` (or `oxide_core::StateChange::Infer`) and
    // glob-import usage like `Infer`.
    if path.segments.len() == 1 {
        return true;
    }

    let Some(prev) = path.segments.iter().rev().nth(1) else {
        return false;
    };
    prev.ident == "StateChange"
}
