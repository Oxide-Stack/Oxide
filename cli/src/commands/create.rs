//! `oxide create` — scaffold a new Oxide + flutter_rust_bridge app.
//!
//! Wiring contract (for `src/cli.rs` / `src/main.rs`, owned by another workstream):
//! - `CreateArgs` derives `clap::Args` and can be embedded as a subcommand.
//! - Entry point: [`run`] takes `CreateArgs` by value and returns `anyhow::Result<()>`,
//!   so `Commands::Create(a)` can convert `cli::CreateArgs` field-by-field:
//!   `name`, `org`, `rust_crate_name`, `rust_crate_dir` (`None` when `"rust"`),
//!   `backend: a.backend.to_string()`, `oxide_source` (`"auto"` when
//!   `OxideSource::Pub` without `--oxide-path`, else `to_string()`),
//!   `oxide_path`, flags `keep_frb_example` / `no_codegen` / `verify` /
//!   `dry_run` / `yes`, plus `base_dir` (from global `--root`, `None` = cwd).
//! - Preflight failures (missing tools / wrong FRB version) exit the process
//!   with code [`EXIT_PREFLIGHT`] (3).
//! - Setting env `OXIDE_CLI_NO_EXTERNAL=1` skips spawning external tools
//!   (`flutter`, `cargo`, `flutter_rust_bridge_codegen`, `dart`); used by tests.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};

use crate::common::{
    self, FRB_PIN, MSRV, fs::write_atomic, gitignore::merge as merge_gitignore, template::Vars,
};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Expected `flutter_rust_bridge_codegen` version. Re-exported under its old
/// name because tests and callers referred to it that way.
pub const FRB_VERSION: &str = FRB_PIN;

/// Minimum supported Rust toolchain, from `common::MSRV`.
pub const RUST_VERSION_FLOOR: &str = MSRV;

/// Oxide crate/package version scaffolded by this CLI.
pub const OXIDE_VERSION: &str = "0.5.0";
/// Process exit code for preflight failures (missing tools / version mismatch).
pub const EXIT_PREFLIGHT: i32 = 3;
/// Upstream git URL used for `--oxide-source git`.
pub const OXIDE_GIT_URL: &str = "https://github.com/Oxide-Stack/Oxide";

// ---------------------------------------------------------------------------
// Args
// ---------------------------------------------------------------------------

/// Arguments for `oxide create`.
#[derive(Debug, Clone, clap::Args)]
pub struct CreateArgs {
    /// Application name (valid Dart package name, e.g. `my_app`).
    pub name: String,
    /// Organisation prefix passed to `flutter_rust_bridge_codegen create` (`--org`).
    #[arg(long)]
    pub org: Option<String>,
    /// Rust crate name passed to `flutter_rust_bridge_codegen create`.
    /// Defaults to `rust_lib_<name>`.
    #[arg(long)]
    pub rust_crate_name: Option<String>,
    /// Fixed at `rust/`, matching every example and the `rust_root:` key in
    /// `flutter_rust_bridge.yaml`. There is deliberately no option to change it:
    /// a flag that only reached FRB produced a second, orphaned crate alongside
    /// the template's, plus a dangling `path:` dependency, and still exited 0.
    pub rust_crate_dir: String,
    /// UI backend: `inherited` (default), `hooks`, `riverpod`, `bloc`.
    #[arg(long, default_value = "inherited")]
    pub backend: String,
    /// Oxide dependency source: `auto` (default), `pub`, `path`, `git`.
    /// `auto` uses relative paths when run inside the Oxide checkout, else pub.
    #[arg(long, default_value = "auto")]
    pub oxide_source: String,
    /// Keep the FRB demo files (`simple.rs` greet example, demo main, simple test).
    #[arg(long, default_value_t = false)]
    pub keep_frb_example: bool,
    /// Skip codegen (`flutter pub get` / FRB generate / build_runner).
    #[arg(long, default_value_t = false)]
    pub no_codegen: bool,
    /// Run `flutter analyze` + `cargo check` after scaffolding.
    #[arg(long, default_value_t = false)]
    pub verify: bool,
    /// Print the file list that would be created without writing anything.
    #[arg(long, default_value_t = false)]
    pub dry_run: bool,
    /// Skip interactive confirmations.
    #[arg(long, default_value_t = false)]
    pub yes: bool,
    /// Explicit Oxide checkout root, overriding `--oxide-source` detection.
    /// (Mirrors the `--oxide-path` global in the crate CLI surface.)
    #[arg(long)]
    pub oxide_path: Option<PathBuf>,
    /// Parent directory the new app is created in. Populated from the global
    /// `--root` flag by `src/main.rs`; `None` means the current directory.
    #[arg(skip)]
    pub base_dir: Option<PathBuf>,
}

// ---------------------------------------------------------------------------
// Backend
// ---------------------------------------------------------------------------

/// Supported UI backends.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Backend {
    Inherited,
    InheritedHooks,
    Riverpod,
    Bloc,
}

/// Parse a `--backend` flag value.
pub fn parse_backend(s: &str) -> Result<Backend> {
    match s {
        "inherited" => Ok(Backend::Inherited),
        "hooks" | "inherited-hooks" | "inheritedHooks" => Ok(Backend::InheritedHooks),
        "riverpod" => Ok(Backend::Riverpod),
        "bloc" => Ok(Backend::Bloc),
        other => bail!("unknown backend `{other}` (expected inherited|hooks|riverpod|bloc)"),
    }
}

impl std::fmt::Display for Backend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Backend {
    /// Canonical flag spelling, also used for the `{{backend}}` template var.
    pub fn as_str(self) -> &'static str {
        match self {
            Backend::Inherited => "inherited",
            Backend::InheritedHooks => "hooks",
            Backend::Riverpod => "riverpod",
            Backend::Bloc => "bloc",
        }
    }

    /// Dart `OxideBackend.*` expression for the `@OxideStore` annotation.
    pub fn oxide_backend_expr(self) -> &'static str {
        match self {
            Backend::Inherited => "OxideBackend.inherited",
            Backend::InheritedHooks => "OxideBackend.inheritedHooks",
            Backend::Riverpod => "OxideBackend.riverpod",
            Backend::Bloc => "OxideBackend.bloc",
        }
    }

    /// Extra Dart imports for `lib/src/oxide.dart` (each includes trailing newline).
    pub fn oxide_extra_imports(self) -> &'static str {
        match self {
            Backend::Inherited => "",
            Backend::InheritedHooks => "import 'package:flutter_hooks/flutter_hooks.dart';\n",
            Backend::Riverpod => "import 'package:flutter_riverpod/flutter_riverpod.dart';\n",
            Backend::Bloc => "import 'package:flutter_bloc/flutter_bloc.dart';\n",
        }
    }

    /// `dependencies:` fragment for pubspec.yaml (2-space indented, trailing newline).
    pub fn pubspec_deps(self) -> &'static str {
        match self {
            Backend::Inherited => "",
            Backend::InheritedHooks => "  flutter_hooks: ^0.20.5\n",
            Backend::Riverpod => "  flutter_riverpod: ^3.2.0\n",
            Backend::Bloc => "  bloc: ^8.1.0\n  flutter_bloc: ^8.1.3\n",
        }
    }

    /// Extra Dart imports prepended to the counter page template.
    pub fn page_imports(self) -> &'static str {
        match self {
            Backend::Inherited => "",
            Backend::InheritedHooks => "import 'package:flutter_hooks/flutter_hooks.dart';\n",
            Backend::Riverpod => "import 'package:flutter_riverpod/flutter_riverpod.dart';\n",
            Backend::Bloc => {
                "import 'package:flutter_bloc/flutter_bloc.dart';\nimport 'package:oxide_runtime/oxide_runtime.dart';\nimport 'rust/state/app_state.dart';\n"
            }
        }
    }

    /// Page widget body wiring the generated `AppStore` glue to `_CounterView`.
    ///
    /// Generated-name assumptions (mirror `examples/counter_app` with
    /// `@OxideStore(name: 'AppStore')`): scope `AppStoreScope`, cubit
    /// `AppStoreCubit`, actions `AppStoreActions`, riverpod provider
    /// `appStoreProvider`, hooks accessor `useAppStoreOxide()`.
    pub fn page_body(self, page_class: &str) -> String {
        let loading = "return const Scaffold(body: Center(child: CircularProgressIndicator()));";
        // Single `$` on purpose: this string is emitted as Dart source, and
        // `\$` would be an escaped dollar that renders the literal text
        // "${view.error}" instead of interpolating the value.
        let error = "return Scaffold(body: Center(child: Text('Error: ${view.error}')));";
        let no_state = "return const Scaffold(body: Center(child: Text('No state yet')));";
        match self {
            Backend::Inherited => format!(
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
            Backend::InheritedHooks => format!(
                r#"class {page_class} extends HookWidget {{
  const {page_class}({{super.key}});

  @override
  Widget build(BuildContext context) {{
    // `useAppStoreOxide` resolves through `AppStoreScope.useOxide`, which
    // throws if no scope is above it in the tree. The other backends supply the
    // scope themselves; hooks has to wrap explicitly.
    return AppStoreScope(
      child: Builder(
        builder: (context) {{
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
        }},
      ),
    );
  }}
}}"#
            ),
            Backend::Riverpod => format!(
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
            Backend::Bloc => format!(
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
// Oxide dependency source
// ---------------------------------------------------------------------------

/// Where Oxide dependencies come from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OxideSource {
    /// crates.io / pub.dev versions (`oxide_* 0.5.0`).
    Pub,
    /// Relative paths into the Oxide checkout rooted at `root`.
    Path { root: PathBuf },
    /// Upstream git URL.
    Git { url: String },
}

/// Find the Oxide checkout root above `cwd` (dir containing
/// `rust/oxide_core/Cargo.toml` and `flutter/oxide_runtime/pubspec.yaml`).
pub fn find_checkout_root(cwd: &Path) -> Option<PathBuf> {
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

/// Resolve `--oxide-source` (`auto`/`pub`/`path`/`git`).
pub fn resolve_oxide_source(flag: &str, cwd: &Path) -> Result<OxideSource> {
    match flag {
        "pub" => Ok(OxideSource::Pub),
        "git" => Ok(OxideSource::Git {
            url: OXIDE_GIT_URL.to_string(),
        }),
        "path" => match find_checkout_root(cwd) {
            Some(root) => Ok(OxideSource::Path { root }),
            None => bail!("--oxide-source path requires running inside the Oxide checkout"),
        },
        "auto" => Ok(match find_checkout_root(cwd) {
            Some(root) => OxideSource::Path { root },
            None => OxideSource::Pub,
        }),
        other => bail!("unknown --oxide-source `{other}` (expected auto|pub|path|git)"),
    }
}

/// Relative path from `from_dir` to `to`, using forward slashes.
pub fn relative_path(from_dir: &Path, to: &Path) -> String {
    let mut from = from_dir.components().peekable();
    let mut to_it = to.components().peekable();
    // Strip common prefix.
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

/// Resolved per-file dependency snippets.
#[derive(Debug, Clone)]
pub struct OxideDepSnippets {
    /// e.g. `  oxide_runtime: ^0.5.0\n` (already indented for pubspec).
    pub oxide_runtime_dep: String,
    /// e.g. `  oxide_generator: ^0.5.0\n` (already indented, dev_dependencies).
    pub oxide_generator_dep: String,
    /// e.g. `oxide_core = { version = "0.5.0" }`.
    pub oxide_core_dep: String,
    /// e.g. `oxide_generator_rs = { version = "0.5.0" }`.
    pub oxide_generator_rs_dep: String,
}

/// Build dependency snippets for `app_dir` given the resolved source.
pub fn dep_snippets(source: &OxideSource, app_dir: &Path) -> OxideDepSnippets {
    match source {
        OxideSource::Pub => OxideDepSnippets {
            oxide_runtime_dep: format!("  oxide_runtime: ^{OXIDE_VERSION}\n"),
            oxide_generator_dep: format!("  oxide_generator: ^{OXIDE_VERSION}\n"),
            oxide_core_dep: format!("oxide_core = {{ version = \"{OXIDE_VERSION}\" }}"),
            oxide_generator_rs_dep: format!(
                "oxide_generator_rs = {{ version = \"{OXIDE_VERSION}\" }}"
            ),
        },
        OxideSource::Path { root } => {
            let dart_rel = |pkg: &str| relative_path(app_dir, &root.join("flutter").join(pkg));
            let rust_rel =
                |pkg: &str| relative_path(&app_dir.join("rust"), &root.join("rust").join(pkg));
            OxideDepSnippets {
                oxide_runtime_dep: format!(
                    "  oxide_runtime:\n    path: {}\n",
                    dart_rel("oxide_runtime")
                ),
                oxide_generator_dep: format!(
                    "  oxide_generator:\n    path: {}\n",
                    dart_rel("oxide_generator")
                ),
                oxide_core_dep: format!(
                    "oxide_core = {{ version = \"{OXIDE_VERSION}\", path = \"{}\" }}",
                    rust_rel("oxide_core")
                ),
                oxide_generator_rs_dep: format!(
                    "oxide_generator_rs = {{ version = \"{OXIDE_VERSION}\", path = \"{}\" }}",
                    rust_rel("oxide_generator_rs")
                ),
            }
        }
        OxideSource::Git { url } => {
            let tag = format!("v{OXIDE_VERSION}");
            OxideDepSnippets {
                oxide_runtime_dep: format!(
                    "  oxide_runtime:\n    git:\n      url: {url}\n      ref: {tag}\n      path: flutter/oxide_runtime\n"
                ),
                oxide_generator_dep: format!(
                    "  oxide_generator:\n    git:\n      url: {url}\n      ref: {tag}\n      path: flutter/oxide_generator\n"
                ),
                oxide_core_dep: format!(
                    "oxide_core = {{ version = \"{OXIDE_VERSION}\", git = \"{url}\", tag = \"{tag}\" }}"
                ),
                oxide_generator_rs_dep: format!(
                    "oxide_generator_rs = {{ version = \"{OXIDE_VERSION}\", git = \"{url}\", tag = \"{tag}\" }}"
                ),
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Templates
// ---------------------------------------------------------------------------

static TEMPLATES: include_dir::Dir<'_> = include_dir::include_dir!("templates/oxide_app");

/// All template variables.
#[derive(Debug, Clone)]
pub struct TemplateVars {
    pub app_name: String,
    pub rust_crate: String,
    pub backend: Backend,
    pub page_class: String,
    pub deps: OxideDepSnippets,
}

impl TemplateVars {
    /// Project these onto the shared placeholder set.
    ///
    /// `create` supplies every placeholder in `common::template::PLACEHOLDERS`.
    /// `integrate` supplies a subset, because it patches an existing
    /// `pubspec.yaml` by line surgery rather than rendering that template.
    fn to_common(&self) -> Vars {
        Vars::new()
            .with("app_name", &self.app_name)
            .with("rust_crate", &self.rust_crate)
            .with("backend", self.backend.as_str())
            .with("oxide_backend", self.backend.oxide_backend_expr())
            .with("oxide_extra_imports", self.backend.oxide_extra_imports())
            .with("oxide_runtime_dep", &self.deps.oxide_runtime_dep)
            .with("oxide_generator_dep", &self.deps.oxide_generator_dep)
            .with("backend_deps", self.backend.pubspec_deps())
            .with("oxide_core_dep", &self.deps.oxide_core_dep)
            .with("oxide_generator_rs_dep", &self.deps.oxide_generator_rs_dep)
            .with("page_class", &self.page_class)
            .with("page_imports", self.backend.page_imports())
            .with("page_body", self.backend.page_body(&self.page_class))
    }
}

fn collect_files<'a>(
    dir: &'a include_dir::Dir<'a>,
    prefix: &str,
    out: &mut Vec<(String, &'a include_dir::File<'a>)>,
) {
    for file in dir.files() {
        let name = file
            .path()
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        out.push((format!("{prefix}{name}"), file));
    }
    for sub in dir.dirs() {
        let name = sub
            .path()
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        collect_files(sub, &format!("{prefix}{name}/"), out);
    }
}

/// Render every template file to `(destination_relpath, content)`.
///
/// `gitignore.additions` is returned with its template name; callers merge it
/// into `.gitignore` via [`merge_gitignore`].
pub fn render_all(vars: &TemplateVars) -> Vec<(String, String)> {
    let mut listed = Vec::new();
    collect_files(&TEMPLATES, "", &mut listed);
    let mut out = Vec::new();
    for (rel, file) in listed {
        let mut content = file.contents_utf8().unwrap_or_default().to_string();
        content = apply_vars(&content, vars);
        out.push((rel, content));
    }
    out.sort_by(|a, b| a.0.cmp(&b.0));
    out
}

fn apply_vars(content: &str, vars: &TemplateVars) -> String {
    vars.to_common().render(content)
}

/// Destination file list for `create --dry-run` (sorted).
///
/// `gitignore.additions` maps to `.gitignore` (merged, not overwritten).
pub fn planned_files() -> Vec<String> {
    let mut listed = Vec::new();
    collect_files(&TEMPLATES, "", &mut listed);
    let mut files: Vec<String> = listed
        .iter()
        .map(|(rel, _)| {
            if rel == "gitignore.additions" {
                ".gitignore (merge additions)".to_string()
            } else {
                rel.clone()
            }
        })
        .collect();
    files.sort();
    files
}

// ---------------------------------------------------------------------------
// FRB demo signature detection
// ---------------------------------------------------------------------------

/// True when `rust/src/api/simple.rs` still contains the unmodified FRB demo
/// (`pub fn greet`, plus FRB's own `init_app` hook, and nothing else).
/// Files with user additions are kept.
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

/// True when `lib/main.dart` still looks like the FRB demo entrypoint
/// (imports the `simple` API and calls `greet`). Approximation of the
/// byte-equal check: only delete when both markers are present.
pub fn is_frb_demo_dart_main(content: &str) -> bool {
    content.contains("api/simple") && content.contains("greet")
}

/// True when an integration test still looks like the FRB `simple_test.dart`
/// demo (references `greet`). Modified tests are kept.
pub fn is_frb_demo_integration_test(file_name: &str, content: &str) -> bool {
    // The FRB template test never mentions `greet` by name; match its stable
    // test title (and keep the old greet check for older templates).
    file_name == "simple_test.dart"
        && (content.contains("greet") || content.contains("Can call rust function"))
}

// ---------------------------------------------------------------------------
// Small fs + process helpers (duplicated in integrate.rs; both files are
// self-contained so either can be wired without the other).
// ---------------------------------------------------------------------------

/// True when external tool spawning is disabled (tests).
pub fn no_external() -> bool {
    crate::common::no_external()
}

/// Run `prog args...` in `dir`; bail on non-zero exit.
/// On Windows batch shims (`flutter.bat`, `dart.bat`) are routed via `cmd /C`.
fn spawn_cmd(prog: &str, args: &[&str], dir: &Path) -> Result<std::process::ExitStatus> {
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
    cmd.current_dir(dir)
        .status()
        .with_context(|| format!("failed to spawn `{prog}`"))
}

/// Run `prog args...` in `dir`; bail on non-zero exit.
pub fn run_cmd(prog: &str, args: &[&str], dir: &Path) -> Result<()> {
    if no_external() {
        println!(
            "(skip external: {} {} [in {}])",
            prog,
            args.join(" "),
            dir.display()
        );
        return Ok(());
    }
    let status = spawn_cmd(prog, args, dir)?;
    if !status.success() {
        bail!("`{} {}` failed with {status}", prog, args.join(" "));
    }
    Ok(())
}

/// Preflight: require flutter/dart/cargo + `flutter_rust_bridge_codegen == 2.12.0`.
/// Exits the process with [`EXIT_PREFLIGHT`] on failure.
///
/// Returns immediately when `OXIDE_CLI_NO_EXTERNAL=1`, which the module docs
/// describe as suppressing external tool use. `run_cmd` already honored it, so
/// skipping here is what makes that escape hatch usable at all; without this a
/// non-dry-run `create` on a machine without FRB installed exited 3 before
/// printing anything, including inside the test suite.
pub fn preflight() {
    if no_external() {
        return;
    }
    for tool in ["flutter", "dart", "cargo", "flutter_rust_bridge_codegen"] {
        if which::which(tool).is_err() {
            eprintln!("error: required tool `{tool}` not found on PATH");
            eprintln!("install it, then re-run `oxide create`");
            std::process::exit(EXIT_PREFLIGHT);
        }
    }
    if !frb_codegen_version_matches() {
        eprintln!("error: `flutter_rust_bridge_codegen` version {FRB_VERSION} is required");
        eprintln!(
            "install it with: cargo install flutter_rust_bridge_codegen --version {FRB_VERSION}"
        );
        std::process::exit(EXIT_PREFLIGHT);
    }
}

/// Whether the installed `flutter_rust_bridge_codegen` matches [`FRB_VERSION`].
///
/// Reads stdout **and** stderr: tools are inconsistent about which stream their
/// version banner goes to, and reading only stdout made this disagree with
/// `doctor`'s toolchain check on the same machine.
fn frb_codegen_version_matches() -> bool {
    let Some(out) = std::process::Command::new("flutter_rust_bridge_codegen")
        .arg("--version")
        .output()
        .ok()
    else {
        return false;
    };
    let banner = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    common::parse_version_token(&banner)
        .and_then(|tok| semver::Version::parse(tok.trim_matches('v')).ok())
        .is_some_and(|v| v == semver::Version::new(2, 12, 0))
}

fn valid_app_name(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        Some(c) if c.is_ascii_lowercase() => {}
        _ => return false,
    }
    name.chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
}

// ---------------------------------------------------------------------------
// run()
// ---------------------------------------------------------------------------

/// Execute `oxide create`.
pub fn run(args: CreateArgs) -> Result<()> {
    if !valid_app_name(&args.name) {
        bail!(
            "invalid app name `{}` (use lowercase letters, digits, underscores, starting with a letter)",
            args.name
        );
    }
    let backend = parse_backend(&args.backend)?;
    let cwd = std::env::current_dir().context("failed to read current dir")?;
    let base = args.base_dir.clone().unwrap_or_else(|| cwd.clone());
    let app_dir = base.join(&args.name);
    if app_dir.exists() {
        bail!(
            "refusing to create `{}`: path already exists",
            app_dir.display()
        );
    }
    let rust_crate = args
        .rust_crate_name
        .clone()
        .unwrap_or_else(|| format!("rust_lib_{}", args.name));
    let source = match &args.oxide_path {
        Some(p) => OxideSource::Path {
            root: p.canonicalize().unwrap_or_else(|_| p.clone()),
        },
        None => resolve_oxide_source(&args.oxide_source, &cwd)?,
    };
    // Canonicalize for stable relative paths (app dir does not exist yet, so
    // canonicalize the base and re-append the name).
    let base_canon = base.canonicalize().unwrap_or(base);
    let app_dir_canon = base_canon.join(&args.name);
    let vars = TemplateVars {
        app_name: args.name.clone(),
        rust_crate: rust_crate.clone(),
        backend,
        page_class: "CounterPage".to_string(),
        deps: dep_snippets(&source, &app_dir_canon),
    };

    if args.dry_run {
        println!("would create {} in {}/:", planned_files().len(), args.name);
        for f in planned_files() {
            println!("  {}/{f}", args.name);
        }
        return Ok(());
    }

    preflight();

    // 1. FRB scaffolding: `flutter create` + rust crate + cargokit wiring.
    let mut frb_args: Vec<&str> = vec!["create", &args.name];
    if let Some(org) = args.org.as_deref() {
        frb_args.push("--org");
        frb_args.push(org);
    }
    // Note: --rust-crate-name is always passed so rust_builder package name
    // matches the `{{rust_crate}}: path: rust_builder` dep in pubspec.yaml.
    // Note: --rust-crate-name is always passed so rust_builder package name
    // matches the `{{rust_crate}}: path: rust_builder` dep in pubspec.yaml.
    run_cmd("flutter_rust_bridge_codegen", &frb_args, &base_canon)?;

    // 2. Delete FRB demo files by content signature (unless kept).
    if !args.keep_frb_example {
        remove_frb_demo(&app_dir)?;
    }

    // 3. Overlay Oxide templates.
    let mut gitignore_add = String::new();
    for (rel, content) in render_all(&vars) {
        if rel == "gitignore.additions" {
            gitignore_add = content;
            continue;
        }
        write_atomic(&app_dir.join(&rel), &content)?;
    }
    let gi_path = app_dir.join(".gitignore");
    let existing = std::fs::read_to_string(&gi_path).ok();
    write_atomic(
        &gi_path,
        &merge_gitignore(existing.as_deref(), &gitignore_add),
    )?;

    // 4. Codegen.
    if !args.no_codegen {
        run_cmd("flutter", &["pub", "get"], &app_dir)?;
        run_cmd(
            "flutter_rust_bridge_codegen",
            &["generate", "--config-file", "flutter_rust_bridge.yaml"],
            &app_dir,
        )?;
        run_cmd(
            "dart",
            &[
                "run",
                "build_runner",
                "build",
                "--delete-conflicting-outputs",
            ],
            &app_dir,
        )?;
    }

    // 5. Optional verification.
    if args.verify {
        run_cmd("flutter", &["analyze"], &app_dir)?;
        run_cmd("cargo", &["check"], &app_dir.join("rust"))?;
    }

    println!("created {} (backend: {})", args.name, backend.as_str());
    println!("next steps:");
    println!("  cd {}", args.name);
    println!("  flutter run");
    println!("regenerate with: oxide generate");
    Ok(())
}

/// Remove FRB demo files under `app_dir` only when their content still matches
/// the known demo signatures.
fn remove_frb_demo(app_dir: &Path) -> Result<()> {
    // rust/src/api/simple.rs + `pub mod simple;` entry.
    let simple_rs = app_dir.join("rust/src/api/simple.rs");
    if simple_rs.is_file() {
        let content = std::fs::read_to_string(&simple_rs).unwrap_or_default();
        if is_frb_demo_rust_api("simple.rs", &content) {
            std::fs::remove_file(&simple_rs)
                .with_context(|| format!("failed to remove {}", simple_rs.display()))?;
            println!("removed FRB demo {}", simple_rs.display());
            let api_mod = app_dir.join("rust/src/api/mod.rs");
            if api_mod.is_file() {
                let orig = std::fs::read_to_string(&api_mod).unwrap_or_default();
                let filtered: Vec<&str> = orig
                    .lines()
                    .filter(|l| l.trim() != "pub mod simple;")
                    .collect();
                if filtered.len() != orig.lines().count() {
                    let mut out = filtered.join("\n");
                    if !out.is_empty() {
                        out.push('\n');
                    }
                    write_atomic(&api_mod, &out)?;
                }
            }
        } else {
            println!("keeping modified rust/src/api/simple.rs (demo signature not found)");
        }
    }
    // Demo lib/main.dart.
    let main_dart = app_dir.join("lib/main.dart");
    if main_dart.is_file() {
        let content = std::fs::read_to_string(&main_dart).unwrap_or_default();
        if is_frb_demo_dart_main(&content) {
            std::fs::remove_file(&main_dart)
                .with_context(|| format!("failed to remove {}", main_dart.display()))?;
            println!("removed FRB demo {}", main_dart.display());
        }
    }
    // integration_test/simple_test.dart.
    let simple_test = app_dir.join("integration_test/simple_test.dart");
    if simple_test.is_file() {
        let content = std::fs::read_to_string(&simple_test).unwrap_or_default();
        if is_frb_demo_integration_test("simple_test.dart", &content) {
            std::fs::remove_file(&simple_test)
                .with_context(|| format!("failed to remove {}", simple_test.display()))?;
            println!("removed FRB demo {}", simple_test.display());
        } else {
            println!("keeping modified integration_test/simple_test.dart");
        }
    }
    // Generated Dart orphans the FRB scaffold emits before we delete the demo
    // Rust sources (FRB never prunes them, and they reference deleted APIs).
    for orphan in [
        "lib/src/rust/api/simple.dart",
        "lib/src/rust/api/simple.freezed.dart",
    ] {
        let path = app_dir.join(orphan);
        if path.is_file() {
            std::fs::remove_file(&path)
                .with_context(|| format!("failed to remove {}", path.display()))?;
            println!("removed FRB demo orphan {}", path.display());
        }
    }
    Ok(())
}
