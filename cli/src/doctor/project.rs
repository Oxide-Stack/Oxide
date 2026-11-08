//! Project checks: Flutter app shape, Oxide/FRB deps, Rust crate, Dart
//! wiring, generated outputs, and single-init discipline.

use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use super::Check;

/// CLI version; project `oxide_*` deps must satisfy it.
const CLI_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn checks(root: &Path) -> Vec<Check> {
    // Note: the FRB pubspec pin is checked by `versions::frb_pin_check`, which
    // also compares the rust crate and the CLI's own pin. A separate
    // `project.frb-dep` here was a strict subset of it and both appeared in the
    // skew list, so one mismatch produced two FAIL rows.
    vec![
        check_is_flutter_app(root),
        check_oxide_deps(root),
        check_rust_lib_dep(root),
        check_frb_yaml(root),
        check_rust_crate(root),
        check_oxide_rs_deps(root),
        check_lib_rs_exports(root),
        check_routes_macro(root),
        check_oxide_dart(root),
        check_generated(root),
        check_init_calls(root),
    ]
}

fn read_file(root: &Path, rel: &str) -> Option<String> {
    fs::read_to_string(root.join(rel)).ok()
}

fn parse_pubspec(root: &Path) -> Option<serde_yaml::Value> {
    read_file(root, "pubspec.yaml").and_then(|s| serde_yaml::from_str(&s).ok())
}

fn yaml_get<'a>(v: &'a serde_yaml::Value, key: &str) -> Option<&'a serde_yaml::Value> {
    v.as_mapping()?
        .get(serde_yaml::Value::String(key.to_string()))
}

fn yaml_str(v: &serde_yaml::Value, key: &str) -> Option<String> {
    yaml_get(v, key)?.as_str().map(str::to_string)
}

/// Version requirement string for a pubspec dependency (string form only).
///
/// Looks in `dependencies` first, then `dev_dependencies`. The `?` operators
/// here used to return from the *whole function* rather than continuing the
/// loop, so a dependency present only under `dev_dependencies` — legal for
/// `flutter_rust_bridge` — was reported as absent, and the FRB pin check that
/// consumes this could not verify anything.
pub(crate) fn pubspec_dep_req(pubspec: &serde_yaml::Value, name: &str) -> Option<String> {
    let key = serde_yaml::Value::String(name.to_string());
    for section in ["dependencies", "dev_dependencies"] {
        let Some(deps) = yaml_get(pubspec, section).and_then(|v| v.as_mapping()) else {
            continue;
        };
        let Some(v) = deps.get(key.clone()) else {
            continue;
        };
        if let Some(s) = v.as_str() {
            return Some(s.to_string());
        }
        if v.is_mapping() {
            return None; // path/git/hosted dep, no version string
        }
    }
    None
}

/// Raw `flutter_rust_bridge` requirement from pubspec.yaml, if a plain version.
pub(crate) fn flutter_rust_bridge_pubspec_req(root: &Path) -> Option<String> {
    parse_pubspec(root).and_then(|p| pubspec_dep_req(&p, "flutter_rust_bridge"))
}

/// `rust_root` from flutter_rust_bridge.yaml, defaulting to `rust/`.
pub(crate) fn rust_root(root: &Path) -> PathBuf {
    let yaml = read_file(root, "flutter_rust_bridge.yaml")
        .and_then(|s| serde_yaml::from_str::<serde_yaml::Value>(&s).ok());
    let rel = yaml
        .as_ref()
        .and_then(|y| yaml_str(y, "rust_root"))
        .unwrap_or_else(|| "rust/".to_string());
    root.join(rel.trim_end_matches('/'))
}

fn parse_rust_manifest(root: &Path) -> Option<toml_edit::DocumentMut> {
    let text = fs::read_to_string(rust_root(root).join("Cargo.toml")).ok()?;
    text.parse::<toml_edit::DocumentMut>().ok()
}

/// Raw `flutter_rust_bridge` requirement from the Rust crate manifest.
pub(crate) fn flutter_rust_bridge_cargo_req(root: &Path) -> Option<String> {
    let doc = parse_rust_manifest(root)?;
    doc["dependencies"]["flutter_rust_bridge"]
        .as_str()
        .map(str::to_string)
}

fn check_is_flutter_app(root: &Path) -> Check {
    const ID: &str = "project.is-flutter-app";
    match parse_pubspec(root) {
        None => Check::fail(
            ID,
            "Flutter app",
            "pubspec.yaml missing or unparseable",
            "Run doctor from a Flutter project root (or pass --root)",
        ),
        Some(p) => {
            let has_flutter_dep = yaml_get(&p, "dependencies")
                .and_then(|d| d.as_mapping())
                .is_some_and(|m| m.contains_key(serde_yaml::Value::String("flutter".to_string())));
            let has_flutter_section = yaml_get(&p, "flutter").is_some();
            if has_flutter_dep || has_flutter_section {
                let name = yaml_str(&p, "name").unwrap_or_else(|| "?".to_string());
                Check::pass(
                    ID,
                    "Flutter app",
                    format!("{name} looks like a Flutter app"),
                )
            } else {
                Check::fail(
                    ID,
                    "Flutter app",
                    "pubspec.yaml has no flutter dependency or flutter: section",
                    "Run doctor from a Flutter project root (or pass --root)",
                )
            }
        }
    }
}

/// Matches a `pub use` line that re-exports `OxideError`.
fn reexports_oxide_error(src: &str) -> bool {
    static RE: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
    let re = RE.get_or_init(|| {
        regex::Regex::new(r"(?m)^\s*pub\s+use\b[^;]*\bOxideError\b").expect("valid re-export regex")
    });
    re.is_match(&strip_rust_comments(src))
}

/// Matches the `#[oxide_generator_rs::routes]` attribute.
fn routes_attr_re() -> &'static regex::Regex {
    static RE: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
    RE.get_or_init(|| {
        regex::Regex::new(r"#\s*\[\s*(?:oxide_generator_rs\s*::\s*)?routes\s*\]")
            .expect("valid routes attribute regex")
    })
}

/// Remove `//` and `/* */` comments so content checks do not match prose.
///
/// Not a full Rust lexer: it only needs to be good enough that a doc comment
/// mentioning a symbol does not satisfy a check looking for that symbol in code.
fn strip_rust_comments(src: &str) -> String {
    let mut out = String::with_capacity(src.len());
    let bytes = src.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'/' && i + 1 < bytes.len() {
            match bytes[i + 1] {
                b'/' => {
                    while i < bytes.len() && bytes[i] != b'\n' {
                        i += 1;
                    }
                }
                b'*' => {
                    i += 2;
                    while i + 1 < bytes.len() && !(bytes[i] == b'*' && bytes[i + 1] == b'/') {
                        i += 1;
                    }
                    i = (i + 2).min(bytes.len());
                }
                _ => {
                    out.push(bytes[i] as char);
                    i += 1;
                }
            }
            // Preserve newlines so line-based matching still lines up.
            out.push('\n');
        } else {
            let ch_len = utf8_len(bytes[i]);
            out.push_str(&src[i..i + ch_len]);
            i += ch_len;
        }
    }
    out
}

fn utf8_len(b: u8) -> usize {
    if b < 0x80 {
        1
    } else if b >> 5 == 0b110 {
        2
    } else if b >> 4 == 0b1110 {
        3
    } else if b >> 3 == 0b11110 {
        4
    } else {
        1
    }
}

/// Describe a non-versioned pubspec dependency, e.g. `path` or `git`.
///
/// Returns `None` for a mapping that is neither, which is what makes it a
/// recognized form rather than an unparseable one.
fn dep_source(v: &serde_yaml::Value) -> Option<&'static str> {
    ["path", "git", "hosted"].into_iter().find(|kind| {
        v.get(serde_yaml::Value::String((*kind).to_string()))
            .is_some()
    })
}

fn check_oxide_deps(root: &Path) -> Check {
    const ID: &str = "project.oxide-deps";
    let Some(pubspec) = parse_pubspec(root) else {
        return Check::fail(
            ID,
            "Oxide deps",
            "pubspec.yaml missing or unparseable",
            "Run doctor from a Flutter project root (or pass --root)",
        );
    };
    let cli = semver::Version::parse(CLI_VERSION).expect("cli version is semver");
    let mut problems = vec![];
    let mut notes = vec![];
    for name in ["oxide_runtime", "oxide_generator"] {
        let mut found = false;
        for section in ["dependencies", "dev_dependencies"] {
            let Some(deps) = yaml_get(&pubspec, section).and_then(|v| v.as_mapping()) else {
                continue;
            };
            let Some(v) = deps.get(serde_yaml::Value::String(name.to_string())) else {
                continue;
            };
            found = true;
            if let Some(s) = v.as_str() {
                match semver::VersionReq::parse(s) {
                    Ok(req) if req.matches(&cli) => notes.push(format!("{name} {s}")),
                    Ok(_) => {
                        problems.push(format!("{name} {s} does not satisfy cli {CLI_VERSION}"))
                    }
                    Err(_) => problems.push(format!("{name} has unparseable version `{s}`")),
                }
            } else if let Some(kind) = dep_source(v) {
                // `path` and `git` are both first-class: `oxide create
                // --oxide-source git` writes exactly this form, and rejecting it
                // made doctor fail a project it had just generated.
                notes.push(format!("{name} ({kind} dep)"));
            } else {
                problems.push(format!("{name} has an unrecognized dependency form"));
            }
        }
        if !found {
            problems.push(format!("{name} missing from pubspec.yaml"));
        }
    }
    if problems.is_empty() {
        Check::pass(ID, "Oxide deps", notes.join(", "))
    } else {
        Check::fail(
            ID,
            "Oxide deps",
            problems.join("; "),
            format!("Depend on oxide_runtime/oxide_generator compatible with cli {CLI_VERSION}"),
        )
    }
}

fn check_rust_lib_dep(root: &Path) -> Check {
    const ID: &str = "project.rust-lib-dep";
    let Some(pubspec) = parse_pubspec(root) else {
        return Check::fail(
            ID,
            "rust_lib dep",
            "pubspec.yaml missing or unparseable",
            "Run doctor from a Flutter project root (or pass --root)",
        );
    };
    let deps = yaml_get(&pubspec, "dependencies").and_then(|v| v.as_mapping());
    let mut found = vec![];
    let mut missing_path = vec![];
    if let Some(map) = deps {
        for (k, v) in map {
            let name = k.as_str().unwrap_or("");
            if !name.starts_with("rust_lib_") {
                continue;
            }
            match v.get(serde_yaml::Value::String("path".to_string())) {
                Some(p) if p.as_str().is_some_and(|s| root.join(s).is_dir()) => {
                    found.push(format!("{name} -> {}", p.as_str().unwrap_or("")));
                }
                _ => missing_path.push(name.to_string()),
            }
        }
    }
    if !found.is_empty() && missing_path.is_empty() {
        Check::pass(ID, "rust_lib dep", found.join(", "))
    } else if !found.is_empty() {
        Check::fail(
            ID,
            "rust_lib dep",
            format!(
                "{}; bad path for {}",
                found.join(", "),
                missing_path.join(", ")
            ),
            "Point each rust_lib_* dependency at its FRB rust_builder directory",
        )
    } else {
        Check::fail(
            ID,
            "rust_lib dep",
            "no rust_lib_* path dependency in pubspec.yaml (missing)",
            "Add e.g. `rust_lib_<app>: { path: rust_builder }` to dependencies",
        )
    }
}

fn check_frb_yaml(root: &Path) -> Check {
    const ID: &str = "project.frb-yaml";
    let Some(text) = read_file(root, "flutter_rust_bridge.yaml") else {
        return Check::fail(
            ID,
            "FRB config",
            "flutter_rust_bridge.yaml missing",
            "Add one with rust_input, rust_root and dart_output keys",
        );
    };
    let yaml: serde_yaml::Value = match serde_yaml::from_str(&text) {
        Ok(y) => y,
        Err(e) => {
            return Check::fail(
                ID,
                "FRB config",
                format!("flutter_rust_bridge.yaml is not valid YAML: {e}"),
                "Fix the YAML syntax",
            );
        }
    };
    let mut missing = vec![];
    let mut values = vec![];
    for key in ["rust_input", "rust_root", "dart_output"] {
        match yaml_str(&yaml, key) {
            Some(v) => values.push(format!("{key}={v}")),
            None => missing.push(key),
        }
    }
    if !missing.is_empty() {
        return Check::fail(
            ID,
            "FRB config",
            format!(
                "flutter_rust_bridge.yaml missing keys: {}",
                missing.join(", ")
            ),
            "Required keys: rust_input, rust_root, dart_output",
        );
    }
    if !rust_root(root).is_dir() {
        return Check::fail(
            ID,
            "FRB config",
            format!("{}; rust_root directory not found", values.join(" ")),
            "Point rust_root at the Rust crate directory",
        );
    }
    Check::pass(ID, "FRB config", values.join(" "))
}

fn check_rust_crate(root: &Path) -> Check {
    const ID: &str = "project.rust-crate";
    let manifest = rust_root(root).join("Cargo.toml");
    if !manifest.is_file() {
        return Check::fail(
            ID,
            "Rust crate",
            format!("{} missing", manifest.display()),
            "The FRB rust_root must contain a Cargo.toml",
        );
    }
    match parse_rust_manifest(root) {
        None => Check::fail(
            ID,
            "Rust crate",
            "rust Cargo.toml is not valid TOML",
            "Fix the TOML syntax",
        ),
        Some(doc) => {
            let kinds: Vec<String> = doc["lib"]["crate-type"]
                .as_array()
                .map(|a| {
                    a.iter()
                        .filter_map(|v| v.as_str().map(str::to_string))
                        .collect()
                })
                .unwrap_or_default();
            let has_cdylib = kinds.iter().any(|k| k == "cdylib");
            let has_staticlib = kinds.iter().any(|k| k == "staticlib");
            if has_cdylib && has_staticlib {
                Check::pass(ID, "Rust crate", format!("crate-type = {kinds:?}"))
            } else {
                Check::fail(
                    ID,
                    "Rust crate",
                    format!("crate-type must include cdylib + staticlib (found {kinds:?})"),
                    "Set `crate-type = [\"cdylib\", \"staticlib\"]` under [lib]",
                )
            }
        }
    }
}

fn check_oxide_rs_deps(root: &Path) -> Check {
    const ID: &str = "project.oxide-rs-deps";
    let Some(doc) = parse_rust_manifest(root) else {
        return Check::fail(
            ID,
            "Oxide Rust deps",
            "rust Cargo.toml missing or not valid TOML",
            "The FRB rust_root must contain a Cargo.toml",
        );
    };
    let deps = &doc["dependencies"];
    let mut missing = vec![];
    for name in ["oxide_core", "oxide_generator_rs"] {
        if deps.get(name).is_none() {
            missing.push(name);
        }
    }
    if !missing.is_empty() {
        return Check::fail(
            ID,
            "Oxide Rust deps",
            format!("rust Cargo.toml missing: {}", missing.join(", ")),
            "Depend on oxide_core and oxide_generator_rs (path or crates.io)",
        );
    }
    let feats = &doc["features"];
    let mut missing_feats = vec![];
    for name in ["navigation-binding", "isolated-channels"] {
        if feats.get(name).is_none() {
            missing_feats.push(name);
        }
    }
    if missing_feats.is_empty() {
        Check::pass(
            ID,
            "Oxide Rust deps",
            "oxide_core + oxide_generator_rs with features",
        )
    } else {
        Check::warn(
            ID,
            "Oxide Rust deps",
            format!("features not declared: {}", missing_feats.join(", ")),
            "Declare navigation-binding / isolated-channels features if the app uses them",
        )
    }
}

fn check_lib_rs_exports(root: &Path) -> Check {
    const ID: &str = "project.lib-rs-exports";
    match fs::read_to_string(rust_root(root).join("src").join("lib.rs")) {
        Err(_) => Check::fail(
            ID,
            "lib.rs exports",
            "rust/src/lib.rs missing",
            "The Rust crate needs src/lib.rs re-exporting OxideError",
        ),
        Ok(src) => {
            // Anchor to an actual `pub use` line and ignore comments. A bare
            // `contains("OxideError")` passed for
            // `// TODO: re-export OxideError`.
            if reexports_oxide_error(&src) {
                Check::pass(ID, "lib.rs exports", "OxideError re-exported")
            } else {
                Check::fail(
                    ID,
                    "lib.rs exports",
                    "rust/src/lib.rs does not re-export OxideError",
                    "Add `pub use oxide_core::{... OxideError}`",
                )
            }
        }
    }
}

fn check_routes_macro(root: &Path) -> Check {
    const ID: &str = "project.routes-macro";
    match fs::read_to_string(rust_root(root).join("src").join("lib.rs")) {
        Err(_) => Check::fail(
            ID,
            "routes macro",
            "rust/src/lib.rs missing",
            "The Rust crate needs src/lib.rs with a #[routes] module",
        ),
        Ok(src) => {
            // Require the crate-qualified attribute. The previous pattern
            // `#\[.*?routes.*?\]` also matched `#[cfg(feature = "routes")]` and
            // a commented-out attribute, and disagreed with the shape check in
            // `project::detect_shape`, so "has the routes macro" had two
            // different definitions in one crate.
            if routes_attr_re().is_match(&strip_rust_comments(&src)) {
                Check::pass(ID, "routes macro", "#[routes] module present")
            } else {
                Check::fail(
                    ID,
                    "routes macro",
                    "no #[oxide_generator_rs::routes] attribute in rust/src/lib.rs",
                    "Add `#[oxide_generator_rs::routes] pub mod routes {}`",
                )
            }
        }
    }
}

fn check_oxide_dart(root: &Path) -> Check {
    const ID: &str = "project.oxide-dart";
    match read_file(root, "lib/src/oxide.dart") {
        None => Check::fail(
            ID,
            "oxide.dart",
            "lib/src/oxide.dart missing",
            "The app needs lib/src/oxide.dart declaring @OxideStore models",
        ),
        Some(src) => {
            let has_part = src.contains("part") && src.contains("oxide.oxide.g.dart");
            let has_store = src.contains("@OxideStore");
            if has_part && has_store {
                Check::pass(ID, "oxide.dart", "part + @OxideStore present")
            } else {
                let mut missing = vec![];
                if !has_part {
                    missing.push("part 'oxide.oxide.g.dart'");
                }
                if !has_store {
                    missing.push("@OxideStore");
                }
                Check::fail(
                    ID,
                    "oxide.dart",
                    format!("lib/src/oxide.dart missing: {}", missing.join(", ")),
                    "Declare @OxideStore models and the generated part directive",
                )
            }
        }
    }
}

fn newest_mtime(path: &Path) -> Option<SystemTime> {
    fs::metadata(path).and_then(|m| m.modified()).ok()
}

fn check_generated(root: &Path) -> Check {
    const ID: &str = "project.generated";
    let outputs = [
        "lib/src/rust/frb_generated.dart",
        "lib/oxide_generated/oxide_stack.g.dart",
    ];
    let missing: Vec<&str> = outputs
        .iter()
        .filter(|rel| !root.join(rel).is_file())
        .copied()
        .collect();
    if !missing.is_empty() {
        return Check::fail(
            ID,
            "generated outputs",
            format!("missing generated files: {}", missing.join(", ")),
            "Run `oxide generate` (flutter_rust_bridge + build_runner)",
        );
    }
    // Freshness via mtime fallback: newest input vs oldest output.
    let mut inputs: Vec<PathBuf> = vec![];
    let rust_src = rust_root(root).join("src");
    if rust_src.is_dir() {
        for entry in walkdir::WalkDir::new(&rust_src)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.path().extension().is_some_and(|x| x == "rs"))
        {
            inputs.push(entry.path().to_path_buf());
        }
    }
    for rel in ["lib/src/oxide.dart", "flutter_rust_bridge.yaml"] {
        inputs.push(root.join(rel));
    }
    let newest_in = inputs.iter().filter_map(|p| newest_mtime(p)).max();
    let oldest_out = outputs
        .iter()
        .filter_map(|rel| newest_mtime(&root.join(rel)))
        .min();
    match (newest_in, oldest_out) {
        (Some(i), Some(o)) if i > o => Check::warn(
            ID,
            "generated outputs",
            "generated outputs are older than rust/src or oxide.dart (stale)",
            "Re-run `oxide generate`",
        ),
        _ => Check::pass(ID, "generated outputs", "FRB + OxideStack outputs present"),
    }
}

fn is_generated_path(path: &Path) -> bool {
    // Note: `path.extension()` of `app_state.g.dart` is `"dart"`, not
    // `"g.dart"`, so a clause comparing against `"g.dart"` could never be true.
    // The `ends_with` test below is what actually does the work; a future edit
    // that "simplified" this to an extension comparison would start scanning
    // generated glue and report phantom double-init failures.
    path.components()
        .any(|c| c.as_os_str() == "oxide_generated")
        || path
            .file_name()
            .is_some_and(|n| n.to_string_lossy().ends_with(".g.dart"))
}

/// Find `OxideStack.init(...)` / `runOxideApp(...)` call sites.
///
/// `files` is (display path, content). Generated files, comments and the
/// `runOxideApp` definition itself are ignored. Returns `path:line` hits.
pub(crate) fn find_init_calls(files: &[(String, String)]) -> Vec<String> {
    let call = regex::Regex::new(r"\b(?:OxideStack\.init|runOxideApp)\s*\(").expect("valid regex");
    let def = regex::Regex::new(r"static\b.*\binit\s*\(").expect("valid regex");
    let mut hits = vec![];
    for (path, content) in files {
        if path.contains("oxide_generated") || path.ends_with(".g.dart") {
            continue;
        }
        for (i, line) in content.lines().enumerate() {
            let trimmed = line.trim();
            if trimmed.starts_with("//") {
                continue;
            }
            if trimmed.contains("Future<void> runOxideApp") || def.is_match(line) {
                continue;
            }
            if call.is_match(line) {
                hits.push(format!("{}:{}", path, i + 1));
            }
        }
    }
    hits
}

fn check_init_calls(root: &Path) -> Check {
    const ID: &str = "project.init";
    let lib = root.join("lib");
    if !lib.is_dir() {
        return Check::fail(
            ID,
            "app init",
            "lib/ directory missing",
            "Run doctor from a Flutter project root (or pass --root)",
        );
    }
    let mut files = vec![];
    for entry in walkdir::WalkDir::new(&lib)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().is_some_and(|x| x == "dart"))
    {
        let path = entry.path();
        if is_generated_path(path) {
            continue;
        }
        if let Ok(content) = fs::read_to_string(path) {
            let rel = path
                .strip_prefix(root)
                .unwrap_or(path)
                .to_string_lossy()
                .replace('\\', "/");
            files.push((rel, content));
        }
    }
    match find_init_calls(&files).as_slice() {
        [] => Check::fail(
            ID,
            "app init",
            "no OxideStack.init()/runOxideApp() call found in lib/ (excluding generated)",
            "Call `await runOxideApp(...)` once from main()",
        ),
        [one] => Check::pass(ID, "app init", format!("single init at {one}")),
        many => Check::warn(
            ID,
            "app init",
            format!("possible double init at {}", many.join(", ")),
            "Init exactly once from main(); remove the duplicate",
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_init_passes_and_double_init_warns() {
        let single = vec![(
            "lib/main.dart".to_string(),
            "await runOxideApp(const MyApp());\n".to_string(),
        )];
        assert_eq!(find_init_calls(&single).len(), 1);

        let double = vec![
            (
                "lib/main.dart".to_string(),
                "await runOxideApp(const MyApp());\n".to_string(),
            ),
            (
                "lib/other.dart".to_string(),
                "await OxideStack.init();\n".to_string(),
            ),
        ];
        assert_eq!(find_init_calls(&double).len(), 2);
    }

    #[test]
    fn generated_defs_and_comments_are_ignored() {
        let files = vec![
            (
                "lib/oxide_generated/oxide_stack.g.dart".to_string(),
                "await OxideStack.init();\nFuture<void> runOxideApp(Widget app) async {}\n"
                    .to_string(),
            ),
            (
                "lib/src/x.g.dart".to_string(),
                "runOxideApp(x);\n".to_string(),
            ),
            (
                "lib/main.dart".to_string(),
                "// runOxideApp(old);\nawait runOxideApp(const MyApp());\n".to_string(),
            ),
        ];
        assert_eq!(find_init_calls(&files), vec!["lib/main.dart:2".to_string()]);
    }

    #[test]
    fn oxide_dart_markers_detected() {
        let good = "part 'oxide.oxide.g.dart';\n@OxideStore(state: S)\nclass M {}\n";
        assert!(good.contains("oxide.oxide.g.dart") && good.contains("@OxideStore"));
        let bad = "class M {}\n";
        assert!(!(bad.contains("oxide.oxide.g.dart") && bad.contains("@OxideStore")));
    }
}
