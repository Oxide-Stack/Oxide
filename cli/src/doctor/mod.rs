//! Read-only health checks for `oxide doctor`.
//!
//! Checks produce plain [`Check`] data rendered as a human table or JSON.
//! Doctor never modifies the project (no `--fix` in v1).
//!
//! Exit codes: `0` healthy, `3` missing prerequisite, `4` check failure,
//! `5` version skew.

pub mod project;
pub mod toolchain;
pub mod versions;

use std::path::Path;

use serde::{Deserialize, Serialize};

/// Pinned `flutter_rust_bridge` version. Re-exported from [`crate::common`],
/// which is the single source of truth across the CLI.
pub use crate::common::{FRB_PIN, MSRV};

pub const EXIT_OK: i32 = 0;
pub const EXIT_MISSING: i32 = 3;
pub const EXIT_FAILURE: i32 = 4;
pub const EXIT_SKEW: i32 = 5;

/// Severity of a single check.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Pass,
    Warn,
    Fail,
}

/// One doctor finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Check {
    pub id: String,
    pub label: String,
    pub status: Status,
    pub detail: String,
    pub hint: String,
    /// How this check maps onto a process exit code. Set explicitly by the
    /// check that produced it, never inferred from `detail` text.
    pub failure: FailureKind,
}

impl Check {
    pub fn new(
        id: &str,
        label: &str,
        status: Status,
        detail: impl Into<String>,
        hint: impl Into<String>,
    ) -> Self {
        Self {
            id: id.to_string(),
            label: label.to_string(),
            status,
            detail: detail.into(),
            hint: hint.into(),
            failure: FailureKind::None,
        }
    }

    pub fn pass(id: &str, label: &str, detail: impl Into<String>) -> Self {
        Self::new(id, label, Status::Pass, detail, "")
    }

    pub fn warn(id: &str, label: &str, detail: impl Into<String>, hint: impl Into<String>) -> Self {
        Self::new(id, label, Status::Warn, detail, hint)
    }

    pub fn fail(id: &str, label: &str, detail: impl Into<String>, hint: impl Into<String>) -> Self {
        Self::new(id, label, Status::Fail, detail, hint)
    }

    /// A failure meaning "this tool is not installed". Exit [`EXIT_MISSING`].
    pub fn missing_tool(
        id: &str,
        label: &str,
        detail: impl Into<String>,
        hint: impl Into<String>,
    ) -> Self {
        let mut c = Self::fail(id, label, detail, hint);
        c.failure = FailureKind::MissingTool;
        c
    }

    /// A failure meaning "a version does not match the pin". Exit [`EXIT_SKEW`].
    pub fn skew(id: &str, label: &str, detail: impl Into<String>, hint: impl Into<String>) -> Self {
        let mut c = Self::fail(id, label, detail, hint);
        c.failure = FailureKind::Skew;
        c
    }
}

/// Options shared by all check groups.
#[derive(Debug, Clone, Default)]
pub struct Options {
    /// Run the slow `flutter doctor` summary.
    pub full: bool,
    /// If set, tool lookup is restricted to this directory (test seam for
    /// fake-PATH tests; production uses the real `PATH`).
    pub path_override: Option<std::path::PathBuf>,
}

/// Run the toolchain-independent check groups against `root`.
///
/// Deliberately excludes [`toolchain`]. The toolchain group is the same for
/// every project on a machine, and it is the only group that spawns processes.
/// It used to be part of `run_all`, so `--all-examples` ran `flutter --version`,
/// `dart --version`, `rustc --version` and
/// `flutter_rust_bridge_codegen --version` once per example — seven times for
/// six examples, with identical output under ids like
/// `counter_app:toolchain.flutter`, and no timeout on any of them. On a cold
/// Flutter SDK a single `flutter --version` can take minutes.
pub fn run_project_checks(root: &Path) -> Vec<Check> {
    let mut out = project::checks(root);
    out.extend(versions::checks(root));
    out
}

/// Run every check group against `root`, toolchain included.
pub fn run_all(root: &Path, opts: &Options) -> Vec<Check> {
    let mut out = toolchain::checks(root, opts);
    out.extend(run_project_checks(root));
    out
}

/// Prefix check ids (used by `--all-examples`: `counter_app:project.lib-rs`).
pub fn prefix_ids(checks: &mut [Check], prefix: &str) {
    for c in checks {
        c.id = format!("{prefix}:{}", c.id);
    }
}

/// How a failing check should affect the process exit code.
///
/// Classifying by scanning `detail`/`hint` prose for words like "missing" was
/// how a wiring bug whose message happened to read "lib/src/oxide.dart missing"
/// got reported as "install a prerequisite" (exit 3) instead of a check failure
/// (exit 4). The class is set explicitly instead.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FailureKind {
    /// Not a failure; classification is irrelevant for a passing check.
    #[default]
    None,
    /// A required tool is not installed. Exit [`EXIT_MISSING`].
    MissingTool,
    /// A declared version does not match the pin. Exit [`EXIT_SKEW`].
    Skew,
    /// Any other check failure. Exit [`EXIT_FAILURE`].
    Failed,
}

fn looks_missing(c: &Check) -> bool {
    c.failure == FailureKind::MissingTool
}

/// Skew = an FRB pin check that failed for a real version mismatch
/// (missing tools are [`EXIT_MISSING`], not skew).
fn is_skew(c: &Check) -> bool {
    c.status == Status::Fail && c.failure == FailureKind::Skew
}

/// Map check results to a process exit code.
///
/// Precedence is skew, then missing tool, then generic failure. Skew wins because
/// a version mismatch is the more actionable diagnosis: a machine that is also
/// missing a tool is usually mid-install, and reporting the skew first is what
/// the pin checks exist for.
pub fn exit_code(checks: &[Check]) -> i32 {
    if checks.iter().any(is_skew) {
        return EXIT_SKEW;
    }
    if checks
        .iter()
        .any(|c| c.status == Status::Fail && looks_missing(c))
    {
        return EXIT_MISSING;
    }
    if checks.iter().any(|c| c.status == Status::Fail) {
        return EXIT_FAILURE;
    }
    EXIT_OK
}

/// Print checks as a human-readable table.
pub fn print_human(checks: &[Check]) {
    use console::style;
    let width = checks
        .iter()
        .map(|c| c.id.len())
        .max()
        .unwrap_or(4)
        .max("CHECK".len());
    let header = "CHECK";
    println!("{header:<width$}  RESULT", width = width);
    for c in checks {
        let status = match c.status {
            Status::Pass => style("PASS").green().bold(),
            Status::Warn => style("WARN").yellow().bold(),
            Status::Fail => style("FAIL").red().bold(),
        };
        println!(
            "{:<width$}  {status} {}  {}",
            c.id,
            c.label,
            c.detail,
            width = width
        );
    }
    for c in checks
        .iter()
        .filter(|c| c.status != Status::Pass && !c.hint.is_empty())
    {
        println!("  hint [{}]: {}", c.id, c.hint);
    }
    let (p, w, f) = (
        checks.iter().filter(|c| c.status == Status::Pass).count(),
        checks.iter().filter(|c| c.status == Status::Warn).count(),
        checks.iter().filter(|c| c.status == Status::Fail).count(),
    );
    println!("\n{p} passed, {w} warnings, {f} failures");
}

/// Print checks as a JSON array.
pub fn print_json(checks: &[Check]) -> anyhow::Result<()> {
    println!("{}", serde_json::to_string_pretty(checks)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_serializes_as_array_with_status() {
        let checks = vec![
            Check::pass("toolchain.flutter", "Flutter SDK", "Flutter 3.38.0"),
            Check::fail("project.frb-dep", "FRB dep", "skew", "align pins"),
        ];
        let v = serde_json::to_value(&checks).unwrap();
        assert!(v.is_array());
        assert_eq!(v.as_array().unwrap().len(), 2);
        assert_eq!(v[0]["id"], "toolchain.flutter");
        assert_eq!(v[0]["status"], "pass");
        assert_eq!(v[1]["status"], "fail");
    }

    #[test]
    fn exit_code_prefers_skew_then_missing_then_failure() {
        let missing = Check::missing_tool("toolchain.dart", "Dart", "dart not found", "install");
        let failure = Check::fail("project.init", "Init", "bad wiring", "fix wiring");
        let skew = Check::skew(
            "versions.frb-pin",
            "FRB pin",
            "version skew: pubspec=2.11.0 pin=2.12.0",
            "align",
        );
        assert_eq!(exit_code(std::slice::from_ref(&missing)), EXIT_MISSING);
        assert_eq!(exit_code(std::slice::from_ref(&failure)), EXIT_FAILURE);
        assert_eq!(exit_code(&[missing, failure, skew]), EXIT_SKEW);
        assert_eq!(exit_code(&[Check::pass("a", "b", "c")]), EXIT_OK);
    }

    #[test]
    fn missing_codegen_maps_to_missing_not_skew() {
        let c = Check::missing_tool(
            "toolchain.frb-codegen",
            "FRB codegen",
            "flutter_rust_bridge_codegen not installed",
            "cargo install flutter_rust_bridge_codegen --version 2.12.0 --locked",
        );
        assert!(!is_skew(&c));
        assert_eq!(exit_code(&[c]), EXIT_MISSING);
    }

    /// Regression: the failure class used to be inferred by searching the
    /// `detail`/`hint` prose for the word "missing", so a wiring bug whose
    /// message happened to contain that word was reported as a missing
    /// prerequisite and exited 3 instead of 4.
    #[test]
    fn failure_class_is_not_inferred_from_message_text() {
        let wiring = Check::fail(
            "project.oxide-dart",
            "oxide.dart",
            "lib/src/oxide.dart missing: @OxideStore annotation not found",
            "re-run oxide integrate",
        );
        assert_eq!(wiring.failure, FailureKind::None);
        assert!(!looks_missing(&wiring));
        assert_eq!(exit_code(&[wiring]), EXIT_FAILURE);
    }

    #[test]
    fn warnings_never_affect_the_exit_code() {
        let w = Check::warn("toolchain.fvm", "FVM", "no .fvmrc", "optional");
        assert_eq!(exit_code(&[w]), EXIT_OK);
    }

    #[test]
    fn missing_tool_classification_ignores_prose() {
        // The message says "found", but the class is what counts.
        let c = Check::missing_tool("toolchain.dart", "Dart", "flutter found, dart absent", "");
        assert!(looks_missing(&c));
        assert_eq!(exit_code(&[c]), EXIT_MISSING);
    }
}
