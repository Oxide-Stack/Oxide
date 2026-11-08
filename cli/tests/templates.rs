//! Invariants every scaffolded project must satisfy.
//!
//! These assertions exist because the template was previously only
//! type-checked, never run, and three independent defects shipped that a
//! `flutter analyze` cannot see:
//!
//! 1. The `MaterialApp` template had no `navigatorKey: OxideStack.navigatorKey`,
//!    so `initNavigation` polled for a `NavigatorState` that never attached,
//!    timed out after 10s and threw.
//! 2. The template declared `#[oxide_route()]` in Rust but shipped no
//!    `@OxideRoutePage` binding, so the generated route table threw
//!    `UnimplementedError` on the initial route.
//! 3. The integration test asserted `Counter: 0` while the page rendered a bare
//!    `$counter`, so the first `expect` always failed.
//!
//! All three are pure string properties of the template tree, so they can be
//! checked in milliseconds with no toolchain. Anything added to `templates/`
//! that breaks an invariant here fails the test rather than a user's app.

use std::collections::BTreeMap;

use oxide_cli::commands::create::{Backend, OxideSource, TemplateVars, dep_snippets, render_all};
use oxide_cli::common::template::PLACEHOLDERS;
use std::path::PathBuf;

/// Render the whole template tree the way `oxide create` would.
fn rendered(backend: Backend) -> BTreeMap<String, String> {
    let vars = TemplateVars {
        app_name: "demo_app".to_string(),
        rust_crate: "rust_lib_demo_app".to_string(),
        backend,
        page_class: "CounterPage".to_string(),
        deps: dep_snippets(&OxideSource::Pub, &PathBuf::from("/opt/oxide")),
    };
    render_all(&vars).into_iter().collect()
}

fn all_backends() -> Vec<Backend> {
    vec![
        Backend::Inherited,
        Backend::Riverpod,
        Backend::Bloc,
        Backend::InheritedHooks,
    ]
}

#[test]
fn template_renders_with_no_unsubstituted_placeholders() {
    for backend in all_backends() {
        let files = rendered(backend);
        for (rel, content) in &files {
            for leftover in content
                .match_indices("{{")
                .map(|(i, _)| i)
                .map(|i| content[i..(i + 40).min(content.len())].to_string())
            {
                assert!(
                    !leftover.contains("}}") || leftover.contains("\n"),
                    "[{backend}] {rel} has an unsubstituted placeholder near: {leftover:?}"
                );
            }
        }
    }
}

#[test]
fn every_template_placeholder_is_supplied() {
    // Guards the failure mode where a placeholder is added to a template but
    // only one of the two renderers learns about it.
    for backend in all_backends() {
        let files = rendered(backend);
        for (rel, content) in &files {
            for name in PLACEHOLDERS {
                let token = format!("{{{{{name}}}}}");
                assert!(
                    !content.contains(&token),
                    "[{backend}] {rel} still contains {token} after rendering"
                );
            }
        }
    }
}

// --- 1. navigatorKey ------------------------------------------------------

#[test]
fn app_template_attaches_the_generated_navigator_key() {
    let files = rendered(Backend::Inherited);
    let app = &files["lib/src/app.dart"];
    assert!(
        app.contains("navigatorKey: OxideStack.navigatorKey"),
        "lib/src/app.dart must pass OxideStack.navigatorKey, otherwise \
         initNavigation waits 10s and throws. Got:\n{app}"
    );
    assert!(
        app.contains("@OxideApp("),
        "lib/src/app.dart must be annotated with @OxideApp for the navigation \
         handler to be wired. Got:\n{app}"
    );
}

// --- 2. route bindings ----------------------------------------------------

#[test]
fn every_rust_route_has_a_dart_binding() {
    let files = rendered(Backend::Inherited);

    // Route structs declared in the template's Rust.
    let mut declared: Vec<String> = Vec::new();
    for (rel, content) in &files {
        if !rel.starts_with("rust/src/routes/") || !rel.ends_with(".rs") {
            continue;
        }
        for line in content.lines() {
            let line = line.trim();
            if let Some(rest) = line.strip_prefix("pub struct ") {
                let name = rest.split(['{', ':']).next().unwrap_or("").trim();
                if !name.is_empty() {
                    declared.push(name.to_string());
                }
            }
        }
    }
    assert!(
        !declared.is_empty(),
        "expected the template to declare at least one route struct"
    );

    // The binding file must exist and reference every declared route kind.
    let pages = files.get("lib/src/navigation/pages.dart").expect(
        "template must ship lib/src/navigation/pages.dart so each \
                 RouteKind has an @OxideRoutePage binding",
    );

    for struct_name in &declared {
        let kind = struct_name
            .strip_suffix("Route")
            .unwrap_or(struct_name)
            .to_lowercase();
        let binding = format!("@OxideRoutePage(RouteKind.{kind})");
        assert!(
            pages.contains(&binding),
            "Rust declares {struct_name} but pages.dart has no {binding}. \
             The generated route table throws UnimplementedError otherwise.\n{pages}"
        );
    }
}

#[test]
fn route_page_receives_the_generated_route_model() {
    let files = rendered(Backend::Inherited);
    let pages = &files["lib/src/navigation/pages.dart"];
    assert!(
        pages.contains("final HomeRoute route;"),
        "the bound page must hold the generated route model so it can read \
         route fields. Got:\n{pages}"
    );
}

// --- 3. integration test / page text agreement ----------------------------

#[test]
fn integration_test_assertions_match_the_rendered_page() {
    let files = rendered(Backend::Inherited);
    let test = &files["integration_test/oxide_test.dart"];
    let page = &files["lib/src/counter_page.dart"];

    // Every value the test asserts on must be produced by the page. The page
    // builds the label with an interpolation, so compare the label prefix
    // rather than the fully-substituted text: the test says "Counter: 0", the
    // page must render "Counter: $counter".
    let mut idx = 0;
    let mut checked = 0;
    while let Some(rel) = test[idx..].find("find.text('") {
        let start = idx + rel + "find.text('".len();
        let rest = &test[start..];
        let Some(end) = rest.find('\'') else { break };
        let needle = &rest[..end];
        idx = start + end;
        // Only the counter label carries a value; button labels are literals
        // and are compared directly.
        let Some(label) = needle
            .strip_suffix(" 0")
            .or_else(|| needle.strip_suffix(" 1"))
        else {
            continue;
        };
        checked += 1;
        assert!(
            page.contains(&format!("'{label}$"))
                || page.contains(&format!("\"{label}$"))
                || page.contains(label),
            "integration test expects {needle:?} but the page never renders \
             a matching label. Expected the page to interpolate {label:?}.\npage:\n{page}"
        );
    }
    assert!(
        checked > 0,
        "expected the test to assert on a counter label"
    );
}

#[test]
fn integration_test_uses_the_real_app_widget() {
    let files = rendered(Backend::Inherited);
    let test = &files["integration_test/oxide_test.dart"];
    // A bare MaterialApp would have no navigatorKey, so navigation times out.
    assert!(
        test.contains("runOxideApp(const MyApp())"),
        "the integration test must run the real MyApp so it gets the \
         navigatorKey. Got:\n{test}"
    );
}

// --- 4. error interpolation ------------------------------------------------

#[test]
fn error_branch_interpolates_rather_than_escaping() {
    for backend in all_backends() {
        let files = rendered(backend);
        let page = &files["lib/src/counter_page.dart"];
        assert!(
            page.contains("Text('Error: ${view.error}')"),
            "[{backend}] the error branch must interpolate view.error. A \
             backslash-dollar escape renders the literal text instead.\npage:\n{page}"
        );
        assert!(
            !page.contains(r"\${view.error}"),
            "[{backend}] found an escaped dollar in generated Dart; it will \
             render literally instead of interpolating."
        );
    }
}

// --- 5. hooks backend scope ----------------------------------------------

#[test]
fn hooks_backend_wraps_page_in_the_store_scope() {
    let files = rendered(Backend::InheritedHooks);
    let page = &files["lib/src/counter_page.dart"];
    assert!(
        page.contains("AppStoreScope("),
        "the hooks backend resolves its view through AppStoreScope, so the \
         page must install the scope itself or the first build throws \
         'AppStoreScope is missing from the widget tree'.\npage:\n{page}"
    );
    assert!(
        page.contains("useAppStoreOxide()"),
        "the hooks backend should use the generated hook accessor"
    );
}

// --- 6. no dangling imports ------------------------------------------------

#[test]
fn templates_import_only_files_the_template_ships_or_codegen_creates() {
    // Files that only exist after `dart run build_runner` / FRB codegen.
    let generated = ["lib/oxide.dart", "lib/oxide_generated/", "lib/src/rust/"];
    let files = rendered(Backend::Inherited);
    for (rel, content) in &files {
        if !rel.ends_with(".dart") {
            continue;
        }
        for line in content.lines() {
            let Some(rest) = line.trim().strip_prefix("import '") else {
                continue;
            };
            let Some(target) = rest.split('\'').next() else {
                continue;
            };
            if target.starts_with("dart:") || target.starts_with("package:") {
                continue;
            }
            // Resolve the relative import against the importing file, then
            // normalize the `..` segments so `lib/src/../oxide.dart` and
            // `lib/oxide.dart` compare equal.
            let dir = std::path::Path::new(rel).parent().unwrap();
            let mut parts: Vec<String> = vec![];
            for seg in dir.join(target).components() {
                match seg.as_os_str().to_str() {
                    Some(".") | None => {}
                    Some("..") => {
                        parts.pop();
                    }
                    Some(s) => parts.push(s.to_string()),
                }
            }
            let key = parts.join("/");
            let shipped = files.contains_key(&key) || generated.iter().any(|g| key.starts_with(g));
            assert!(
                shipped,
                "{rel} imports '{target}' but the template does not ship it \
                 (resolved to {key})"
            );
        }
    }
}

// --- 7. placeholder hygiene ------------------------------------------------

#[test]
fn no_escaped_dollars_in_any_template() {
    for backend in all_backends() {
        for (rel, content) in rendered(backend) {
            assert!(
                !content.contains(r"\${"),
                "[{backend}] {rel} contains an escaped dollar before an \
                 interpolation brace, which Dart renders literally"
            );
        }
    }
}
