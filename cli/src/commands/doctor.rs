//! `oxide doctor` command: read-only project health check.

use std::fs;
use std::path::{Path, PathBuf};

use clap::Args;

use crate::doctor::{self, Options};

/// Arguments for `oxide doctor`.
#[derive(Debug, Args)]
pub struct DoctorArgs {
    /// Project root to diagnose (defaults to the current directory).
    #[arg(long, default_value = ".")]
    pub root: PathBuf,
    /// Emit the raw check array as JSON.
    #[arg(long)]
    pub json: bool,
    /// Also run the slow `flutter doctor` summary.
    #[arg(long)]
    pub full: bool,
    /// Diagnose every `examples/*/` project (expects `examples/` under root).
    #[arg(long = "all-examples")]
    pub all_examples: bool,
}

/// Run doctor and return the process exit code (0/3/4/5).
pub fn run(args: DoctorArgs) -> anyhow::Result<i32> {
    let root = canonical_root(&args.root)?;
    let opts = Options {
        full: args.full,
        path_override: None,
    };
    let checks = if args.all_examples {
        run_all_examples(&root, &opts)?
    } else {
        doctor::run_all(&root, &opts)
    };
    if args.json {
        doctor::print_json(&checks)?;
    } else {
        doctor::print_human(&checks);
    }
    Ok(doctor::exit_code(&checks))
}

fn canonical_root(root: &Path) -> anyhow::Result<PathBuf> {
    let joined = if root.is_absolute() {
        root.to_path_buf()
    } else {
        std::env::current_dir()?.join(root)
    };
    Ok(joined.canonicalize().unwrap_or(joined))
}

/// Toolchain checks once against the repo root, project + version checks per
/// example with ids prefixed as `<example>:<check-id>`.
fn run_all_examples(repo_root: &Path, opts: &Options) -> anyhow::Result<Vec<doctor::Check>> {
    let examples = repo_root.join("examples");
    if !examples.is_dir() {
        anyhow::bail!(
            "--all-examples: 'examples/' not found under {}",
            repo_root.display()
        );
    }
    let mut dirs: Vec<PathBuf> = fs::read_dir(&examples)?
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.is_dir() && p.join("pubspec.yaml").is_file())
        .collect();
    dirs.sort();
    if dirs.is_empty() {
        anyhow::bail!(
            "--all-examples: no projects found under {}",
            examples.display()
        );
    }
    let mut out = doctor::toolchain::checks(repo_root, opts);
    for dir in dirs {
        let name = dir
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "?".to_string());
        // Toolchain checks are per-machine, not per-project, and they spawn
        // processes. Running them once here instead of once per example is what
        // keeps `--all-examples` from re-probing flutter/dart/rustc N+1 times.
        let mut checks = doctor::run_project_checks(&dir);
        doctor::prefix_ids(&mut checks, &name);
        out.extend(checks);
    }
    Ok(out)
}
