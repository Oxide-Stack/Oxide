//! `oxide integrate` — add the Oxide layer to an existing Flutter app.
//!
//! Wiring contract (for `src/cli.rs` / `src/main.rs`, owned by another workstream):
//! - `IntegrateArgs` derives `clap::Args` and can be embedded as a subcommand.
//! - Entry point: [`run`] takes `IntegrateArgs` by value and returns
//!   `anyhow::Result<IntegrateReport>`, so `Commands::Integrate(a)` can convert
//!   `cli::IntegrateArgs` field-by-field: `path` comes from the `--root`
//!   global, `backend: a.backend.to_string()`, `oxide_source` (`"auto"` when
//!   `OxideSource::Pub` without `--oxide-path`, else `to_string()`),
//!   `example: a.example.to_string()`, `no_init_main: a.no_init_main`,
//!   flags `keep_frb_example` / `skip_frb` / `no_codegen` / `dry_run` / `yes`,
//!   `oxide_path`.
//! - Setting env `OXIDE_CLI_NO_EXTERNAL=1` skips spawning external tools
//!   (used by tests).
//!
//! Shapes handled:
//! - `FlutterNoFrb` — plain Flutter app: back up `lib/main.dart`, run
//!   `flutter_rust_bridge_codegen integrate`, restore the backup (FRB comments
//!   out the existing entrypoint), then continue as `FrbApp`.
//! - `FrbApp` — FRB app without Oxide: additive overlay only, never rewrites
//!   user code; demo files removed only by content signature.
//! - `OxideApp` — already integrated: idempotent fill-missing; a second run is
//!   a no-op.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};

use crate::common::{
    fs::write_atomic,
    gitignore::merge as merge_gitignore,
    glue::{GlueSnapshot, nuke as nuke_oxide_glue},
    template::Vars,
};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Oxide crate/package version scaffolded by this CLI.
pub const OXIDE_VERSION: &str = "0.5.0";
/// Upstream git URL used for `--oxide-source git`.
pub const OXIDE_GIT_URL: &str = "https://github.com/Oxide-Stack/Oxide";
/// Marker comment prefix opening Oxide-managed patches.
pub const MARK_BEGIN_PREFIX: &str = "// oxide:begin";
/// Marker comment prefix closing Oxide-managed patches.
pub const MARK_END_PREFIX: &str = "// oxide:end";

/// Help snippet printed when `main.dart` does not match the expected pattern.
pub const MAIN_SNIPPET: &str = r#"// Add Oxide initialization to main.dart manually:
import 'oxide.dart';

Future<void> main() async {
  await runOxideApp(const MyApp());
}"#;

/// Starter entrypoint replacing an unmodified FRB demo `lib/main.dart`
/// (or recreating a missing one). Boots the integrated demo page.
pub const STARTER_MAIN_DART: &str = r#"import 'package:flutter/material.dart';

import 'oxide.dart';
import 'src/oxide_demo_page.dart';

Future<void> main() async {
  // Oxide + FRB initialization and binding boilerplate is handled by runOxideApp.
  await runOxideApp(const MaterialApp(home: OxideDemoPage()));
}
"#;

// ---------------------------------------------------------------------------
// Args + report
// ---------------------------------------------------------------------------

/// Arguments for `oxide integrate`.
#[derive(Debug, Clone, clap::Args)]
pub struct IntegrateArgs {
    /// Path to the Flutter app root (default: current dir).
    #[arg(long, default_value = ".")]
    pub path: PathBuf,
    /// UI backend: `inherited` (default), `hooks`, `riverpod`, `bloc`.
    #[arg(long, default_value = "inherited")]
    pub backend: String,
    /// Oxide dependency source: `auto` (default), `pub`, `path`, `git`.
    #[arg(long, default_value = "auto")]
    pub oxide_source: String,
    /// Demo page: `counter` (default) writes `lib/src/oxide_demo_page.dart`;
    /// `none` skips it.
    #[arg(long, default_value = "counter")]
    pub example: String,
    /// Keep FRB demo files even when they match the demo signature.
    #[arg(long, default_value_t = false)]
    pub keep_frb_example: bool,
    /// Skip patching `lib/main.dart` (init patch is on by default).
    #[arg(long = "no-init-main", default_value_t = false)]
    pub no_init_main: bool,
    /// Skip codegen (`flutter pub get` / FRB generate / build_runner).
    #[arg(long, default_value_t = false)]
    pub no_codegen: bool,
    /// Print unified diffs of the planned changes without writing anything.
    #[arg(long, default_value_t = false)]
    pub dry_run: bool,
    /// Skip interactive confirmations.
    #[arg(long, default_value_t = false)]
    pub yes: bool,
    /// Skip running `flutter_rust_bridge_codegen integrate` for `FlutterNoFrb`
    /// projects (assume FRB wiring already exists).
    #[arg(long, default_value_t = false)]
    pub skip_frb: bool,
    /// Explicit Oxide checkout root, overriding `--oxide-source` detection.
    #[arg(long)]
    pub oxide_path: Option<PathBuf>,
}

/// Outcome of [`run`].
#[derive(Debug, Clone)]
pub struct IntegrateReport {
    /// Detected project shape (`FlutterNoFrb` / `FrbApp` / `OxideApp`).
    pub shape: String,
    /// Relative paths created or patched (sorted, deduplicated).
    pub changed: Vec<String>,
    /// True when nothing needed to change (idempotent second run).
    pub noop: bool,
}

// ---------------------------------------------------------------------------
// Backend
// ---------------------------------------------------------------------------

/// Backend key used by [`patch_pubspec`] and template rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackendKind {
    Inherited,
    InheritedHooks,
    Riverpod,
    Bloc,
}

/// Parse `--backend`.
pub fn parse_backend(s: &str) -> Result<BackendKind> {
    match s {
        "inherited" => Ok(BackendKind::Inherited),
        "hooks" | "inherited-hooks" | "inheritedHooks" => Ok(BackendKind::InheritedHooks),
        "riverpod" => Ok(BackendKind::Riverpod),
        "bloc" => Ok(BackendKind::Bloc),
        other => bail!("unknown backend `{other}` (expected inherited|hooks|riverpod|bloc)"),
    }
}

impl BackendKind {
    pub fn as_str(self) -> &'static str {
        match self {
            BackendKind::Inherited => "inherited",
            BackendKind::InheritedHooks => "hooks",
            BackendKind::Riverpod => "riverpod",
            BackendKind::Bloc => "bloc",
        }
    }

    pub fn oxide_backend_expr(self) -> &'static str {
        match self {
            BackendKind::Inherited => "OxideBackend.inherited",
            BackendKind::InheritedHooks => "OxideBackend.inheritedHooks",
            BackendKind::Riverpod => "OxideBackend.riverpod",
            BackendKind::Bloc => "OxideBackend.bloc",
        }
    }

    pub fn oxide_extra_imports(self) -> &'static str {
        match self {
            BackendKind::Inherited => "",
            BackendKind::InheritedHooks => "import 'package:flutter_hooks/flutter_hooks.dart';\n",
            BackendKind::Riverpod => "import 'package:flutter_riverpod/flutter_riverpod.dart';\n",
            BackendKind::Bloc => "import 'package:flutter_bloc/flutter_bloc.dart';\n",
        }
    }

    /// `(pubspec key, entry lines)` pairs for `dependencies:`.
    pub fn dep_entries(self) -> Vec<(String, Vec<String>)> {
        match self {
            BackendKind::Inherited => vec![],
            BackendKind::InheritedHooks => vec![(
                "flutter_hooks".to_string(),
                vec!["flutter_hooks: ^0.20.5".to_string()],
            )],
            BackendKind::Riverpod => vec![(
                "flutter_riverpod".to_string(),
                vec!["flutter_riverpod: ^3.2.0".to_string()],
            )],
            BackendKind::Bloc => vec![
                ("bloc".to_string(), vec!["bloc: ^8.1.0".to_string()]),
                (
                    "flutter_bloc".to_string(),
                    vec!["flutter_bloc: ^8.1.3".to_string()],
                ),
            ],
        }
    }

    pub fn page_imports(self) -> &'static str {
        match self {
            BackendKind::Inherited => "",
            BackendKind::InheritedHooks => "import 'package:flutter_hooks/flutter_hooks.dart';\n",
            BackendKind::Riverpod => "import 'package:flutter_riverpod/flutter_riverpod.dart';\n",
            BackendKind::Bloc => {
                "import 'package:flutter_bloc/flutter_bloc.dart';\nimport 'package:oxide_runtime/oxide_runtime.dart';\nimport 'rust/state/app_state.dart';\n"
            }
        }
    }

    /// Page widget body wiring the generated `AppStore` glue to `_CounterView`.
    /// Same generated-name assumptions as `create.rs` (see its docs).
    pub fn page_body(self, page_class: &str) -> String {
        let loading = "return const Scaffold(body: Center(child: CircularProgressIndicator()));";
        // Single `$` on purpose: this string is emitted as Dart source, and
        // `\$` would be an escaped dollar that renders the literal text
        // "${view.error}" instead of interpolating the value.
        let error = "return Scaffold(body: Center(child: Text('Error: ${view.error}')));";
        let no_state = "return const Scaffold(body: Center(child: Text('No state yet')));";
        match self {
            BackendKind::Inherited => format!(
                r#"class {page_class} extends StatelessWidget {{
  const {page_class}({{super.key}});

  @override
  Widget build(BuildContext context) {{
    return AppStoreScope(
      child: Builder(
        builder: (context) {{
          final controller = AppStoreScope.controllerOf(context);
          return AnimatedBuilder(
            animation: controller,
            builder: (context, _) {{
              final view = controller.oxide;
              final state = view.state;
              if (view.isLoading) {{
                {loading}
              }}
              if (view.error != null) {{
                {error}
              }}
              if (state == null) {{
                {no_state}
              }}
              return _CounterView(
                counter: state.counter.toInt(),
                onIncrement: () => view.actions.increment(),
                onDecrement: () => view.actions.decrement(),
                onReset: () => view.actions.reset(),
              );
            }},
          );
        }},
      ),
    );
  }}
}}"#
            ),
            BackendKind::InheritedHooks => format!(
                r#"class {page_class} extends HookWidget {{
  const {page_class}({{super.key}});

  @override
  Widget build(BuildContext context) {{
    final view = useAppStoreOxide();
    final state = view.state;
    if (view.isLoading) {{
      {loading}
    }}
    if (view.error != null) {{
      {error}
    }}
    if (state == null) {{
      {no_state}
    }}
    return _CounterView(
      counter: state.counter.toInt(),
      onIncrement: () => view.actions.increment(),
      onDecrement: () => view.actions.decrement(),
      onReset: () => view.actions.reset(),
    );
  }}
}}"#
            ),
            BackendKind::Riverpod => format!(
                r#"class {page_class} extends StatelessWidget {{
  const {page_class}({{super.key}});

  @override
  Widget build(BuildContext context) {{
    return const ProviderScope(child: _CounterConsumer());
  }}
}}

class _CounterConsumer extends ConsumerWidget {{
  const _CounterConsumer();

  @override
  Widget build(BuildContext context, WidgetRef ref) {{
    final view = ref.watch(appStoreProvider);
    final state = view.state;
    if (view.isLoading) {{
      {loading}
    }}
    if (view.error != null) {{
      {error}
    }}
    if (state == null) {{
      {no_state}
    }}
    final actions = ref.read(appStoreProvider).actions;
    return _CounterView(
      counter: state.counter.toInt(),
      onIncrement: () => actions.increment(),
      onDecrement: () => actions.decrement(),
      onReset: () => actions.reset(),
    );
  }}
}}"#
            ),
            BackendKind::Bloc => format!(
                r#"class {page_class} extends StatelessWidget {{
  const {page_class}({{super.key}});

  @override
  Widget build(BuildContext context) {{
    return AppStoreScope(
      child: BlocBuilder<AppStoreCubit, OxideView<AppState, AppStoreActions>>(
        builder: (context, view) {{
          final state = view.state;
          if (view.isLoading) {{
            {loading}
          }}
          if (view.error != null) {{
            {error}
          }}
          if (state == null) {{
            {no_state}
          }}
          final actions = context.read<AppStoreCubit>().actions;
          return _CounterView(
            counter: state.counter.toInt(),
            onIncrement: () => actions.increment(),
            onDecrement: () => actions.decrement(),
            onReset: () => actions.reset(),
          );
        }},
      ),
    );
  }}
}}"#
            ),
        }
    }
}

// ---------------------------------------------------------------------------
// Oxide source
// ---------------------------------------------------------------------------

/// Where Oxide dependencies come from.
/// Public because [`patch_pubspec`] takes it.
#[derive(Debug, Clone)]
pub enum OxideSourceKind {
    Pub,
    Path { root: PathBuf },
    Git { url: String, tag: String },
}

fn find_checkout_root(cwd: &Path) -> Option<PathBuf> {
    let mut dir = cwd.canonicalize().ok()?;
    loop {
        if dir.join("rust/oxide_core/Cargo.toml").is_file()
            && dir.join("flutter/oxide_runtime/pubspec.yaml").is_file()
        {
            return Some(dir);
        }
        if !dir.pop() {
            return None;
        }
    }
}

fn resolve_oxide_source(flag: &str, cwd: &Path) -> Result<OxideSourceKind> {
    match flag {
        "pub" => Ok(OxideSourceKind::Pub),
        "git" => Ok(OxideSourceKind::Git {
            url: OXIDE_GIT_URL.to_string(),
            tag: format!("v{OXIDE_VERSION}"),
        }),
        "path" => match find_checkout_root(cwd) {
            Some(root) => Ok(OxideSourceKind::Path { root }),
            None => bail!("--oxide-source path requires running inside the Oxide checkout"),
        },
        "auto" => Ok(match find_checkout_root(cwd) {
            Some(root) => OxideSourceKind::Path { root },
            None => OxideSourceKind::Pub,
        }),
        other => bail!("unknown --oxide-source `{other}` (expected auto|pub|path|git)"),
    }
}

/// Relative path from the directory containing `from_file` to `to`,
/// using forward slashes.
fn relative_forward(from_file: &Path, to: &Path) -> String {
    let from_dir = from_file.parent().unwrap_or_else(|| Path::new("."));
    let mut from = from_dir.components().peekable();
    let mut to_it = to.components().peekable();
    loop {
        match (from.peek(), to_it.peek()) {
            (Some(a), Some(b)) if a == b => {
                from.next();
                to_it.next();
            }
            _ => break,
        }
    }
    let mut rel = String::new();
    for _ in from {
        rel.push_str("../");
    }
    let rest: Vec<String> = to_it
        .map(|c| c.as_os_str().to_string_lossy().into_owned())
        .collect();
    rel.push_str(&rest.join("/"));
    if rel.is_empty() { ".".to_string() } else { rel }
}

// ---------------------------------------------------------------------------
// Shape detection
// ---------------------------------------------------------------------------

/// Detected project shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectShape {
    FlutterNoFrb,
    FrbApp,
    OxideApp,
}

impl ProjectShape {
    pub fn as_str(self) -> &'static str {
        match self {
            ProjectShape::FlutterNoFrb => "FlutterNoFrb",
            ProjectShape::FrbApp => "FrbApp",
            ProjectShape::OxideApp => "OxideApp",
        }
    }
}

/// Classify the project at `root`.
pub fn detect_shape(root: &Path) -> ProjectShape {
    let oxide_dart = root.join("lib/src/oxide.dart");
    if oxide_dart.is_file() {
        let content = std::fs::read_to_string(&oxide_dart).unwrap_or_default();
        if content.contains("@OxideStore") {
            return ProjectShape::OxideApp;
        }
    }
    let rust_state = root.join("rust/src/state");
    let bridge = root.join("rust/src/api/bridge.rs");
    let oxide_bridge = root.join("rust/src/api/oxide_bridge.rs");
    if rust_state.is_dir() && (bridge.is_file() || oxide_bridge.is_file()) {
        return ProjectShape::OxideApp;
    }
    if root.join("flutter_rust_bridge.yaml").is_file()
        || (root.join("rust/Cargo.toml").is_file() && root.join("lib/src/rust").exists())
        || root.join("rust_builder").is_dir()
    {
        return ProjectShape::FrbApp;
    }
    ProjectShape::FlutterNoFrb
}

// ---------------------------------------------------------------------------
// Marker + EOL helpers (public for tests)
// ---------------------------------------------------------------------------

/// Wrap `content` in an `// oxide:begin/end <id>` marker block.
pub fn marker_block(id: &str, content: &str) -> String {
    format!("{MARK_BEGIN_PREFIX} {id}\n{content}\n{MARK_END_PREFIX} {id}")
}

/// True when `text` already contains the marker block `id`.
pub fn has_marker(text: &str, id: &str) -> bool {
    text.contains(&format!("{MARK_BEGIN_PREFIX} {id}"))
}

/// Dominant line ending of `s` (`\r\n` only if present, else `\n`).
pub fn detect_eol(s: &str) -> &'static str {
    if s.contains("\r\n") { "\r\n" } else { "\n" }
}

/// Convert `\n` newlines in `s` to `eol`.
pub fn to_eol(s: &str, eol: &str) -> String {
    if eol == "\n" {
        s.replace("\r\n", "\n")
    } else {
        s.replace("\r\n", "\n").replace('\n', "\r\n")
    }
}

// ---------------------------------------------------------------------------
// Patch builders (pure, tested)
// ---------------------------------------------------------------------------

/// Patch `rust/src/lib.rs`: insert missing Oxide items in a marker block.
/// Returns `None` when nothing needs to change.
pub fn patch_lib_rs(orig: &str) -> Option<String> {
    if has_marker(orig, "oxide-lib") {
        return None;
    }
    let mut block_lines = Vec::new();
    if !orig.contains("mod state;") {
        block_lines.push("mod state;");
    }
    if !orig.contains("oxide_core::{CoreResult") {
        block_lines.push("pub use oxide_core::{CoreResult, OxideError};");
    }
    if !orig.contains("oxide_generator_rs::routes") {
        block_lines.push("#[oxide_generator_rs::routes]");
        block_lines.push("pub mod routes {}");
    }
    if block_lines.is_empty() {
        return None;
    }
    let mut out = orig.to_string();
    if !out.ends_with('\n') && !out.is_empty() {
        out.push('\n');
    }
    out.push_str(&marker_block("oxide-lib", &block_lines.join("\n")));
    out.push('\n');
    Some(out)
}

/// Patch `rust/src/api/mod.rs`: ensure the bridge module entry exists.
/// Returns `None` when nothing needs to change.
pub fn patch_api_mod(orig: &str, bridge_mod: &str) -> Option<String> {
    let entry = format!("pub mod {bridge_mod};");
    if orig.lines().any(|l| l.trim() == entry) || has_marker(orig, "oxide-api") {
        return None;
    }
    let mut out = orig.to_string();
    if !out.ends_with('\n') && !out.is_empty() {
        out.push('\n');
    }
    out.push_str(&marker_block("oxide-api", &entry));
    out.push('\n');
    Some(out)
}

/// Outcome of [`patch_main_dart`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MainPatch {
    /// `main.dart` already uses `runOxideApp`.
    AlreadyPatched,
    /// Pattern not found; caller should print [`MAIN_SNIPPET`].
    PatternMiss,
    /// Patched content.
    Patched(String),
}

/// Matches a top-level `main` declaration in any legal Dart spelling.
///
/// The previous code looked for the literal `"void main() {"`, so `main() {`,
/// `void  main() {` (two spaces) and `void main()\n{` were all missed while the
/// `await` was still inserted, producing a non-async `main` containing `await`:
/// an analyzer error reported as success.
fn main_decl_re() -> &'static regex::Regex {
    static RE: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
    RE.get_or_init(|| {
        regex::Regex::new(
            r"(?m)^[ \t]*(?:Future\s*<\s*void\s*>|void)?\s*main\s*\(\s*\)\s*(?:async\s*)?\{",
        )
        .expect("valid main declaration regex")
    })
}

/// True when `main` is already declared `async`.
fn main_is_async(src: &str) -> bool {
    main_decl_re()
        .find_iter(src)
        .any(|m| m.as_str().contains("async"))
}

/// Rewrite the `main` declaration to `Future<void> main() async {`.
fn make_main_async(src: &str) -> String {
    main_decl_re()
        .replace_all(src, "Future<void> main() async {")
        .into_owned()
}

/// Patch `lib/main.dart`: wrap `runApp(` with `runOxideApp(`, make `main`
/// async, and add the `oxide.dart` import in a marker block. Never touches a
/// file that already uses `runOxideApp`.
///
/// `runApp(` is only rewritten inside `main`'s body. `str::replace` is global,
/// so a second `runApp(` in another function or in a comment used to pick up an
/// `await` too, which is a syntax error in any non-async function.
pub fn patch_main_dart(orig: &str) -> MainPatch {
    if orig.contains("runOxideApp") {
        return MainPatch::AlreadyPatched;
    }
    if !orig.contains("runApp(") {
        return MainPatch::PatternMiss;
    }
    let eol = detect_eol(orig);
    let mut out = rewrite_runapp_in_main(orig);
    if !main_is_async(&out) {
        out = make_main_async(&out);
    }
    if !has_marker(&out, "oxide-main-import") {
        let import_block = to_eol(
            &marker_block("oxide-main-import", "import 'oxide.dart';"),
            eol,
        );
        let mut lines: Vec<String> = out.lines().map(|s| s.to_string()).collect();
        let mut idx = 0;
        for (i, l) in lines.iter().enumerate() {
            let t = l.trim_start();
            if t.starts_with("import ") || t.starts_with("export ") {
                idx = i + 1;
            }
        }
        lines.insert(idx, import_block);
        out = lines.join(eol);
        if orig.ends_with('\n') {
            out.push_str(eol);
        }
    }
    MainPatch::Patched(out)
}

/// Replace `runApp(` with `await runOxideApp(` only within `main`'s body.
///
/// The body is delimited by brace matching from the `main` declaration, so
/// `runApp(` elsewhere in the file is left alone.
fn rewrite_runapp_in_main(src: &str) -> String {
    let Some(decl) = main_decl_re().find(src) else {
        return src.to_string();
    };
    let body_start = decl.end();
    // Find the opening brace of the body; the regex consumes it, so `body_start`
    // already points just past it.
    let bytes = src.as_bytes();
    let mut depth = 1usize;
    let mut i = body_start;
    while i < bytes.len() {
        match bytes[i] {
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    break;
                }
            }
            b'"' | b'\'' | b'`' => {
                // Skip over string literals so a brace inside one does not
                // throw off the match.
                let quote = bytes[i];
                i += 1;
                while i < bytes.len() {
                    if bytes[i] == b'\\' {
                        i += 2;
                        continue;
                    }
                    if bytes[i] == quote {
                        break;
                    }
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }
    let body_end = i.min(src.len());
    if body_end <= body_start {
        return src.to_string();
    }
    let mut out = String::with_capacity(src.len() + 16);
    out.push_str(&src[..body_start]);
    out.push_str(&src[body_start..body_end].replace("runApp(", "await runOxideApp("));
    out.push_str(&src[body_end..]);
    out
}

// ---------------------------------------------------------------------------
// FRB demo signature detection (mirrors create.rs)
// ---------------------------------------------------------------------------

/// True when `rust/src/api/simple.rs` still contains the unmodified FRB demo:
/// the greet function (plus FRB's own `init_app` hook) and nothing else.
/// User additions (any other `pub fn`) mean real code lives here.
pub fn is_frb_demo_rust_api(file_name: &str, content: &str) -> bool {
    if file_name != "simple.rs" || !content.contains("pub fn greet") {
        return false;
    }
    content
        .lines()
        .filter_map(|l| {
            let (_, rest) = l.split_once("pub fn ")?;
            Some(
                rest.split(['(', '<', ' '])
                    .next()
                    .unwrap_or("")
                    .trim()
                    .to_string(),
            )
        })
        .all(|name| name == "greet" || name == "init_app")
}

/// True when `lib/main.dart` still looks like the FRB demo entrypoint.
pub fn is_frb_demo_dart_main(content: &str) -> bool {
    content.contains("api/simple") && content.contains("greet")
}

/// True when an integration test still looks like the FRB demo test.
pub fn is_frb_demo_integration_test(file_name: &str, content: &str) -> bool {
    file_name == "simple_test.dart"
        && (content.contains("greet") || content.contains("Can call rust function"))
}

// ---------------------------------------------------------------------------
// Backups, commands, confirmations, atomic writes
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, serde::Serialize)]
struct BackupEntry {
    path: String,
    backup: String,
}

/// Copy `files` (absolute paths) under `.oxide/backups/<ts>/` + manifest.
fn backup_files(root: &Path, files: &[PathBuf]) -> Result<PathBuf> {
    let ts = chrono::Local::now().format("%Y%m%d-%H%M%S").to_string();
    let dir = root.join(".oxide/backups").join(&ts);
    std::fs::create_dir_all(&dir).context("failed to create backup dir")?;
    let mut entries = Vec::new();
    for f in files {
        let rel = f
            .strip_prefix(root)
            .unwrap_or(f)
            .to_string_lossy()
            .replace('\\', "/");
        let dest = dir.join(&rel);
        if let Some(parent) = dest.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::copy(f, &dest).with_context(|| format!("failed to back up {}", f.display()))?;
        entries.push(BackupEntry {
            path: rel.clone(),
            backup: dest
                .strip_prefix(root)
                .unwrap_or(&dest)
                .to_string_lossy()
                .replace('\\', "/"),
        });
    }
    let manifest = serde_json::to_string_pretty(&serde_json::json!({
        "created": ts,
        "files": entries,
    }))?;
    std::fs::write(dir.join("manifest.json"), manifest)?;
    Ok(dir)
}

fn run_cmd(prog: &str, args: &[&str], dir: &Path) -> Result<()> {
    if crate::common::no_external() {
        println!(
            "(skip external: {} {} [in {}])",
            prog,
            args.join(" "),
            dir.display()
        );
        return Ok(());
    }
    // Windows batch shims (flutter.bat, dart.bat) need `cmd /C`.
    let resolved = which::which(prog).unwrap_or_else(|_| std::path::PathBuf::from(prog));
    let is_script = resolved
        .extension()
        .is_some_and(|e| e.eq_ignore_ascii_case("bat") || e.eq_ignore_ascii_case("cmd"));
    let mut cmd = if cfg!(windows) && is_script {
        let mut c = std::process::Command::new("cmd");
        c.arg("/C").arg(&resolved);
        c.args(args);
        c
    } else {
        let mut c = std::process::Command::new(&resolved);
        c.args(args);
        c
    };
    let status = cmd
        .current_dir(dir)
        .status()
        .with_context(|| format!("failed to spawn `{prog}`"))?;
    if !status.success() {
        bail!("`{} {}` failed with {status}", prog, args.join(" "));
    }
    Ok(())
}

/// True when the Dart outputs codegen produces already exist:
/// `oxide.oxide.g.dart` next to every `oxide.dart`, plus at least one FRB
/// bridge file. Used to detect half-finished runs that need codegen repair.
fn generated_outputs_present(root: &Path) -> bool {
    let oxide_g = root.join("lib/src/oxide.oxide.g.dart");
    if !oxide_g.is_file() && root.join("lib/src/oxide.dart").is_file() {
        return false;
    }
    let api = root.join("lib/src/rust/api");
    oxide_g.is_file()
        && (api.join("bridge.dart").is_file() || api.join("oxide_bridge.dart").is_file())
}

/// Ordered codegen: `flutter pub get` -> FRB `generate` (with config file) ->
/// `build_runner build -d`.
fn run_codegen(root: &Path) -> Result<()> {
    run_cmd("flutter", &["pub", "get"], root)?;
    run_cmd(
        "flutter_rust_bridge_codegen",
        &["generate", "--config-file", "flutter_rust_bridge.yaml"],
        root,
    )?;
    // The oxide builders read `rust/target/oxide_routes/*.json` via `dart:io`,
    // invisible to build_runner's asset graph: their outputs would otherwise be
    // wrongly cached after route changes. Drop oxide-owned glue first so the
    // builders rerun (other builders stay incremental).
    //
    // The drop happens before anything has regenerated the files, and the
    // outputs are gitignored, so a failed build would otherwise leave the
    // project with no glue and no way to recover. Snapshot first and put it
    // back if the build fails.
    let snapshot = GlueSnapshot::capture(root);
    nuke_oxide_glue(root);
    if let Err(e) = run_cmd(
        "dart",
        &[
            "run",
            "build_runner",
            "build",
            "--delete-conflicting-outputs",
        ],
        root,
    ) {
        if !snapshot.is_empty() {
            snapshot.restore();
            eprintln!(
                "warning: codegen failed; restored {} previously generated file(s)",
                snapshot.len()
            );
        }
        return Err(e);
    }
    Ok(())
}

/// Best-effort command: warn instead of failing.
fn run_cmd_warn(prog: &str, args: &[&str], dir: &Path) {
    if let Err(e) = run_cmd(prog, args, dir) {
        eprintln!("warning: {e:#}");
    }
}

fn ensure_yes(yes: bool, msg: &str) -> Result<()> {
    if yes {
        return Ok(());
    }
    use std::io::{IsTerminal, Write};
    if !std::io::stdin().is_terminal() {
        bail!("{msg} (pass --yes in non-interactive shells)");
    }
    print!("{msg} [y/N]: ");
    std::io::stdout().flush().ok();
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;
    if line.trim().eq_ignore_ascii_case("y") || line.trim().eq_ignore_ascii_case("yes") {
        Ok(())
    } else {
        bail!("aborted by user (pass --yes to confirm)")
    }
}

// ---------------------------------------------------------------------------
// pubspec.yaml patching (format-preserving line surgery + serde_yaml checks)
// ---------------------------------------------------------------------------

/// Insertion lines for `oxide_runtime` / `oxide_generator` per source.
/// Each inner vec is one entry; lines after the first are extra-indented.
fn oxide_pubspec_entries(source: &OxideSourceKind, app_dir: &Path) -> (Vec<String>, Vec<String>) {
    match source {
        OxideSourceKind::Pub => (
            vec![format!("oxide_runtime: ^{OXIDE_VERSION}")],
            vec![format!("oxide_generator: ^{OXIDE_VERSION}")],
        ),
        OxideSourceKind::Path { root } => (
            vec![
                "oxide_runtime:".to_string(),
                format!(
                    "  path: {}",
                    relative_forward(&app_dir.join("dummy"), &root.join("flutter/oxide_runtime"))
                ),
            ],
            vec![
                "oxide_generator:".to_string(),
                format!(
                    "  path: {}",
                    relative_forward(
                        &app_dir.join("dummy"),
                        &root.join("flutter/oxide_generator")
                    )
                ),
            ],
        ),
        OxideSourceKind::Git { url, tag } => (
            vec![
                "oxide_runtime:".to_string(),
                "  git:".to_string(),
                format!("    url: {url}"),
                format!("    ref: {tag}"),
                "    path: flutter/oxide_runtime".to_string(),
            ],
            vec![
                "oxide_generator:".to_string(),
                "  git:".to_string(),
                format!("    url: {url}"),
                format!("    ref: {tag}"),
                "    path: flutter/oxide_generator".to_string(),
            ],
        ),
    }
}

fn yaml_has_key(doc: &serde_yaml::Value, section: &str, key: &str) -> bool {
    doc.get(section)
        .and_then(|v| v.as_mapping())
        .is_some_and(|m| m.contains_key(serde_yaml::Value::String(key.to_string())))
}

fn top_key(entry_lines: &[String]) -> String {
    entry_lines[0]
        .split(':')
        .next()
        .unwrap_or_default()
        .trim()
        .to_string()
}

/// Insert entries (each a block of lines; first line at 2-space indent,
/// continuation lines already indented) at the end of top-level `section:`.
fn insert_into_section(orig: &str, section: &str, entries: &[Vec<String>]) -> String {
    let eol = detect_eol(orig);
    let header = format!("{section}:");
    let raw: Vec<&str> = orig.lines().collect();
    // Locate the section header at indent 0.
    let mut sec_idx = None;
    for (i, l) in raw.iter().enumerate() {
        if !l.starts_with(|c: char| c.is_whitespace()) && l.trim_end() == header {
            sec_idx = Some(i);
            break;
        }
    }
    let mut out: Vec<String> = raw.iter().map(|s| s.to_string()).collect();
    let mut block: Vec<String> = Vec::new();
    for entry in entries {
        block.push(format!("  {}", entry[0]));
        for cont in entry.iter().skip(1) {
            block.push(format!("  {cont}"));
        }
    }
    match sec_idx {
        None => {
            if !out.is_empty() && !out.last().is_some_and(|l| l.trim().is_empty()) {
                out.push(String::new());
            }
            out.push(header);
            out.extend(block);
        }
        Some(si) => {
            // End of block: next non-blank, non-comment line at indent 0.
            let mut j = si + 1;
            while j < raw.len() {
                let t = raw[j].trim();
                if t.is_empty() || t.starts_with('#') {
                    j += 1;
                    continue;
                }
                if !raw[j].starts_with(|c: char| c.is_whitespace()) {
                    break;
                }
                j += 1;
            }
            // Step back over trailing blanks/comments so insertion hugs content.
            let mut k = j;
            while k > si + 1 {
                let t = raw[k - 1].trim();
                if t.is_empty() || t.starts_with('#') {
                    k -= 1;
                } else {
                    break;
                }
            }
            let mut merged: Vec<String> = out[..k].to_vec();
            merged.extend(block);
            merged.extend(out[k..].iter().cloned());
            out = merged;
        }
    }
    let mut s = out.join(eol);
    s.push_str(eol);
    s
}

/// Patch pubspec.yaml content: ensure Oxide + backend deps. Returns `None`
/// when nothing needs to change.
pub fn patch_pubspec(
    orig: &str,
    backend: BackendKind,
    source: &OxideSourceKind,
    app_dir: &Path,
) -> Result<Option<String>> {
    // A pubspec that does not parse is not a pubspec with nothing to do. The
    // previous code substituted `Value::Null` and carried on, so every key
    // looked absent, the line surgery ran on garbage, the post-validation
    // re-parse failed, and the function returned `None` — which the caller
    // reports as "already integrated" and exits 0, having silently touched
    // none of the dependencies.
    let doc: serde_yaml::Value = serde_yaml::from_str(orig).map_err(|e| {
        anyhow::anyhow!(
            "pubspec.yaml is not valid YAML, refusing to patch it: {e}. \
             Fix the YAML and re-run, or restore it from \
             .oxide/backups/ if a previous run modified it."
        )
    })?;
    let (runtime_entry, generator_entry) = oxide_pubspec_entries(source, app_dir);
    let mut missing_deps: Vec<Vec<String>> = Vec::new();
    if !yaml_has_key(&doc, "dependencies", "flutter_rust_bridge") {
        missing_deps.push(vec!["flutter_rust_bridge: 2.12.0".to_string()]);
    }
    if !yaml_has_key(&doc, "dependencies", "oxide_runtime") {
        missing_deps.push(runtime_entry);
    }
    // FRB requires these for scanned enums/structs once `rust_input: crate`
    // exposes routes + state types.
    if !yaml_has_key(&doc, "dependencies", "freezed_annotation") {
        missing_deps.push(vec!["freezed_annotation: ^3.0.0".to_string()]);
    }
    for (key, entry) in backend.dep_entries() {
        if !yaml_has_key(&doc, "dependencies", &key) {
            missing_deps.push(entry);
        }
    }
    let mut missing_dev: Vec<Vec<String>> = Vec::new();
    if !yaml_has_key(&doc, "dev_dependencies", "build_runner") {
        missing_dev.push(vec!["build_runner: ^2.4.13".to_string()]);
    }
    if !yaml_has_key(&doc, "dev_dependencies", "oxide_generator") {
        missing_dev.push(generator_entry);
    }
    if !yaml_has_key(&doc, "dev_dependencies", "freezed") {
        missing_dev.push(vec!["freezed: ^3.0.0".to_string()]);
    }
    if missing_deps.is_empty() && missing_dev.is_empty() {
        return Ok(None);
    }
    let mut out = orig.to_string();
    if !missing_deps.is_empty() {
        out = insert_into_section(&out, "dependencies", &missing_deps);
    }
    if !missing_dev.is_empty() {
        out = insert_into_section(&out, "dev_dependencies", &missing_dev);
    }
    if out == *orig {
        return Ok(None);
    }
    // The edited text must still be valid YAML. If the insertion corrupted it,
    // that is a real failure to report, not a reason to quietly do nothing.
    let check: serde_yaml::Value = serde_yaml::from_str(&out)
        .context("internal error: patching pubspec.yaml produced invalid YAML")?;
    for entry in missing_deps.iter().chain(missing_dev.iter()) {
        let key = top_key(entry);
        if !yaml_has_key(&check, "dependencies", &key)
            && !yaml_has_key(&check, "dev_dependencies", &key)
        {
            bail!(
                "internal error: failed to insert `{key}` into pubspec.yaml; \
                 leaving the file untouched"
            );
        }
    }
    Ok(Some(out))
}

// ---------------------------------------------------------------------------
// rust/Cargo.toml patching via toml_edit
// ---------------------------------------------------------------------------

fn inline_dep(version: &str, path: Option<&str>, git: Option<(&str, &str)>) -> toml_edit::Item {
    let mut t = toml_edit::InlineTable::new();
    t.insert("version", toml_edit::Value::from(version));
    if let Some(p) = path {
        t.insert("path", toml_edit::Value::from(p));
    }
    if let Some((url, tag)) = git {
        t.insert("git", toml_edit::Value::from(url));
        t.insert("tag", toml_edit::Value::from(tag));
    }
    toml_edit::Item::Value(toml_edit::Value::InlineTable(t))
}

fn ensure_str_array(table: &mut toml_edit::Item, key: &str, values: &[&str]) {
    if !table.is_table_like() {
        return;
    }
    if table.get(key).is_none() {
        let mut arr = toml_edit::Array::new();
        for v in values {
            arr.push(*v);
        }
        table[key] = toml_edit::Item::Value(toml_edit::Value::Array(arr));
        return;
    }
    if let Some(arr) = table[key].as_array_mut() {
        for v in values {
            if !arr.iter().any(|e| e.as_str() == Some(v)) {
                arr.push(*v);
            }
        }
    }
}

/// Patch `rust/Cargo.toml`: ensure Oxide + support deps, features, and the
/// FRB `check-cfg` lint. Returns `None` when nothing needs to change.
pub fn patch_cargo_toml(
    orig: &str,
    oxide_path_root: Option<&Path>,
    rust_dir: &Path,
    git: Option<(&str, &str)>,
) -> Result<Option<String>> {
    let mut doc: toml_edit::DocumentMut =
        orig.parse().context("failed to parse rust/Cargo.toml")?;
    if doc.contains_key("dependencies") && !doc["dependencies"].is_table_like() {
        bail!("rust/Cargo.toml has a non-table [dependencies]");
    }
    if !doc.contains_key("dependencies") {
        doc["dependencies"] = toml_edit::Item::Table(toml_edit::Table::new());
    }
    let (core_path, gen_path) = match oxide_path_root {
        Some(root) => (
            Some(relative_forward(
                &rust_dir.join("Cargo.toml"),
                &root.join("rust/oxide_core"),
            )),
            Some(relative_forward(
                &rust_dir.join("Cargo.toml"),
                &root.join("rust/oxide_generator_rs"),
            )),
        ),
        None => (None, None),
    };
    if doc["dependencies"].get("oxide_core").is_none() {
        doc["dependencies"]["oxide_core"] = inline_dep(OXIDE_VERSION, core_path.as_deref(), git);
    }
    if doc["dependencies"].get("oxide_generator_rs").is_none() {
        doc["dependencies"]["oxide_generator_rs"] =
            inline_dep(OXIDE_VERSION, gen_path.as_deref(), git);
    }
    for snippet in [
        "serde = { version = \"1\", features = [\"derive\"] }",
        "serde_json = \"1\"",
        "tokio-stream = { version = \"0.1\", features = [\"sync\"] }",
    ] {
        let key = snippet
            .split('=')
            .next()
            .unwrap_or_default()
            .trim()
            .to_string();
        if doc["dependencies"].get(&key).is_none() {
            let mut frag: toml_edit::DocumentMut =
                snippet.parse().context("failed to parse dep snippet")?;
            if let Some(item) = frag.remove(&key) {
                doc["dependencies"][&key] = item;
            }
        }
    }
    // Target-specific tokio deps.
    let tokio_variants = [
        (
            "cfg(not(target_arch = \"wasm32\"))",
            vec!["macros", "rt-multi-thread", "sync", "time"],
        ),
        (
            "cfg(all(target_arch = \"wasm32\", target_os = \"unknown\"))",
            vec!["sync", "rt"],
        ),
        (
            "cfg(all(target_arch = \"wasm32\", target_os = \"wasi\"))",
            vec!["sync", "rt", "time"],
        ),
    ];
    for (target, features) in &tokio_variants {
        let has_tokio = doc
            .get("target")
            .and_then(|t| t.get(*target))
            .and_then(|t| t.get("dependencies"))
            .and_then(|t| t.get("tokio"))
            .is_some();
        if has_tokio {
            continue;
        }
        let feats = features
            .iter()
            .map(|f| format!("\"{f}\""))
            .collect::<Vec<_>>()
            .join(", ");
        let frag_src = format!(
            "[target.'{target}'.dependencies]\ntokio = {{ version = \"1\", features = [{feats}] }}\n"
        );
        let frag: toml_edit::DocumentMut = frag_src.parse()?;
        if let Some(item) = frag
            .get("target")
            .and_then(|t| t.get(*target))
            .and_then(|t| t.get("dependencies"))
            .and_then(|t| t.get("tokio"))
            .cloned()
        {
            if doc.get("target").is_none() {
                doc["target"] = toml_edit::Item::Table(toml_edit::Table::new());
            }
            if doc["target"].get(*target).is_none() {
                doc["target"][*target] = toml_edit::Item::Table(toml_edit::Table::new());
            }
            if doc["target"][*target].get("dependencies").is_none() {
                doc["target"][*target]["dependencies"] =
                    toml_edit::Item::Table(toml_edit::Table::new());
            }
            doc["target"][*target]["dependencies"]["tokio"] = item;
        }
    }
    // Features.
    if !doc.contains_key("features") {
        doc["features"] = toml_edit::Item::Table(toml_edit::Table::new());
    }
    {
        let feats = &mut doc["features"];
        ensure_str_array(
            feats,
            "default",
            &["navigation-binding", "isolated-channels"],
        );
        if feats.get("navigation-binding").is_none() {
            let mut arr = toml_edit::Array::new();
            arr.push("oxide_core/navigation-binding");
            arr.push("oxide_generator_rs/navigation-binding");
            feats["navigation-binding"] = toml_edit::Item::Value(toml_edit::Value::Array(arr));
        }
        if feats.get("isolated-channels").is_none() {
            let mut arr = toml_edit::Array::new();
            arr.push("oxide_core/isolated-channels");
            arr.push("oxide_generator_rs/isolated-channels");
            feats["isolated-channels"] = toml_edit::Item::Value(toml_edit::Value::Array(arr));
        }
    }
    // [lints.rust] unexpected_cfgs check-cfg frb_expand.
    if doc.get("lints").is_none() {
        doc["lints"] = toml_edit::Item::Table(toml_edit::Table::new());
    }
    if doc["lints"].get("rust").is_none() {
        doc["lints"]["rust"] = toml_edit::Item::Table(toml_edit::Table::new());
    }
    if doc["lints"]["rust"].get("unexpected_cfgs").is_none() {
        let frag: toml_edit::DocumentMut =
            "[lints.rust]\nunexpected_cfgs = { level = \"warn\", check-cfg = ['cfg(frb_expand)'] }\n"
                .parse()?;
        if let Some(item) = frag
            .get("lints")
            .and_then(|t| t.get("rust"))
            .and_then(|t| t.get("unexpected_cfgs"))
            .cloned()
        {
            doc["lints"]["rust"]["unexpected_cfgs"] = item;
        }
    }
    let new = doc.to_string();
    if new == *orig {
        return Ok(None);
    }
    Ok(Some(new))
}

// ---------------------------------------------------------------------------
// Template rendering (subset of the oxide_app template dir)
// ---------------------------------------------------------------------------

static TEMPLATES: include_dir::Dir<'_> = include_dir::include_dir!("templates/oxide_app");

fn template_content(name: &str) -> Result<String> {
    TEMPLATES
        .get_file(name)
        .with_context(|| format!("template `{name}` missing from oxide_app dir"))?
        .contents_utf8()
        .map(|s| s.to_string())
        .with_context(|| format!("template `{name}` is not valid UTF-8"))
}

/// Substitute template placeholders for `integrate`.
///
/// Takes the values positionally rather than a struct because `integrate`
/// already threads them through `render` from several call sites.
#[allow(clippy::too_many_arguments)]
fn apply_vars(
    content: &str,
    app_name: &str,
    rust_crate: &str,
    backend: BackendKind,
    page_class: &str,
    rt_dep: &str,
    gen_dep: &str,
    core_dep: &str,
    gen_rs_dep: &str,
) -> String {
    // `integrate` does not render pubspec.yaml (it patches the existing one by
    // line surgery), so `backend_deps` is intentionally not supplied here.
    Vars::new()
        .with("app_name", app_name)
        .with("rust_crate", rust_crate)
        .with("backend", backend.as_str())
        .with("oxide_backend", backend.oxide_backend_expr())
        .with("oxide_extra_imports", backend.oxide_extra_imports())
        .with("oxide_runtime_dep", rt_dep)
        .with("oxide_generator_dep", gen_dep)
        .with("oxide_core_dep", core_dep)
        .with("oxide_generator_rs_dep", gen_rs_dep)
        .with("page_class", page_class)
        .with("page_imports", backend.page_imports())
        .with("page_body", backend.page_body(page_class))
        .render(content)
}

#[allow(clippy::too_many_arguments)]
fn render(
    name: &str,
    app_name: &str,
    rust_crate: &str,
    backend: BackendKind,
    page_class: &str,
    rt_dep: &str,
    gen_dep: &str,
    core_dep: &str,
    gen_rs_dep: &str,
) -> Result<String> {
    Ok(apply_vars(
        &template_content(name)?,
        app_name,
        rust_crate,
        backend,
        page_class,
        rt_dep,
        gen_dep,
        core_dep,
        gen_rs_dep,
    ))
}

// ---------------------------------------------------------------------------
// Planned operations
// ---------------------------------------------------------------------------

/// A single filesystem change computed before any write.
#[derive(Debug, Clone)]
enum Op {
    /// Create a new file with this content.
    Create { rel: String, content: String },
    /// Replace an existing file's full content.
    Patch {
        rel: String,
        old: String,
        new: String,
    },
    /// Delete an FRB demo file.
    Delete { rel: String, old: String },
}

impl Op {
    fn rel(&self) -> &str {
        match self {
            Op::Create { rel, .. } | Op::Patch { rel, .. } | Op::Delete { rel, .. } => rel,
        }
    }
}

fn read_if_exists(root: &Path, rel: &str) -> Option<String> {
    std::fs::read_to_string(root.join(rel)).ok()
}

/// Compute the full op list for `root` without touching the filesystem
/// (except reads).
#[allow(clippy::too_many_arguments)]
fn plan_ops(
    root: &Path,
    app_name: &str,
    rust_crate: &str,
    backend: BackendKind,
    source: &OxideSourceKind,
    with_example: bool,
    init_main: bool,
    keep_frb_example: bool,
) -> Result<Vec<Op>> {
    let mut ops = Vec::new();
    let (rt_dep, gen_dep) = match source {
        OxideSourceKind::Pub => (
            format!("  oxide_runtime: ^{OXIDE_VERSION}\n"),
            format!("  oxide_generator: ^{OXIDE_VERSION}\n"),
        ),
        OxideSourceKind::Path { root: co } => (
            format!(
                "  oxide_runtime:\n    path: {}\n",
                relative_forward(&root.join("dummy"), &co.join("flutter/oxide_runtime"))
            ),
            format!(
                "  oxide_generator:\n    path: {}\n",
                relative_forward(&root.join("dummy"), &co.join("flutter/oxide_generator"))
            ),
        ),
        OxideSourceKind::Git { url, tag } => (
            format!(
                "  oxide_runtime:\n    git:\n      url: {url}\n      ref: {tag}\n      path: flutter/oxide_runtime\n"
            ),
            format!(
                "  oxide_generator:\n    git:\n      url: {url}\n      ref: {tag}\n      path: flutter/oxide_generator\n"
            ),
        ),
    };
    let (core_dep, gen_rs_dep) = match source {
        OxideSourceKind::Pub => (
            format!("oxide_core = {{ version = \"{OXIDE_VERSION}\" }}"),
            format!("oxide_generator_rs = {{ version = \"{OXIDE_VERSION}\" }}"),
        ),
        OxideSourceKind::Path { root: co } => (
            format!(
                "oxide_core = {{ version = \"{OXIDE_VERSION}\", path = \"{}\" }}",
                relative_forward(&root.join("rust/Cargo.toml"), &co.join("rust/oxide_core"))
            ),
            format!(
                "oxide_generator_rs = {{ version = \"{OXIDE_VERSION}\", path = \"{}\" }}",
                relative_forward(
                    &root.join("rust/Cargo.toml"),
                    &co.join("rust/oxide_generator_rs")
                )
            ),
        ),
        OxideSourceKind::Git { url, tag } => (
            format!(
                "oxide_core = {{ version = \"{OXIDE_VERSION}\", git = \"{url}\", tag = \"{tag}\" }}"
            ),
            format!(
                "oxide_generator_rs = {{ version = \"{OXIDE_VERSION}\", git = \"{url}\", tag = \"{tag}\" }}"
            ),
        ),
    };
    let render_file = |name: &str, page_class: &str| {
        render(
            name,
            app_name,
            rust_crate,
            backend,
            page_class,
            &rt_dep,
            &gen_dep,
            &core_dep,
            &gen_rs_dep,
        )
    };

    // --- Rust layer ---
    let state_files = [
        ("rust/src/state/mod.rs", "rust/src/state/mod.rs"),
        ("rust/src/state/app_state.rs", "rust/src/state/app_state.rs"),
        (
            "rust/src/state/app_action.rs",
            "rust/src/state/app_action.rs",
        ),
    ];
    for (rel, tmpl) in state_files {
        if read_if_exists(root, rel).is_none() {
            ops.push(Op::Create {
                rel: rel.to_string(),
                content: render_file(tmpl, "OxideDemoPage")?,
            });
        }
    }
    // Starter route: apps without any #[oxide_route] need one. Without it the
    // navigation builder emits glue referencing FRB route files that cannot
    // exist (empty `routes {}` produces an invalid empty Dart enum).
    if !has_oxide_routes(root) {
        for (rel, tmpl) in [
            ("rust/src/routes/mod.rs", "rust/src/routes/mod.rs"),
            (
                "rust/src/routes/home_route.rs",
                "rust/src/routes/home_route.rs",
            ),
        ] {
            if read_if_exists(root, rel).is_none() {
                ops.push(Op::Create {
                    rel: rel.to_string(),
                    content: render_file(tmpl, "OxideDemoPage")?,
                });
            }
        }
    }
    // Bridge: prefer existing bridge.rs, else oxide_bridge.rs, else create it.
    let bridge_mod: &str;
    if root.join("rust/src/api/bridge.rs").is_file() {
        bridge_mod = "bridge";
    } else if root.join("rust/src/api/oxide_bridge.rs").is_file() {
        bridge_mod = "oxide_bridge";
    } else {
        bridge_mod = "oxide_bridge";
        ops.push(Op::Create {
            rel: "rust/src/api/oxide_bridge.rs".to_string(),
            content: render_file("rust/src/api/bridge.rs", "OxideDemoPage")?,
        });
    }
    // Whether the FRB `pub mod simple;` entry must go (unmodified demo).
    let remove_simple_mod = !keep_frb_example
        && root.join("rust/src/api/simple.rs").is_file()
        && std::fs::read_to_string(root.join("rust/src/api/simple.rs"))
            .map(|c| is_frb_demo_rust_api("simple.rs", &c))
            .unwrap_or(false);
    match read_if_exists(root, "rust/src/api/mod.rs") {
        None => {
            let mut content = render_file("rust/src/api/mod.rs", "OxideDemoPage")?;
            if bridge_mod == "oxide_bridge" {
                content = content.replace("pub mod bridge;", "pub mod oxide_bridge;");
            }
            if remove_simple_mod {
                content = strip_simple_mod_line(&content);
            }
            ops.push(Op::Create {
                rel: "rust/src/api/mod.rs".to_string(),
                content,
            });
        }
        Some(orig) => {
            let mut base = orig.clone();
            let mut need_patch = false;
            if let Some(patched) = patch_api_mod(&orig, bridge_mod) {
                base = patched;
                need_patch = true;
            }
            if remove_simple_mod && base.lines().any(|l| l.trim() == "pub mod simple;") {
                base = strip_simple_mod_line(&base);
                need_patch = true;
            }
            if need_patch {
                ops.push(Op::Patch {
                    rel: "rust/src/api/mod.rs".to_string(),
                    old: orig.clone(),
                    new: to_eol(&base, detect_eol(&orig)),
                });
            }
        }
    }
    if let Some(orig) = read_if_exists(root, "rust/src/lib.rs") {
        if let Some(new) = patch_lib_rs(&orig) {
            ops.push(Op::Patch {
                rel: "rust/src/lib.rs".to_string(),
                old: orig.clone(),
                new: to_eol(&new, detect_eol(&orig)),
            });
        }
    }
    if let Some(orig) = read_if_exists(root, "rust/Cargo.toml") {
        let (path_root, git) = match source {
            OxideSourceKind::Path { root: co } => (Some(co.as_path()), None),
            OxideSourceKind::Git { url, tag } => (None, Some((url.as_str(), tag.as_str()))),
            OxideSourceKind::Pub => (None, None),
        };
        if let Some(new) = patch_cargo_toml(&orig, path_root, &root.join("rust"), git)? {
            ops.push(Op::Patch {
                rel: "rust/Cargo.toml".to_string(),
                old: orig.clone(),
                new: to_eol(&new, detect_eol(&orig)),
            });
        }
    }

    // --- Flutter layer ---
    if read_if_exists(root, "lib/src/oxide.dart").is_none() {
        let mut oxide_dart = render_file("lib/src/oxide.dart", "OxideDemoPage")?;
        if bridge_mod == "oxide_bridge" {
            oxide_dart = oxide_dart.replace("rust/api/bridge.dart", "rust/api/oxide_bridge.dart");
        }
        ops.push(Op::Create {
            rel: "lib/src/oxide.dart".to_string(),
            content: oxide_dart,
        });
    }
    if with_example && read_if_exists(root, "lib/src/oxide_demo_page.dart").is_none() {
        let mut content = render_file("lib/src/counter_page.dart", "OxideDemoPage")?;
        ops.push(Op::Create {
            rel: "lib/src/oxide_demo_page.dart".to_string(),
            content: std::mem::take(&mut content),
        });
        if read_if_exists(root, "integration_test/oxide_test.dart").is_none()
            && root.join("integration_test").is_dir()
        {
            let test = render_file("integration_test/oxide_test.dart", "OxideDemoPage")?
                .replace("src/counter_page.dart", "src/oxide_demo_page.dart")
                .replace("CounterPage", "OxideDemoPage");
            ops.push(Op::Create {
                rel: "integration_test/oxide_test.dart".to_string(),
                content: test,
            });
        }
    }
    if let Some(orig) = read_if_exists(root, "pubspec.yaml") {
        if let Some(new) = patch_pubspec(&orig, backend, source, root)? {
            ops.push(Op::Patch {
                rel: "pubspec.yaml".to_string(),
                old: orig.clone(),
                new: to_eol(&new, detect_eol(&orig)),
            });
        }
    }
    // main.dart init patch (default on). An unmodified FRB demo main is
    // replaced with the starter entrypoint below instead of patched.
    if init_main {
        if let Some(orig) = read_if_exists(root, "lib/main.dart") {
            if with_example && is_frb_demo_dart_main(&orig) {
                // Handled by the demo replacement below.
            } else {
                match patch_main_dart(&orig) {
                    MainPatch::Patched(new) => ops.push(Op::Patch {
                        rel: "lib/main.dart".to_string(),
                        old: orig,
                        new,
                    }),
                    MainPatch::AlreadyPatched => {}
                    MainPatch::PatternMiss => {
                        eprintln!("warning: lib/main.dart pattern not recognized; left untouched.");
                        eprintln!("{MAIN_SNIPPET}");
                    }
                }
            }
        }
    }
    // FRB scan scope: Oxide needs `rust_input: crate`. FRB's `integrate`
    // default (`crate::api`) hides the `#[routes]` output and the
    // `OxideError` re-export from codegen, breaking the navigation glue.
    if let Some(orig) = read_if_exists(root, "flutter_rust_bridge.yaml") {
        if let Some(new) = patch_frb_rust_input(&orig) {
            ops.push(Op::Patch {
                rel: "flutter_rust_bridge.yaml".to_string(),
                old: orig.clone(),
                new: to_eol(&new, detect_eol(&orig)),
            });
        }
    }
    // .gitignore additions merge.
    {
        let additions = template_content("gitignore.additions")?;
        let existing = read_if_exists(root, ".gitignore");
        let merged = merge_gitignore(existing.as_deref(), &additions);
        match existing {
            None => ops.push(Op::Create {
                rel: ".gitignore".to_string(),
                content: merged,
            }),
            Some(old) => {
                if merged != old {
                    ops.push(Op::Patch {
                        rel: ".gitignore".to_string(),
                        old,
                        new: merged,
                    });
                }
            }
        }
    }
    // FRB demo cleanup by signature.
    if !keep_frb_example {
        // The FRB scaffold also emits generated Dart for its demo API before
        // we delete the Rust sources (FRB never prunes orphans). Drop those
        // only together with the unmodified Rust demo.
        let rust_demo_gone = read_if_exists(root, "rust/src/api/simple.rs")
            .is_some_and(|c| is_frb_demo_rust_api("simple.rs", &c));
        let demo_candidates = [
            ("rust/src/api/simple.rs", "simple.rs", 0),
            ("integration_test/simple_test.dart", "simple_test.dart", 2),
            ("lib/src/rust/api/simple.dart", "simple.dart", 3),
            (
                "lib/src/rust/api/simple.freezed.dart",
                "simple.freezed.dart",
                3,
            ),
        ];
        for (rel, name, kind) in demo_candidates {
            if let Some(old) = read_if_exists(root, rel) {
                // Skip files we are already patching (demo main handled below).
                if ops.iter().any(|o| o.rel() == rel) {
                    continue;
                }
                let is_demo = match kind {
                    0 => is_frb_demo_rust_api(name, &old),
                    2 => is_frb_demo_integration_test(name, &old),
                    // Generated Dart for the demo API: drop it with the
                    // unmodified Rust demo, or when it is an orphan (demo
                    // content but the Rust source is already gone — FRB never
                    // prunes these, and they reference deleted APIs).
                    _ => {
                        old.contains("greet")
                            && (rust_demo_gone || !root.join("rust/src/api/simple.rs").is_file())
                    }
                };
                if is_demo {
                    ops.push(Op::Delete {
                        rel: rel.to_string(),
                        old,
                    });
                }
            }
        }
        // An unmodified FRB demo main is replaced with the starter entrypoint
        // (deleting it would leave the app without `main`). User-modified
        // mains are patched above and never touched here.
        if with_example {
            if let Some(old) = read_if_exists(root, "lib/main.dart") {
                if !ops.iter().any(|o| o.rel() == "lib/main.dart") && is_frb_demo_dart_main(&old) {
                    ops.push(Op::Patch {
                        rel: "lib/main.dart".to_string(),
                        old,
                        new: STARTER_MAIN_DART.to_string(),
                    });
                    println!("replacing FRB demo lib/main.dart with the oxide starter entrypoint");
                }
            } else if !ops.iter().any(|o| o.rel() == "lib/main.dart") {
                ops.push(Op::Create {
                    rel: "lib/main.dart".to_string(),
                    content: STARTER_MAIN_DART.to_string(),
                });
                println!("creating missing lib/main.dart with the oxide starter entrypoint");
            }
        }
    }
    Ok(ops)
}

/// True when the app already declares at least one `#[oxide_route]`.
fn has_oxide_routes(root: &Path) -> bool {
    let dir = root.join("rust/src/routes");
    let Ok(entries) = std::fs::read_dir(&dir) else {
        return false;
    };
    entries
        .flatten()
        .filter(|e| e.path().extension().is_some_and(|x| x == "rs"))
        .any(|e| std::fs::read_to_string(e.path()).is_ok_and(|c| c.contains("oxide_route")))
}

/// Ensure FRB scans the whole crate (`rust_input: crate`).
/// Returns `None` when already correct or no `rust_input` line exists.
pub fn patch_frb_rust_input(content: &str) -> Option<String> {
    let mut changed = false;
    let out: Vec<String> = content
        .lines()
        .map(|l| {
            let t = l.trim_start();
            if t.starts_with("rust_input:") {
                let v = t
                    .strip_prefix("rust_input:")
                    .unwrap_or(t)
                    .trim()
                    .trim_matches('"')
                    .trim_matches('\'');
                if v != "crate" {
                    changed = true;
                    let indent = &l[..l.len() - t.len()];
                    return format!("{indent}rust_input: crate");
                }
            }
            l.to_string()
        })
        .collect();
    if !changed {
        return None;
    }
    let mut s = out.join("\n");
    if content.ends_with('\n') {
        s.push('\n');
    }
    Some(s)
}

/// Drop `pub mod simple;` lines (FRB demo cleanup), preserving the trailing
/// newline style of `content`.
fn strip_simple_mod_line(content: &str) -> String {
    let filtered: Vec<&str> = content
        .lines()
        .filter(|l| l.trim() != "pub mod simple;")
        .collect();
    let mut out = filtered.join("\n");
    if content.ends_with('\n') && !out.is_empty() {
        out.push('\n');
    }
    out
}

// ---------------------------------------------------------------------------
// run()
// ---------------------------------------------------------------------------

fn print_diff(rel: &str, old: &str, new: &str) {
    use similar::TextDiff;
    println!("--- {rel}");
    let diff = TextDiff::from_lines(old, new);
    print!("{}", diff.unified_diff());
}

fn app_name_from_pubspec(root: &Path) -> String {
    std::fs::read_to_string(root.join("pubspec.yaml"))
        .ok()
        .and_then(|s| serde_yaml::from_str::<serde_yaml::Value>(&s).ok())
        .and_then(|v| v.get("name")?.as_str().map(|s| s.to_string()))
        .unwrap_or_else(|| {
            root.file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_else(|| "app".to_string())
        })
}

fn rust_crate_from_manifest(root: &Path, app_name: &str) -> String {
    std::fs::read_to_string(root.join("rust/Cargo.toml"))
        .ok()
        .and_then(|s| s.parse::<toml_edit::DocumentMut>().ok())
        .and_then(|d| {
            d.get("package")?
                .get("name")?
                .as_str()
                .map(|s| s.to_string())
        })
        .unwrap_or_else(|| format!("rust_lib_{app_name}"))
}

/// Execute `oxide integrate`.
pub fn run(args: IntegrateArgs) -> Result<IntegrateReport> {
    let cwd = std::env::current_dir().context("failed to read current dir")?;
    let root = if args.path.as_os_str().is_empty() {
        cwd
    } else {
        cwd.join(&args.path)
    };
    let root = root.canonicalize().unwrap_or(root);
    if !root.join("pubspec.yaml").is_file() {
        bail!("no pubspec.yaml in {} (not a Flutter app?)", root.display());
    }
    let backend = parse_backend(&args.backend)?;
    let source = match &args.oxide_path {
        Some(p) => OxideSourceKind::Path {
            root: p.canonicalize().unwrap_or_else(|_| p.clone()),
        },
        None => resolve_oxide_source(&args.oxide_source, &root)?,
    };
    let with_example = match args.example.as_str() {
        "counter" => true,
        "none" => false,
        other => bail!("unknown --example `{other}` (expected counter|none)"),
    };
    let init_main = !args.no_init_main;

    let mut shape = detect_shape(&root);
    // FlutterNoFrb: run `frb integrate` (backing up main.dart first), then
    // restore main.dart and continue as FrbApp.
    if shape == ProjectShape::FlutterNoFrb && !args.dry_run && !args.skip_frb {
        let main = root.join("lib/main.dart");
        let mains: Vec<PathBuf> = if main.is_file() {
            vec![main.clone()]
        } else {
            vec![]
        };
        let backup_dir = if mains.is_empty() {
            None
        } else {
            ensure_yes(
                args.yes,
                "Back up lib/main.dart and run `flutter_rust_bridge_codegen integrate`?",
            )?;
            Some(backup_files(&root, &mains)?)
        };
        run_cmd("flutter_rust_bridge_codegen", &["integrate"], &root)?;
        if let Some(dir) = backup_dir {
            let saved = dir.join("lib/main.dart");
            if saved.is_file() {
                std::fs::copy(&saved, &main)
                    .with_context(|| format!("failed to restore {}", main.display()))?;
                println!("restored lib/main.dart from backup (FRB integrate modifies it)");
            }
        }
        shape = detect_shape(&root);
        if shape == ProjectShape::FlutterNoFrb {
            shape = ProjectShape::FrbApp; // integrate ran; proceed additively anyway
        }
    }

    let app_name = app_name_from_pubspec(&root);
    let rust_crate = rust_crate_from_manifest(&root, &app_name);
    let ops = plan_ops(
        &root,
        &app_name,
        &rust_crate,
        backend,
        &source,
        with_example,
        init_main,
        args.keep_frb_example,
    )?;

    if args.dry_run {
        println!(
            "integrate plan for {} (shape: {}):",
            root.display(),
            shape.as_str()
        );
        if ops.is_empty() {
            println!("  no changes (already integrated)");
        }
        for op in &ops {
            match op {
                Op::Create { rel, content } => {
                    println!("  create {rel} ({} lines)", content.lines().count());
                }
                Op::Patch { rel, old, new } => print_diff(rel, old, new),
                Op::Delete { rel, old } => {
                    println!("  delete {rel} ({} lines, FRB demo)", old.lines().count());
                }
            }
        }
        let mut changed: Vec<String> = ops.iter().map(|o| o.rel().to_string()).collect();
        changed.sort();
        changed.dedup();
        return Ok(IntegrateReport {
            shape: shape.as_str().to_string(),
            changed,
            noop: ops.is_empty(),
        });
    }

    if ops.is_empty() {
        // No file changes, but generated outputs may still be missing or stale
        // (e.g. a previous run failed halfway through codegen). Repair those
        // instead of claiming success while the app cannot build.
        if !args.no_codegen && !args.dry_run && !generated_outputs_present(&root) {
            ensure_yes(
                args.yes,
                "Generated outputs are missing; run codegen to repair?",
            )?;
            run_codegen(&root)?;
            doctor_summary(&root);
            return Ok(IntegrateReport {
                shape: shape.as_str().to_string(),
                changed: vec!["(codegen repair)".to_string()],
                noop: false,
            });
        }
        println!("already integrated (shape: {}); no changes", shape.as_str());
        return Ok(IntegrateReport {
            shape: shape.as_str().to_string(),
            changed: Vec::new(),
            noop: true,
        });
    }

    ensure_yes(
        args.yes,
        &format!(
            "Apply {} integrate change(s) to {}?",
            ops.len(),
            root.display()
        ),
    )?;
    // Back up every existing file we will patch or delete.
    let to_backup: Vec<PathBuf> = ops
        .iter()
        .filter_map(|o| match o {
            Op::Patch { rel, .. } | Op::Delete { rel, .. } => Some(root.join(rel)),
            Op::Create { .. } => None,
        })
        .filter(|p| p.is_file())
        .collect();
    if !to_backup.is_empty() {
        let dir = backup_files(&root, &to_backup)?;
        println!("backed up {} file(s) to {}", to_backup.len(), dir.display());
    }
    let mut changed = Vec::new();
    for op in &ops {
        match op {
            Op::Create { rel, content } => {
                write_atomic(&root.join(rel), content)?;
                println!("created {rel}");
                changed.push(rel.clone());
            }
            Op::Patch { rel, new, .. } => {
                write_atomic(&root.join(rel), new)?;
                println!("patched {rel}");
                changed.push(rel.clone());
            }
            Op::Delete { rel, .. } => {
                std::fs::remove_file(root.join(rel))
                    .with_context(|| format!("failed to remove {rel}"))?;
                println!("removed FRB demo {rel}");
                changed.push(rel.clone());
            }
        }
    }

    // Format + fix (best-effort), then codegen unless skipped.
    run_cmd_warn("dart", &["fix", "--apply"], &root);
    run_cmd_warn(
        "dart",
        &["format", "lib", "test", "integration_test"],
        &root,
    );
    if !args.no_codegen {
        run_codegen(&root)?;
    }

    doctor_summary(&root);
    changed.sort();
    changed.dedup();
    Ok(IntegrateReport {
        shape: shape.as_str().to_string(),
        changed,
        noop: false,
    })
}

/// Print a short doctor-style summary of the integrated project.
pub fn doctor_summary(root: &Path) {
    let check = |label: &str, ok: bool| {
        println!("  [{}] {label}", if ok { "ok" } else { "missing" });
    };
    println!("oxide integrate summary for {}:", root.display());
    check(
        "flutter_rust_bridge.yaml",
        root.join("flutter_rust_bridge.yaml").is_file(),
    );
    check("rust/Cargo.toml", root.join("rust/Cargo.toml").is_file());
    check(
        "rust state + bridge",
        root.join("rust/src/state").is_dir()
            && (root.join("rust/src/api/bridge.rs").is_file()
                || root.join("rust/src/api/oxide_bridge.rs").is_file()),
    );
    let oxide_dart = std::fs::read_to_string(root.join("lib/src/oxide.dart")).unwrap_or_default();
    check(
        "lib/src/oxide.dart @OxideStore",
        oxide_dart.contains("@OxideStore"),
    );
    check(
        "codegen output (oxide.oxide.g.dart)",
        root.join("lib/src/oxide.oxide.g.dart").is_file(),
    );
    check(
        "codegen output (oxide_stack.g.dart)",
        root.join("lib/oxide_generated/oxide_stack.g.dart")
            .is_file(),
    );
}
