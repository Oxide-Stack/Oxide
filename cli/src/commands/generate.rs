//! `oxide generate`: regenerate FRB bindings and oxide build_runner outputs.
//!
//! Canonical order: `flutter pub get` (only when stale) ->
//! `flutter_rust_bridge_codegen generate --config-file <cfg>` (+ passthrough) ->
//! `dart run build_runner build -d`. The command is idempotent: a second run
//! is a no-op when outputs are fresh. Nothing is ever installed over the
//! network; missing tools are reported, not fetched.
//!
//! The local [`CommandRunner`] trait and [`resolve_tool`] mirror the shared
//! process/project APIs (`CommandRunner::run(cmd, args, cwd)`,
//! `resolve_tool(name)`) so this module compiles standalone.

use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

use anyhow::{Context, Result, bail};

use crate::common::glue::{GlueSnapshot, nuke as nuke_oxide_glue};

/// Exit code when `--check` detects drift in generated output.
pub const EXIT_DRIFT: i32 = 4;

const FRB_CONFIG_NAME: &str = "flutter_rust_bridge.yaml";

/// CLI args for `oxide generate`.
#[derive(Debug, Clone, clap::Args)]
pub struct GenerateArgs {
    /// Project root (Flutter app dir). Defaults to cwd.
    #[arg(long, value_name = "DIR")]
    pub root: Option<PathBuf>,

    /// Skip `flutter pub get` even when stale.
    #[arg(long)]
    pub no_pub_get: bool,

    /// Only run FRB codegen (skip build_runner).
    #[arg(long)]
    pub frb_only: bool,

    /// Only run build_runner (skip FRB codegen).
    #[arg(long)]
    pub oxide_only: bool,

    /// Print subcommands without running them.
    #[arg(long)]
    pub dry_run: bool,

    /// Watch rust/src + lib and re-run on changes (debounced 500ms).
    #[arg(long)]
    pub watch: bool,

    /// CI mode: generate, then fail with exit code 4 on drift.
    #[arg(long)]
    pub check: bool,

    /// FRB config file. Defaults to flutter_rust_bridge.yaml in root.
    #[arg(long, value_name = "FILE")]
    pub config_file: Option<PathBuf>,

    /// Extra args appended to the FRB codegen invocation (after `--`).
    #[arg(last = true, value_name = "FRB_ARGS")]
    pub frb_args: Vec<String>,
}

/// Minimal runner contract: run `cmd args...` in `cwd`, error on failure.
pub trait CommandRunner {
    fn run(&self, cmd: &str, args: &[String], cwd: &Path) -> Result<()>;
}

/// Resolve a tool via PATH. Never installs; errors with a hint instead.
pub fn resolve_tool(name: &str) -> Result<PathBuf> {
    which::which(name).with_context(|| {
        format!(
            "required tool `{name}` not found on PATH \
             (install it manually; oxide never installs tools over the network)"
        )
    })
}

/// Process-spawning runner with inherited stdio.
pub struct StdRunner;

impl CommandRunner for StdRunner {
    fn run(&self, cmd: &str, args: &[String], cwd: &Path) -> Result<()> {
        let status = std::process::Command::new(cmd)
            .args(args)
            .current_dir(cwd)
            .status()
            .with_context(|| format!("failed to spawn `{}`", format_cmd(cmd, args)))?;
        if status.success() {
            Ok(())
        } else {
            bail!("command failed ({}): {}", status, format_cmd(cmd, args))
        }
    }
}

fn quote_arg(s: &str) -> String {
    if s.chars()
        .any(|c| c.is_whitespace() || c == '"' || c == '\'')
    {
        format!("\"{}\"", s.replace('"', "\\\""))
    } else {
        s.to_string()
    }
}

/// Render a copy-pasteable command line.
pub fn format_cmd(cmd: &str, args: &[String]) -> String {
    let mut parts = vec![quote_arg(cmd)];
    parts.extend(args.iter().map(|a| quote_arg(a)));
    parts.join(" ")
}

/// Resolve project root: `--root` or cwd.
pub fn resolve_root(args: &GenerateArgs) -> Result<PathBuf> {
    match &args.root {
        Some(r) => Ok(r.clone()),
        None => std::env::current_dir().context("cannot determine current directory (pass --root)"),
    }
}

/// Passthrough already carries `--config-file` (`--config-file x` or `--config-file=x`)?
fn passthrough_has_config(extra: &[String]) -> Option<String> {
    let mut it = extra.iter();
    while let Some(a) = it.next() {
        if a == "--config-file" {
            return it.next().cloned();
        }
        if let Some(v) = a.strip_prefix("--config-file=") {
            return Some(v.to_string());
        }
    }
    None
}

fn join_root(root: &Path, raw: &str) -> PathBuf {
    let p = PathBuf::from(raw);
    if p.is_absolute() { p } else { root.join(p) }
}

/// Resolve FRB config: explicit flag > passthrough > root/flutter_rust_bridge.yaml.
/// Returns None when the FRB step is skipped (`--oxide-only`).
fn resolve_config(root: &Path, args: &GenerateArgs) -> Result<Option<PathBuf>> {
    if args.oxide_only {
        return Ok(None);
    }
    if let Some(c) = &args.config_file {
        let p = if c.is_absolute() {
            c.clone()
        } else {
            root.join(c)
        };
        if !p.is_file() {
            bail!(
                "FRB config not found: {} (pass --config-file <path>)",
                p.display()
            );
        }
        return Ok(Some(p));
    }
    if let Some(v) = passthrough_has_config(&args.frb_args) {
        let p = join_root(root, &v);
        if !p.is_file() {
            bail!(
                "FRB config not found: {} (pass --config-file <path>)",
                p.display()
            );
        }
        return Ok(Some(p));
    }
    let def = root.join(FRB_CONFIG_NAME);
    if def.is_file() {
        Ok(Some(def))
    } else {
        bail!(
            "no {FRB_CONFIG_NAME} in {} (pass --config-file <path> or copy one from an example app)",
            root.display()
        )
    }
}

/// Config path as invoked from root (relative when under root, for pasteability).
fn display_config(root: &Path, config: &Path) -> String {
    config
        .strip_prefix(root)
        .unwrap_or(config)
        .to_string_lossy()
        .into_owned()
}

fn mtime(p: &Path) -> Option<SystemTime> {
    std::fs::metadata(p).and_then(|m| m.modified()).ok()
}

fn is_generated_name(name: &str) -> bool {
    name.contains("frb_generated") || name.ends_with(".g.dart")
}

fn all_files(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        let Ok(rd) = std::fs::read_dir(&d) else {
            continue;
        };
        for e in rd.flatten() {
            let p = e.path();
            if p.is_dir() {
                let n = e.file_name().to_string_lossy().into_owned();
                if n == ".dart_tool" || n == "build" || n == ".git" {
                    continue;
                }
                stack.push(p);
            } else {
                files.push(p);
            }
        }
    }
    files
}

fn file_name_of(p: &Path) -> String {
    p.file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default()
}

/// True when generated outputs exist and are newer than all inputs.
pub fn generated_outputs_fresh(root: &Path, config: Option<&Path>) -> bool {
    let mut inputs: Vec<PathBuf> = Vec::new();
    if let Some(c) = config {
        inputs.push(c.to_path_buf());
    }
    for f in ["pubspec.yaml", "build.yaml"] {
        let p = root.join(f);
        if p.is_file() {
            inputs.push(p);
        }
    }
    for d in ["rust", "lib"] {
        let d = root.join(d);
        if d.is_dir() {
            inputs.extend(
                all_files(&d)
                    .into_iter()
                    .filter(|p| !is_generated_name(&file_name_of(p))),
            );
        }
    }
    let mut outputs = Vec::new();
    for d in ["lib", "rust"] {
        let d = root.join(d);
        if d.is_dir() {
            outputs.extend(
                all_files(&d)
                    .into_iter()
                    .filter(|p| is_generated_name(&file_name_of(p))),
            );
        }
    }
    if outputs.is_empty() {
        return false;
    }
    let newest_input = inputs.iter().filter_map(|p| mtime(p)).max();
    let newest_output = outputs.iter().filter_map(|p| mtime(p)).max();
    match (newest_input, newest_output) {
        (_, None) => false,
        (None, Some(_)) => true,
        (Some(i), Some(o)) => o >= i,
    }
}

/// True when `flutter pub get` should run: package config missing or older than pubspec.
pub fn pub_get_needed(root: &Path) -> bool {
    if !root.join("pubspec.yaml").is_file() {
        return false;
    }
    match (
        mtime(&root.join("pubspec.yaml")),
        mtime(&root.join(".dart_tool").join("package_config.json")),
    ) {
        (Some(p), Some(c)) => c < p,
        _ => true,
    }
}

/// Entry point: returns the process exit code (0 ok, 4 drift under `--check`).
/// Tool failures are returned as errors (caller maps to exit 1).
pub fn run(args: &GenerateArgs, runner: &dyn CommandRunner) -> Result<i32> {
    run_with_freshness(args, runner, None)
}

fn run_with_freshness(
    args: &GenerateArgs,
    runner: &dyn CommandRunner,
    fresh_override: Option<bool>,
) -> Result<i32> {
    if args.frb_only && args.oxide_only {
        bail!("--frb-only and --oxide-only are mutually exclusive");
    }
    if args.watch && args.check {
        bail!("--watch and --check are mutually exclusive");
    }
    let root = resolve_root(args)?;
    if args.watch {
        return run_watch(args, runner, &root);
    }
    let config = resolve_config(&root, args)?;
    // Test-only override mocks full freshness (inputs and pub cache).
    let need_pub = fresh_override != Some(true) && !args.no_pub_get && pub_get_needed(&root);
    let fresh = fresh_override.unwrap_or_else(|| generated_outputs_fresh(&root, config.as_deref()));
    run_steps(args, runner, &root, &config, fresh, need_pub)?;
    if args.check {
        return run_check(&root, config.as_deref());
    }
    Ok(0)
}

fn run_steps(
    args: &GenerateArgs,
    runner: &dyn CommandRunner,
    root: &Path,
    config: &Option<PathBuf>,
    fresh: bool,
    need_pub: bool,
) -> Result<bool> {
    if !root.join("pubspec.yaml").is_file() {
        eprintln!(
            "warning: no pubspec.yaml in {}; skipping `flutter pub get`",
            root.display()
        );
    }
    if fresh && !need_pub {
        println!("oxide generate: outputs up to date (nothing to do)");
        return Ok(false);
    }
    let mut steps: Vec<(&str, Vec<String>)> = Vec::new();
    if need_pub {
        steps.push(("flutter", vec!["pub".into(), "get".into()]));
    }
    if !args.oxide_only {
        let cfg = config
            .as_ref()
            .expect("config resolved unless --oxide-only");
        let mut frb: Vec<String> = vec!["generate".into()];
        if passthrough_has_config(&args.frb_args).is_none() {
            frb.push("--config-file".into());
            frb.push(display_config(root, cfg));
        }
        frb.extend(args.frb_args.clone());
        steps.push(("flutter_rust_bridge_codegen", frb));
    }
    if !args.frb_only {
        steps.push((
            "dart",
            vec![
                "run".into(),
                "build_runner".into(),
                "build".into(),
                "-d".into(),
            ],
        ));
    }
    for (cmd, step_args) in &steps {
        println!("{}", format_cmd(cmd, step_args));
        if args.dry_run {
            continue;
        }
        if resolve_tool(cmd).is_err() {
            eprintln!("warning: `{cmd}` not found on PATH; attempting anyway");
        }
        // The oxide builders read `rust/target/oxide_routes/*.json` via
        // `dart:io`, invisible to build_runner's asset graph: drop oxide-owned
        // glue first so the builders rerun instead of serving stale cache.
        //
        // Those outputs are gitignored, so snapshot them first and put them back
        // if this step fails. Otherwise a build error leaves the project with no
        // oxide glue at all and no way to recover it.
        let snapshot = if *cmd == "dart" && step_args.get(1).is_some_and(|s| s == "build_runner") {
            let snap = GlueSnapshot::capture(root);
            nuke_oxide_glue(root);
            Some(snap)
        } else {
            None
        };
        if let Err(e) = runner.run(cmd, step_args, root) {
            if let Some(s) = snapshot.filter(|s| !s.is_empty()) {
                s.restore();
                eprintln!(
                    "warning: codegen failed; restored {} previously generated path(s)",
                    s.len()
                );
            }
            return Err(e);
        }
    }
    Ok(true)
}

fn git_dir_walk(root: &Path) -> bool {
    let mut cur = root.to_path_buf();
    loop {
        if cur.join(".git").exists() {
            return true;
        }
        if !cur.pop() {
            return false;
        }
    }
}

fn is_git_repo(root: &Path) -> bool {
    match std::process::Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .current_dir(root)
        .output()
    {
        Ok(o) => {
            if o.status.success() {
                return true;
            }
            // Git works but this is not a work tree; a stale `.git` walk
            // agrees, so skip it.
            false
        }
        Err(_) => git_dir_walk(root),
    }
}

fn check_freshness_fallback(root: &Path, config: Option<&Path>) -> i32 {
    if generated_outputs_fresh(root, config) {
        0
    } else {
        eprintln!("generated outputs are stale (not a git repo; freshness check failed)");
        EXIT_DRIFT
    }
}

/// CI drift check: `git diff --exit-code -- lib rust`, or freshness outside git.
fn run_check(root: &Path, config: Option<&Path>) -> Result<i32> {
    if !is_git_repo(root) {
        return Ok(check_freshness_fallback(root, config));
    }
    match std::process::Command::new("git")
        .args(["diff", "--exit-code", "--name-only", "--", "lib", "rust"])
        .current_dir(root)
        .output()
    {
        Ok(o) if o.status.success() => Ok(0),
        Ok(o) => {
            let files = String::from_utf8_lossy(&o.stdout);
            eprintln!("generated outputs drift detected:");
            let mut any = false;
            for f in files.lines().map(str::trim).filter(|l| !l.is_empty()) {
                eprintln!("  {f}");
                any = true;
            }
            if !any {
                eprintln!(
                    "  (run `git diff --name-only -- lib rust` in {} for details)",
                    root.display()
                );
            }
            Ok(EXIT_DRIFT)
        }
        Err(e) => {
            eprintln!("warning: git unavailable ({e}); falling back to freshness check");
            Ok(check_freshness_fallback(root, config))
        }
    }
}

/// Watch `rust/src` + `lib` and re-run the sequence debounced 500ms.
/// Blocks until the watcher channel disconnects; Ctrl-C stops the process.
fn run_watch(args: &GenerateArgs, runner: &dyn CommandRunner, root: &Path) -> Result<i32> {
    use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};

    let config = resolve_config(root, args)?;
    if args.dry_run {
        let need_pub = !args.no_pub_get && pub_get_needed(root);
        let fresh = generated_outputs_fresh(root, config.as_deref());
        run_steps(args, runner, root, &config, fresh, need_pub)?;
        println!("oxide generate: would watch rust/src + lib (debounced 500ms)");
        return Ok(0);
    }
    println!("oxide generate --watch: initial run");
    let need_pub = !args.no_pub_get && pub_get_needed(root);
    let fresh = generated_outputs_fresh(root, config.as_deref());
    run_steps(args, runner, root, &config, fresh, need_pub)?;

    let (tx, rx) = std::sync::mpsc::channel::<Result<Event, notify::Error>>();
    let mut watcher =
        RecommendedWatcher::new(tx, Config::default()).context("failed to start file watcher")?;
    let mut watched = Vec::new();
    if !args.oxide_only {
        let rust_src = root.join("rust").join("src");
        let rust_dir = if rust_src.is_dir() {
            rust_src
        } else {
            root.join("rust")
        };
        if rust_dir.is_dir() {
            watcher
                .watch(&rust_dir, RecursiveMode::Recursive)
                .with_context(|| format!("failed to watch {}", rust_dir.display()))?;
            watched.push(rust_dir.to_string_lossy().into_owned());
        }
    }
    if !args.frb_only && root.join("lib").is_dir() {
        watcher
            .watch(&root.join("lib"), RecursiveMode::Recursive)
            .context("failed to watch lib")?;
        watched.push(root.join("lib").to_string_lossy().into_owned());
    }
    if let Some(c) = &config {
        let under_watch = watched.iter().any(|w| c.starts_with(Path::new(w.as_str())));
        if c.is_file() && !under_watch {
            watcher
                .watch(c, RecursiveMode::NonRecursive)
                .with_context(|| format!("failed to watch {}", c.display()))?;
            watched.push(c.to_string_lossy().into_owned());
        }
    }
    if watched.is_empty() {
        bail!(
            "nothing to watch: no rust/src or lib dir under {}",
            root.display()
        );
    }
    println!(
        "oxide generate: watching {} (debounced 500ms; Ctrl-C to stop)",
        watched.join(", ")
    );
    let relevant = |ev: &Event| {
        matches!(
            ev.kind,
            EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_)
        )
    };
    loop {
        match rx.recv() {
            Ok(Ok(ev)) if relevant(&ev) => {
                // Debounce: wait for 500ms of quiet before re-running.
                loop {
                    match rx.recv_timeout(Duration::from_millis(500)) {
                        Ok(Ok(ev)) if relevant(&ev) => {}
                        Ok(_) => {}
                        Err(std::sync::mpsc::RecvTimeoutError::Timeout) => break,
                        Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                            println!("oxide generate: watcher stopped");
                            return Ok(0);
                        }
                    }
                }
                println!("change detected; regenerating...");
                let need_pub = !args.no_pub_get && pub_get_needed(root);
                let fresh = generated_outputs_fresh(root, config.as_deref());
                if let Err(e) = run_steps(args, runner, root, &config, fresh, need_pub) {
                    eprintln!("regeneration failed: {e:#}");
                }
            }
            Ok(_) => {}
            Err(_) => {
                println!("oxide generate: watcher stopped");
                return Ok(0);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    struct FakeRunner {
        calls: Mutex<Vec<(String, Vec<String>, PathBuf)>>,
    }

    impl FakeRunner {
        fn new() -> Self {
            Self {
                calls: Mutex::new(Vec::new()),
            }
        }

        fn calls(&self) -> Vec<(String, Vec<String>, PathBuf)> {
            self.calls.lock().unwrap().clone()
        }

        fn cmd_names(&self) -> Vec<String> {
            self.calls().iter().map(|(c, _, _)| c.clone()).collect()
        }
    }

    impl CommandRunner for FakeRunner {
        fn run(&self, cmd: &str, args: &[String], cwd: &Path) -> Result<()> {
            self.calls
                .lock()
                .unwrap()
                .push((cmd.to_string(), args.to_vec(), cwd.to_path_buf()));
            Ok(())
        }
    }

    fn write(path: &Path, content: &str) {
        if let Some(p) = path.parent() {
            std::fs::create_dir_all(p).unwrap();
        }
        std::fs::write(path, content).unwrap();
    }

    fn set_mtime(path: &Path, t: SystemTime) {
        std::fs::File::options()
            .write(true)
            .open(path)
            .unwrap()
            .set_modified(t)
            .unwrap();
    }

    fn base_args(root: &Path) -> GenerateArgs {
        GenerateArgs {
            root: Some(root.to_path_buf()),
            no_pub_get: false,
            frb_only: false,
            oxide_only: false,
            dry_run: false,
            watch: false,
            check: false,
            config_file: None,
            frb_args: Vec::new(),
        }
    }

    /// Root with stale pub cache, config present, and no generated outputs.
    fn stale_root() -> tempfile::TempDir {
        let tmp = tempfile::tempdir().unwrap();
        let r = tmp.path();
        write(&r.join("pubspec.yaml"), "name: demo\n");
        write(&r.join(".dart_tool/package_config.json"), "{}");
        write(
            &r.join("flutter_rust_bridge.yaml"),
            "rust_input: crate::api\n",
        );
        write(&r.join("rust/src/api.rs"), "// api\n");
        write(&r.join("lib/main.dart"), "// app\n");
        let now = SystemTime::now();
        set_mtime(&r.join("pubspec.yaml"), now);
        set_mtime(
            &r.join(".dart_tool/package_config.json"),
            now - Duration::from_secs(60),
        );
        tmp
    }

    /// Root where outputs are newer than all inputs and pub cache is fresh.
    fn fresh_root() -> tempfile::TempDir {
        let tmp = tempfile::tempdir().unwrap();
        let r = tmp.path();
        write(&r.join("pubspec.yaml"), "name: demo\n");
        write(&r.join(".dart_tool/package_config.json"), "{}");
        write(
            &r.join("flutter_rust_bridge.yaml"),
            "rust_input: crate::api\n",
        );
        write(&r.join("rust/src/api.rs"), "// api\n");
        write(&r.join("lib/main.dart"), "// app\n");
        write(&r.join("lib/frb_generated.dart"), "// generated\n");
        write(&r.join("rust/src/frb_generated.rs"), "// generated\n");
        let t0 = SystemTime::now() - Duration::from_secs(120);
        for f in [
            "pubspec.yaml",
            "flutter_rust_bridge.yaml",
            "rust/src/api.rs",
            "lib/main.dart",
        ] {
            set_mtime(&r.join(f), t0);
        }
        set_mtime(
            &r.join(".dart_tool/package_config.json"),
            t0 + Duration::from_secs(30),
        );
        set_mtime(
            &r.join("lib/frb_generated.dart"),
            t0 + Duration::from_secs(60),
        );
        set_mtime(
            &r.join("rust/src/frb_generated.rs"),
            t0 + Duration::from_secs(60),
        );
        tmp
    }

    #[test]
    fn runs_pub_get_then_frb_then_build_runner() {
        let tmp = stale_root();
        let args = base_args(tmp.path());
        let runner = FakeRunner::new();
        let code = run(&args, &runner).unwrap();
        assert_eq!(code, 0);
        let calls = runner.calls();
        assert_eq!(calls.len(), 3, "expected 3 steps, got {calls:?}");
        assert_eq!(calls[0].0, "flutter");
        assert_eq!(calls[0].1, vec!["pub".to_string(), "get".to_string()]);
        assert_eq!(calls[1].0, "flutter_rust_bridge_codegen");
        assert_eq!(
            calls[1].1,
            vec![
                "generate".to_string(),
                "--config-file".to_string(),
                "flutter_rust_bridge.yaml".to_string(),
            ]
        );
        assert_eq!(calls[2].0, "dart");
        assert_eq!(
            calls[2].1,
            vec![
                "run".to_string(),
                "build_runner".to_string(),
                "build".to_string(),
                "-d".to_string(),
            ]
        );
        for (_, _, cwd) in &calls {
            assert_eq!(cwd, tmp.path());
        }
    }

    #[test]
    fn frb_only_skips_build_runner() {
        let tmp = stale_root();
        let mut args = base_args(tmp.path());
        args.frb_only = true;
        let runner = FakeRunner::new();
        assert_eq!(run(&args, &runner).unwrap(), 0);
        assert_eq!(
            runner.cmd_names(),
            vec!["flutter", "flutter_rust_bridge_codegen"]
        );
    }

    #[test]
    fn oxide_only_skips_frb_and_needs_no_config() {
        let tmp = stale_root();
        std::fs::remove_file(tmp.path().join("flutter_rust_bridge.yaml")).unwrap();
        let mut args = base_args(tmp.path());
        args.oxide_only = true;
        let runner = FakeRunner::new();
        assert_eq!(run(&args, &runner).unwrap(), 0);
        assert_eq!(runner.cmd_names(), vec!["flutter", "dart"]);
    }

    #[test]
    fn second_run_noop_when_fresh_mocked() {
        let tmp = stale_root();
        let args = base_args(tmp.path());
        let runner = FakeRunner::new();
        let code = run_with_freshness(&args, &runner, Some(true)).unwrap();
        assert_eq!(code, 0);
        assert!(
            runner.calls().is_empty(),
            "expected no-op, got {:?}",
            runner.calls()
        );
    }

    #[test]
    fn second_run_noop_when_fresh_on_disk() {
        let tmp = fresh_root();
        let args = base_args(tmp.path());
        let runner = FakeRunner::new();
        assert_eq!(run(&args, &runner).unwrap(), 0);
        assert!(
            runner.calls().is_empty(),
            "expected no-op, got {:?}",
            runner.calls()
        );
    }

    #[test]
    fn dry_run_prints_only() {
        let tmp = stale_root();
        let mut args = base_args(tmp.path());
        args.dry_run = true;
        let runner = FakeRunner::new();
        assert_eq!(run(&args, &runner).unwrap(), 0);
        assert!(runner.calls().is_empty());
    }

    #[test]
    fn conflicting_modes_fail() {
        let tmp = stale_root();
        let runner = FakeRunner::new();
        let mut args = base_args(tmp.path());
        args.frb_only = true;
        args.oxide_only = true;
        assert!(run(&args, &runner).is_err());
        let mut args = base_args(tmp.path());
        args.watch = true;
        args.check = true;
        assert!(run(&args, &runner).is_err());
    }

    #[test]
    fn missing_config_fails_with_hint() {
        let tmp = tempfile::tempdir().unwrap();
        write(&tmp.path().join("pubspec.yaml"), "name: demo\n");
        let args = base_args(tmp.path());
        let runner = FakeRunner::new();
        let err = run(&args, &runner).unwrap_err();
        assert!(
            err.to_string().contains("flutter_rust_bridge.yaml"),
            "{err:#}"
        );
    }

    #[test]
    fn watch_dry_run_prints_only() {
        let tmp = stale_root();
        let mut args = base_args(tmp.path());
        args.watch = true;
        args.dry_run = true;
        let runner = FakeRunner::new();
        assert_eq!(run(&args, &runner).unwrap(), 0);
        assert!(runner.calls().is_empty());
    }

    fn git_available() -> bool {
        std::process::Command::new("git")
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    fn git(root: &Path, args: &[&str]) {
        let st = std::process::Command::new("git")
            .args(args)
            .current_dir(root)
            .status()
            .unwrap();
        assert!(st.success(), "git {args:?} failed");
    }

    #[test]
    fn check_returns_4_on_drift() {
        if !git_available() {
            eprintln!("skipping: git not installed");
            return;
        }
        let tmp = tempfile::tempdir().unwrap();
        let r = tmp.path();
        write(
            &r.join("flutter_rust_bridge.yaml"),
            "rust_input: crate::api\n",
        );
        write(&r.join("rust/src/api.rs"), "// api\n");
        write(&r.join("lib/frb_generated.dart"), "// generated v1\n");
        git(r, &["init"]);
        git(r, &["add", "."]);
        git(
            r,
            &[
                "-c",
                "user.name=t",
                "-c",
                "user.email=t@t",
                "commit",
                "-m",
                "init",
            ],
        );
        // Drift the generated file and mark inputs newer so generate runs first.
        write(&r.join("lib/frb_generated.dart"), "// generated v2 drift\n");
        let now = SystemTime::now();
        set_mtime(
            &r.join("lib/frb_generated.dart"),
            now - Duration::from_secs(120),
        );
        set_mtime(&r.join("rust/src/api.rs"), now);
        let mut args = base_args(r);
        args.check = true;
        let runner = FakeRunner::new();
        assert_eq!(run(&args, &runner).unwrap(), EXIT_DRIFT);
        let names = runner.cmd_names();
        assert!(
            names.contains(&"flutter_rust_bridge_codegen".to_string()),
            "{names:?}"
        );
        assert!(names.contains(&"dart".to_string()), "{names:?}");
    }

    #[test]
    fn check_clean_repo_returns_0() {
        if !git_available() {
            eprintln!("skipping: git not installed");
            return;
        }
        let tmp = tempfile::tempdir().unwrap();
        let r = tmp.path();
        write(
            &r.join("flutter_rust_bridge.yaml"),
            "rust_input: crate::api\n",
        );
        write(&r.join("lib/frb_generated.dart"), "// generated\n");
        git(r, &["init"]);
        git(r, &["add", "."]);
        git(
            r,
            &[
                "-c",
                "user.name=t",
                "-c",
                "user.email=t@t",
                "commit",
                "-m",
                "init",
            ],
        );
        // Mark output newer than inputs so generate is a no-op, then diff is clean.
        let now = SystemTime::now();
        set_mtime(
            &r.join("flutter_rust_bridge.yaml"),
            now - Duration::from_secs(60),
        );
        set_mtime(&r.join("lib/frb_generated.dart"), now);
        let mut args = base_args(r);
        args.check = true;
        let runner = FakeRunner::new();
        assert_eq!(run(&args, &runner).unwrap(), 0);
    }

    #[test]
    fn check_fallback_fails_4_when_stale_outside_git() {
        let tmp = stale_root();
        assert!(!is_git_repo(tmp.path()));
        let mut args = base_args(tmp.path());
        args.check = true;
        let runner = FakeRunner::new();
        assert_eq!(run(&args, &runner).unwrap(), EXIT_DRIFT);
    }

    #[test]
    fn check_fallback_returns_0_when_fresh_outside_git() {
        let tmp = fresh_root();
        assert!(!is_git_repo(tmp.path()));
        let mut args = base_args(tmp.path());
        args.check = true;
        let runner = FakeRunner::new();
        assert_eq!(run(&args, &runner).unwrap(), 0);
    }
}
