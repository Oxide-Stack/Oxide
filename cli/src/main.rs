//! `oxide` binary: argument parsing and dispatch.
//!
//! All behavior lives in the [`oxide_cli`] library so the integration tests
//! exercise the same code paths the binary runs.

use clap::Parser;

use oxide_cli::cli::{Commands, OxideCli};
use oxide_cli::process::runner::CommandRunner;
use oxide_cli::{cli, commands, exit, process};

struct GenerateRunner<'a> {
    inner: &'a dyn CommandRunner,
}

impl commands::generate::CommandRunner for GenerateRunner<'_> {
    fn run(&self, cmd: &str, args: &[String], cwd: &std::path::Path) -> anyhow::Result<()> {
        self.inner.run(cmd, args, cwd)
    }
}

/// Resolve the `--oxide-source` / `--oxide-path` pair into the wire spelling the
/// commands expect.
///
/// `auto` (the default) resolves to `path` inside the Oxide checkout and `pub`
/// outside it, which is what a bare `oxide create` should do. An explicit
/// `--oxide-source pub` is now distinguishable from the default and is honored
/// as written, so asking for pub.dev dependencies from inside the checkout
/// actually produces pub.dev dependencies instead of relative paths.
///
/// Supplying `--oxide-path` without a source implies `path`, since a path to a
/// local checkout is meaningless for the other modes.
fn oxide_source_flag(source: &cli::OxideSource, oxide_path: &Option<std::path::PathBuf>) -> String {
    match (source, oxide_path) {
        (cli::OxideSource::Auto, Some(_)) => cli::OxideSource::Path.as_str().to_owned(),
        (s, _) => s.as_str().to_owned(),
    }
}

fn run() -> anyhow::Result<i32> {
    let args = OxideCli::parse();
    let out = process::output::Output::new(args.verbose, args.dry_run, args.no_color);
    out.apply();
    let cwd = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    let root = args.root.clone().unwrap_or(cwd);
    out.verbose(&format!("root: {}", root.display()));
    match args.command {
        Commands::Create(mut a) => {
            a.dry_run |= args.dry_run;
            a.yes |= args.yes;
            commands::create::run(commands::create::CreateArgs {
                name: a.name,
                org: a.org,
                rust_crate_name: a.rust_crate_name,
                rust_crate_dir: a.rust_crate_dir,
                backend: a.backend.to_string(),
                oxide_source: oxide_source_flag(&a.oxide_source, &a.oxide_path),
                keep_frb_example: a.keep_frb_example,
                no_codegen: a.no_codegen,
                verify: a.verify,
                dry_run: a.dry_run,
                yes: a.yes,
                oxide_path: a.oxide_path,
                // The resolved root, not the raw option, so `--root` means the
                // same thing for `create` as it does for `integrate` and
                // `doctor`. Passing the raw option made `--root` silently
                // ignored whenever it was absent.
                base_dir: Some(root.clone()),
            })?;
            Ok(0)
        }
        Commands::Integrate(mut a) => {
            a.dry_run |= args.dry_run;
            a.yes |= args.yes;
            let report = commands::integrate::run(commands::integrate::IntegrateArgs {
                path: root.clone(),
                backend: a.backend.to_string(),
                oxide_source: oxide_source_flag(&a.oxide_source, &a.oxide_path),
                example: a.example.to_string(),
                keep_frb_example: a.keep_frb_example,
                no_init_main: a.no_init_main,
                no_codegen: a.no_codegen,
                dry_run: a.dry_run,
                yes: a.yes,
                skip_frb: a.skip_frb,
                oxide_path: a.oxide_path,
            })?;
            out.verbose(&format!(
                "shape: {}, changed {} file(s), noop: {}",
                report.shape,
                report.changed.len(),
                report.noop
            ));
            Ok(0)
        }
        Commands::Generate(a) => {
            let gargs = commands::generate::GenerateArgs {
                root: Some(root.clone()),
                no_pub_get: a.no_pub_get,
                frb_only: a.frb_only,
                oxide_only: a.oxide_only,
                dry_run: args.dry_run,
                watch: a.watch,
                check: a.check,
                config_file: a.config_file,
                frb_args: a.frb_args,
            };
            let runner = process::runner::RealRunner::new(args.dry_run, args.verbose);
            let adapter = GenerateRunner { inner: &runner };
            let code = commands::generate::run(&gargs, &adapter)?;
            Ok(code)
        }
        Commands::Doctor(a) => {
            let dargs = commands::doctor::DoctorArgs {
                root: a.root.clone().unwrap_or(root),
                json: a.json,
                full: a.full,
                all_examples: a.all_examples,
            };
            let code = commands::doctor::run(dargs)?;
            Ok(code)
        }
    }
}

fn main() {
    match run() {
        Ok(code) => std::process::exit(code),
        Err(err) => {
            eprintln!("error: {err:?}");
            exit::code_of(&err).exit();
        }
    }
}
