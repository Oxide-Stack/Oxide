//! End-to-end tests for all three CLI flows:
//! 1. `oxide create` a new app.
//! 2. `oxide integrate` into a plain Flutter app.
//! 3. `oxide integrate` into an existing FRB app (with user code).
//!
//! Each test drives the real `oxide` binary plus the real toolchains
//! (flutter/dart/cargo/FRB codegen) and finishes with a platform-native
//! `flutter build`, so a green run means the produced app actually builds.
//!
//! These tests are OPT-IN: they need network access and take tens of minutes.
//! Enable with:
//!
//! ```powershell
//! $env:OXIDE_CLI_E2E = "1"
//! cargo test --manifest-path cli/Cargo.toml --test e2e
//! ```
//!
//! Without the env var every test passes trivially (skip).
//! Native build selection: `$env:OXIDE_CLI_E2E_BUILD` (`windows`/`apk`/name of
//! a `flutter build` target), defaulting to `windows` on Windows and skipped
//! elsewhere. A single global lock serializes the tests: parallel `pub get`
//! runs contend on the pub cache lock.

use assert_cmd::Command as AssertCommand;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command as StdCommand, Stdio};
use std::sync::Mutex;
use std::time::{Duration, Instant};

static E2E_LOCK: Mutex<()> = Mutex::new(());
/// Generous per-command timeout: FRB scaffolding + pub resolution + native
/// builds are slow on cold caches.
const STEP_TIMEOUT: Duration = Duration::from_secs(30 * 60);

fn e2e_enabled() -> bool {
    std::env::var("OXIDE_CLI_E2E").as_deref() == Ok("1")
}

/// Build a child command for `prog`. Mirrors `src/process/runner.rs`:
/// Windows batch shims (`flutter.bat`, `dart.bat`) cannot be spawned
/// directly and are routed through `cmd /C`.
fn spawn_for(prog: &str, args: &[&str]) -> StdCommand {
    let resolved = which::which(prog).unwrap_or_else(|_| PathBuf::from(prog));
    let is_script = resolved
        .extension()
        .is_some_and(|e| e.eq_ignore_ascii_case("bat") || e.eq_ignore_ascii_case("cmd"));
    if cfg!(windows) && is_script {
        let mut c = StdCommand::new("cmd");
        c.arg("/C").arg(resolved);
        c.args(args);
        c
    } else {
        let mut c = StdCommand::new(resolved);
        c.args(args);
        c
    }
}

fn have_tool(name: &str) -> bool {
    if which::which(name).is_err() {
        return false;
    }
    spawn_for(name, &["--version"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Fail fast when a required tool is missing. Runs BEFORE taking the serial
/// lock so a missing toolchain cannot poison it for the other tests.
fn require_tools(cmds: &[&str]) {
    for tool in cmds {
        assert!(have_tool(tool), "e2e requires `{tool}` on PATH");
    }
}

fn serial() -> std::sync::MutexGuard<'static, ()> {
    E2E_LOCK.lock().unwrap_or_else(|e| e.into_inner())
}

fn oxide_bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_oxide"))
}

fn oxide(log: &Log, root: &Path, args: &[String]) -> assert_cmd::assert::Assert {
    log.step(&format!("RUN  oxide {}", args.join(" ")));
    let started = Instant::now();
    let mut cmd = AssertCommand::new(oxide_bin());
    cmd.current_dir(root).args(args).timeout(STEP_TIMEOUT);
    let out = cmd
        .output()
        .unwrap_or_else(|e| panic!("oxide {args:?}: {e}"));
    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    log.phase(
        &format!("oxide {}", args.join(" ")),
        &combined,
        started.elapsed(),
    );
    assert_cmd::assert::Assert::new(out)
}

fn s(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

// ---------------------------------------------------------------------------
// Per-test progress log
// ---------------------------------------------------------------------------

/// Appends timestamped progress lines to `<workdir>/e2e.log` *and* stderr, so
/// a long run is never silent and leaves evidence behind after it finishes.
struct Log {
    path: PathBuf,
    start: Instant,
}

impl Log {
    fn new(dir: &Path, test: &str) -> Self {
        let path = dir.join("e2e.log");
        let start = Instant::now();
        let log = Log { path, start };
        log.step(&format!("=== {test} START (pid {})", std::process::id()));
        log
    }

    fn step(&self, msg: &str) {
        let elapsed = self.start.elapsed().as_secs();
        let line = format!("[{elapsed:6}s] {msg}");
        eprintln!("{line}");
        if let Ok(mut f) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
        {
            let _ = writeln!(f, "{line}");
        }
    }

    /// Log a long-running step: announces the start, then prints the tail of
    /// the command output and the duration on completion.
    fn phase(&self, what: &str, output: &str, dur: Duration) {
        let tail: String = output
            .lines()
            .rev()
            .take(5)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect::<Vec<_>>()
            .join("\n      | ");
        self.step(&format!(
            "{what} done in {}s\n      | {tail}",
            dur.as_secs()
        ));
    }
}

/// Run `prog args...` in `dir`, streaming its output live, logging progress to
/// `log`, and asserting success.
fn run_logged(
    log: &Log,
    dir: &Path,
    prog: &str,
    args: &[&str],
    what: &str,
) -> std::process::Output {
    log.step(&format!(
        "RUN  {what}: `{prog} {}` (in {})",
        args.join(" "),
        dir.display()
    ));
    let started = Instant::now();
    let out = spawn_for(prog, args)
        .current_dir(dir)
        .stdin(Stdio::null())
        .output()
        .unwrap_or_else(|e| panic!("{what}: failed to spawn `{prog}`: {e}"));
    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    log.phase(what, &combined, started.elapsed());
    assert!(
        out.status.success(),
        "{what}: `{prog} {}` exited {}\n{combined}",
        args.join(" "),
        out.status
    );
    out
}

fn shell_success(log: &Log, dir: &Path, prog: &str, args: &[&str], what: &str) {
    run_logged(log, dir, prog, args, what);
}

/// `flutter analyze` must report zero errors. Infos are tolerated (generated
/// glue carries lint infos that repo examples carry too).
fn assert_analyze_no_errors(log: &Log, app: &Path, what: &str) {
    let out = run_logged(log, app, "flutter", &["analyze", "--no-pub"], what);
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        !text.contains(" error - "),
        "{what}: `flutter analyze` reported errors:\n{text}"
    );
}

/// Platform-native build of the finished app, or `None` when skipped.
fn native_build_target() -> Option<&'static str> {
    if let Ok(t) = std::env::var("OXIDE_CLI_E2E_BUILD") {
        if t == "none" || t.is_empty() {
            return None;
        }
        return match t.as_str() {
            "windows" => Some("windows"),
            "apk" => Some("apk"),
            _ => panic!("unsupported OXIDE_CLI_E2E_BUILD={t} (expected windows|apk|none)"),
        };
    }
    if cfg!(windows) { Some("windows") } else { None }
}

fn native_build(log: &Log, app: &Path, what: &str) {
    let Some(target) = native_build_target() else {
        log.step(&format!(
            "{what}: skipping native build (no target for this platform)"
        ));
        return;
    };
    let mut args = vec!["build", target];
    let extra;
    if target == "windows" {
        extra = vec!["--debug"];
        args.extend(extra.iter());
    }
    shell_success(
        log,
        app,
        "flutter",
        &args,
        &format!("{what}: native build ({target})"),
    );
}

fn write(path: &Path, content: &str) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }
    std::fs::write(path, content).unwrap();
}

// ---------------------------------------------------------------------------
// 1. create
// ---------------------------------------------------------------------------

#[test]
fn e2e_create_builds() {
    if !e2e_enabled() {
        eprintln!("skipping e2e_create_builds (set OXIDE_CLI_E2E=1 to run)");
        return;
    }
    let _guard = serial();
    require_tools(&["flutter", "dart", "cargo", "flutter_rust_bridge_codegen"]);
    let dir = tempfile::tempdir().unwrap();
    let log = Log::new(dir.path(), "e2e_create_builds");

    oxide(
        &log,
        dir.path(),
        &[
            "create".to_string(),
            "e2e_app".to_string(),
            "--yes".to_string(),
        ],
    )
    .success()
    .stdout(predicates::str::contains("created e2e_app"));

    let app = dir.path().join("e2e_app");
    for rel in [
        "lib/src/oxide.dart",
        "lib/src/oxide.oxide.g.dart",
        "lib/oxide_generated/oxide_stack.g.dart",
        "rust/src/routes/home_route.rs",
        "rust/src/api/bridge.rs",
        "lib/src/rust/api/bridge.dart",
        "flutter_rust_bridge.yaml",
        "pubspec.yaml",
        "rust/Cargo.toml",
    ] {
        assert!(app.join(rel).is_file(), "create: missing {rel}");
    }
    // Regenerating is a no-op (idempotent codegen).
    oxide(
        &log,
        dir.path(),
        &[
            "generate".to_string(),
            "--check".to_string(),
            "--root".to_string(),
            s(&app),
        ],
    )
    .success();
    // Doctor is clean (warnings tolerated, failures are not).
    oxide(
        &log,
        dir.path(),
        &["doctor".to_string(), "--root".to_string(), s(&app)],
    )
    .success();
    assert_analyze_no_errors(&log, &app, "create: analyze");
    shell_success(
        &log,
        &app.join("rust"),
        "cargo",
        &["test"],
        "create: cargo test",
    );
    shell_success(&log, &app, "flutter", &["test"], "create: flutter test");
    native_build(&log, &app, "create");
    log.step("=== e2e_create_builds DONE");
}

// ---------------------------------------------------------------------------
// 2. integrate into a plain Flutter app
// ---------------------------------------------------------------------------

#[test]
fn e2e_integrate_flutter_app_builds() {
    if !e2e_enabled() {
        eprintln!("skipping e2e_integrate_flutter_app_builds (set OXIDE_CLI_E2E=1 to run)");
        return;
    }
    let _guard = serial();
    require_tools(&["flutter", "dart", "cargo", "flutter_rust_bridge_codegen"]);
    let dir = tempfile::tempdir().unwrap();
    let log = Log::new(dir.path(), "e2e_integrate_flutter_app_builds");

    shell_success(
        &log,
        dir.path(),
        "flutter",
        &["create", "--no-pub", "plain"],
        "setup: flutter create",
    );
    let app = dir.path().join("plain");
    let main_before = std::fs::read_to_string(app.join("lib/main.dart")).unwrap();
    assert!(
        main_before.contains("MyApp"),
        "fixture main.dart unexpected"
    );

    oxide(
        &log,
        dir.path(),
        &[
            "integrate".to_string(),
            "--yes".to_string(),
            "--root".to_string(),
            s(&app),
            "--oxide-source".to_string(),
            "pub".to_string(),
        ],
    )
    .success();

    // Original app untouched apart from init wiring.
    let main_after = std::fs::read_to_string(app.join("lib/main.dart")).unwrap();
    assert!(
        main_after.contains("class MyApp"),
        "integrate broke the original app"
    );
    assert!(
        main_after.contains("runOxideApp") || main_after.contains("OxideStack.init"),
        "integrate did not wire Oxide init into main.dart"
    );
    // Oxide layer present.
    for rel in [
        "lib/src/oxide.dart",
        "lib/src/oxide.oxide.g.dart",
        "rust/src/api/oxide_bridge.rs",
    ] {
        assert!(app.join(rel).is_file(), "integrate: missing {rel}");
    }
    // Idempotent: second run is a no-op.
    oxide(
        &log,
        dir.path(),
        &[
            "integrate".to_string(),
            "--yes".to_string(),
            "--root".to_string(),
            s(&app),
            "--oxide-source".to_string(),
            "pub".to_string(),
        ],
    )
    .success()
    .stdout(predicates::str::contains("no changes"));
    // Original widget test still passes: the app is not broken.
    shell_success(
        &log,
        &app,
        "flutter",
        &["test"],
        "integrate: original flutter test",
    );
    assert_analyze_no_errors(&log, &app, "integrate: analyze");
    native_build(&log, &app, "integrate");
    log.step("=== e2e_integrate_flutter_app_builds DONE");
}

// ---------------------------------------------------------------------------
// 3. integrate into an existing FRB app carrying user code
// ---------------------------------------------------------------------------

#[test]
fn e2e_integrate_frb_app_preserves_user_code_and_builds() {
    if !e2e_enabled() {
        eprintln!(
            "skipping e2e_integrate_frb_app_preserves_user_code_and_builds (set OXIDE_CLI_E2E=1 to run)"
        );
        return;
    }
    let _guard = serial();
    require_tools(&["flutter", "dart", "cargo", "flutter_rust_bridge_codegen"]);
    let dir = tempfile::tempdir().unwrap();
    let log = Log::new(
        dir.path(),
        "e2e_integrate_frb_app_preserves_user_code_and_builds",
    );

    shell_success(
        &log,
        dir.path(),
        "flutter_rust_bridge_codegen",
        &["create", "frb_e2e", "--rust-crate-name", "rust_lib_frb_e2e"],
        "setup: frb create",
    );
    let app = dir.path().join("frb_e2e");
    // Simulate user code living next to the FRB demo.
    let simple_rs = app.join("rust/src/api/simple.rs");
    let mut content = std::fs::read_to_string(&simple_rs).unwrap();
    content.push_str(
        "\n#[flutter_rust_bridge::frb(sync)]\n\
         pub fn triple(value: i32) -> i32 {\n    value * 3\n}\n",
    );
    write(&simple_rs, &content);

    oxide(
        &log,
        dir.path(),
        &[
            "integrate".to_string(),
            "--yes".to_string(),
            "--root".to_string(),
            s(&app),
            "--oxide-source".to_string(),
            "pub".to_string(),
        ],
    )
    .success();

    // User code survives in Rust and in regenerated bindings.
    let simple_rs = std::fs::read_to_string(app.join("rust/src/api/simple.rs")).unwrap();
    assert!(
        simple_rs.contains("pub fn triple"),
        "integrate dropped user code"
    );
    let simple_dart = std::fs::read_to_string(app.join("lib/src/rust/api/simple.dart")).unwrap();
    assert!(
        simple_dart.contains("triple"),
        "integrate did not regenerate bindings for user code"
    );
    // Oxide layer present and the app has an entrypoint.
    assert!(app.join("lib/src/oxide.dart").is_file());
    assert!(app.join("lib/main.dart").is_file());
    assert_analyze_no_errors(&log, &app, "integrate-frb: analyze");
    oxide(
        &log,
        dir.path(),
        &["doctor".to_string(), "--root".to_string(), s(&app)],
    )
    .success();
    native_build(&log, &app, "integrate-frb");
    log.step("=== e2e_integrate_frb_app_preserves_user_code_and_builds DONE");
}
