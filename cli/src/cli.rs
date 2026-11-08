use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(
    name = "oxide",
    version,
    about = "Oxide developer CLI for Flutter+Rust (FRB) apps"
)]
pub struct OxideCli {
    #[arg(long, global = true)]
    pub verbose: bool,
    #[arg(long, global = true, value_name = "DIR")]
    pub root: Option<PathBuf>,
    #[arg(long, global = true)]
    pub yes: bool,
    #[arg(long, global = true)]
    pub dry_run: bool,
    #[arg(long, global = true)]
    pub no_color: bool,
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Create(CreateArgs),
    Integrate(IntegrateArgs),
    Generate(GenerateArgs),
    Doctor(DoctorArgs),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, ValueEnum)]
#[value(rename_all = "lowercase")]
pub enum Backend {
    #[default]
    Inherited,
    Riverpod,
    Bloc,
    Hooks,
}

impl std::fmt::Display for Backend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Backend::Inherited => "inherited",
            Backend::Riverpod => "riverpod",
            Backend::Bloc => "bloc",
            Backend::Hooks => "hooks",
        };
        f.write_str(s)
    }
}

/// How a generated project should depend on the Oxide packages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, ValueEnum)]
#[value(rename_all = "lowercase")]
pub enum OxideSource {
    /// Resolve automatically: `path` when run inside the Oxide checkout, `pub`
    /// otherwise.
    #[default]
    Auto,
    /// Always use pub.dev version constraints.
    Pub,
    /// Always use a relative path into the local checkout.
    Path,
    /// Always use a git dependency on the upstream repository.
    Git,
}

impl std::fmt::Display for OxideSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl OxideSource {
    /// Wire spelling passed to the `create` / `integrate` commands.
    pub fn as_str(self) -> &'static str {
        match self {
            OxideSource::Auto => "auto",
            OxideSource::Pub => "pub",
            OxideSource::Path => "path",
            OxideSource::Git => "git",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, ValueEnum)]
#[value(rename_all = "lowercase")]
pub enum ExampleKind {
    #[default]
    Counter,
    None,
}

impl std::fmt::Display for ExampleKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ExampleKind::Counter => "counter",
            ExampleKind::None => "none",
        };
        f.write_str(s)
    }
}

#[derive(Debug, Parser)]
pub struct CreateArgs {
    pub name: String,
    #[arg(long)]
    pub org: Option<String>,
    #[arg(long)]
    pub rust_crate_name: Option<String>,
    /// The Rust crate directory is fixed at `rust/`; see the note on
    /// `create::CreateArgs::rust_crate_dir`.
    #[arg(long, default_value = "rust", hide = true)]
    pub rust_crate_dir: String,
    #[arg(long, value_enum, default_value_t = Backend::Inherited)]
    pub backend: Backend,
    #[arg(long, value_enum, default_value_t = OxideSource::Pub)]
    pub oxide_source: OxideSource,
    #[arg(long)]
    pub oxide_path: Option<PathBuf>,
    #[arg(long)]
    pub no_codegen: bool,
    #[arg(long)]
    pub keep_frb_example: bool,
    #[arg(long)]
    pub verify: bool,
    #[arg(long)]
    pub dry_run: bool,
    #[arg(long)]
    pub yes: bool,
}

#[derive(Debug, Parser)]
pub struct IntegrateArgs {
    #[arg(long, value_enum, default_value_t = Backend::Inherited)]
    pub backend: Backend,
    #[arg(long, value_enum, default_value_t = OxideSource::Pub)]
    pub oxide_source: OxideSource,
    #[arg(long)]
    pub oxide_path: Option<PathBuf>,
    #[arg(long, value_enum, default_value_t = ExampleKind::Counter)]
    pub example: ExampleKind,
    #[arg(
        long = "init-main",
        default_value_t = true,
        overrides_with = "no_init_main"
    )]
    pub init_main: bool,
    #[arg(long = "no-init-main", overrides_with = "init_main")]
    pub no_init_main: bool,
    #[arg(long)]
    pub keep_frb_example: bool,
    #[arg(long)]
    pub skip_frb: bool,
    #[arg(long)]
    pub no_codegen: bool,
    #[arg(long)]
    pub dry_run: bool,
    #[arg(long)]
    pub yes: bool,
}

impl IntegrateArgs {
    pub fn effective_init_main(&self) -> bool {
        self.init_main && !self.no_init_main
    }
}

#[derive(Debug, Parser)]
pub struct GenerateArgs {
    #[arg(long)]
    pub watch: bool,
    #[arg(long)]
    pub check: bool,
    #[arg(long)]
    pub frb_only: bool,
    #[arg(long)]
    pub oxide_only: bool,
    #[arg(long)]
    pub no_pub_get: bool,
    #[arg(long, value_name = "FILE")]
    pub config_file: Option<PathBuf>,
    #[arg(last = true, value_name = "FRB_ARGS")]
    pub frb_args: Vec<String>,
}

#[derive(Debug, Parser)]
pub struct DoctorArgs {
    #[arg(long)]
    pub json: bool,
    #[arg(long)]
    pub full: bool,
    #[arg(long, value_name = "DIR")]
    pub root: Option<PathBuf>,
    #[arg(long)]
    pub all_examples: bool,
}
