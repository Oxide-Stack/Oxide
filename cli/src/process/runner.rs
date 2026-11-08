use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub trait CommandRunner {
    fn run(&self, program: &str, args: &[String], cwd: &Path) -> anyhow::Result<()>;
    fn run_capture(&self, program: &str, args: &[String], cwd: &Path) -> anyhow::Result<String>;
    fn is_dry_run(&self) -> bool;
}

pub struct RealRunner {
    pub dry_run: bool,
    pub verbose: bool,
}

impl RealRunner {
    pub fn new(dry_run: bool, verbose: bool) -> Self {
        Self { dry_run, verbose }
    }
}

pub fn quote_arg(arg: &str) -> String {
    if arg.is_empty() {
        return "\"\"".to_owned();
    }
    let needs_quote = arg.chars().any(|c| {
        c.is_whitespace()
            || matches!(
                c,
                '"' | '\'' | '\\' | '$' | '`' | '!' | '&' | '|' | '<' | '>' | '(' | ')' | ';' | '*'
            )
    });
    if !needs_quote {
        return arg.to_owned();
    }
    format!("\"{}\"", arg.replace('"', "\\\""))
}

pub fn render_command(program: &str, args: &[String]) -> String {
    let mut parts = vec![quote_arg(program)];
    parts.extend(args.iter().map(|a| quote_arg(a)));
    parts.join(" ")
}

pub fn log_command(program: &str, args: &[String], cwd: &Path) {
    println!(
        "$ {}  # in {}",
        render_command(program, args),
        cwd.display()
    );
}

/// Build a `Command` for `program`. On Windows many dev tools are batch shims
/// (`flutter.bat`, `dart.bat`) which `CreateProcess` cannot execute directly,
/// so they are routed through `cmd /C` with the resolved path.
pub fn build_command(program: &str, args: &[String]) -> Command {
    let resolved = which::which(program).unwrap_or_else(|_| PathBuf::from(program));
    let is_script = resolved
        .extension()
        .is_some_and(|e| e.eq_ignore_ascii_case("bat") || e.eq_ignore_ascii_case("cmd"));
    if cfg!(windows) && is_script {
        let mut c = Command::new("cmd");
        c.arg("/C").arg(resolved);
        c.args(args);
        c
    } else {
        let mut c = Command::new(resolved);
        c.args(args);
        c
    }
}

impl CommandRunner for RealRunner {
    fn run(&self, program: &str, args: &[String], cwd: &Path) -> anyhow::Result<()> {
        log_command(program, args, cwd);
        if self.dry_run {
            return Ok(());
        }
        let status = build_command(program, args)
            .current_dir(cwd)
            .stdin(Stdio::null())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .map_err(|e| anyhow::anyhow!("failed to spawn `{program}`: {e}"))?;
        if status.success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "command `{}` failed with {}",
                render_command(program, args),
                status
                    .code()
                    .map(|c| c.to_string())
                    .unwrap_or_else(|| "signal".to_owned())
            ))
        }
    }

    fn run_capture(&self, program: &str, args: &[String], cwd: &Path) -> anyhow::Result<String> {
        if self.verbose {
            log_command(program, args, cwd);
        }
        if self.dry_run {
            return Ok(String::new());
        }
        let output = build_command(program, args)
            .current_dir(cwd)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|e| anyhow::anyhow!("failed to spawn `{program}`: {e}"))?;
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).into_owned())
        } else {
            Err(anyhow::anyhow!(
                "command `{}` failed: {}",
                render_command(program, args),
                String::from_utf8_lossy(&output.stderr).trim()
            ))
        }
    }

    fn is_dry_run(&self) -> bool {
        self.dry_run
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quoting_is_copy_pasteable() {
        assert_eq!(
            render_command("flutter", &["pub".into(), "get".into()]),
            "flutter pub get"
        );
        assert_eq!(quote_arg("C:\\My Dir\\tool"), "\"C:\\My Dir\\tool\"");
        assert_eq!(quote_arg(""), "\"\"");
    }
}
