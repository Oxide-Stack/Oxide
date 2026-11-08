#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ExitCode {
    Ok = 0,
    Error = 1,
    Usage = 2,
    Toolchain = 3,
    Drift = 4,
    Skew = 5,
}

impl ExitCode {
    pub fn code(self) -> i32 {
        self as i32
    }

    pub fn exit(self) -> ! {
        std::process::exit(self.code())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CliError {
    Usage,
    Toolchain,
    Drift,
    Skew,
    Failure,
}

impl CliError {
    pub fn exit_code(self) -> ExitCode {
        match self {
            CliError::Usage => ExitCode::Usage,
            CliError::Toolchain => ExitCode::Toolchain,
            CliError::Drift => ExitCode::Drift,
            CliError::Skew => ExitCode::Skew,
            CliError::Failure => ExitCode::Error,
        }
    }
}

impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            CliError::Usage => "usage error",
            CliError::Toolchain => "toolchain error",
            CliError::Drift => "generated-output drift",
            CliError::Skew => "version skew",
            CliError::Failure => "error",
        };
        f.write_str(s)
    }
}

impl std::error::Error for CliError {}

pub fn usage_err(msg: impl Into<String>) -> anyhow::Error {
    anyhow::Error::new(CliError::Usage).context(msg.into())
}

pub fn toolchain_err(msg: impl Into<String>) -> anyhow::Error {
    anyhow::Error::new(CliError::Toolchain).context(msg.into())
}

pub fn drift_err(msg: impl Into<String>) -> anyhow::Error {
    anyhow::Error::new(CliError::Drift).context(msg.into())
}

pub fn skew_err(msg: impl Into<String>) -> anyhow::Error {
    anyhow::Error::new(CliError::Skew).context(msg.into())
}

pub fn code_of(err: &anyhow::Error) -> ExitCode {
    if let Some(kind) = err.downcast_ref::<CliError>() {
        return kind.exit_code();
    }
    for cause in err.chain().skip(1) {
        if let Some(kind) = cause.downcast_ref::<CliError>() {
            return kind.exit_code();
        }
    }
    ExitCode::Error
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discriminants_match_spec() {
        assert_eq!(ExitCode::Ok.code(), 0);
        assert_eq!(ExitCode::Error.code(), 1);
        assert_eq!(ExitCode::Usage.code(), 2);
        assert_eq!(ExitCode::Toolchain.code(), 3);
        assert_eq!(ExitCode::Drift.code(), 4);
        assert_eq!(ExitCode::Skew.code(), 5);
    }

    #[test]
    fn error_kinds_map_to_codes() {
        assert_eq!(code_of(&usage_err("bad flag")), ExitCode::Usage);
        assert_eq!(code_of(&toolchain_err("no flutter")), ExitCode::Toolchain);
        assert_eq!(code_of(&drift_err("stale")), ExitCode::Drift);
        assert_eq!(code_of(&skew_err("mismatch")), ExitCode::Skew);
        assert_eq!(code_of(&anyhow::anyhow!("plain")), ExitCode::Error);
    }
}
