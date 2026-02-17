use std::sync::OnceLock;

use crate::{CoreResult, OxideError};

use super::{OxideChannelError, OxideChannelResult};

static RUNTIME: OnceLock<OxideIsolatedChannelsRuntime> = OnceLock::new();

/// Global runtime marker for the isolated channels feature.
///
/// The runtime currently acts as an explicit initialization boundary and as the
/// stable anchor point for macro-generated glue code.
pub struct OxideIsolatedChannelsRuntime {
    _private: (),
}

/// Initializes the Oxide isolated channels runtime.
///
/// This initialization is explicit to keep startup order deterministic and to
/// ensure the feature has zero runtime impact unless opted into.
pub fn init_isolated_channels() -> CoreResult<()> {
    let _ = RUNTIME.get_or_init(|| OxideIsolatedChannelsRuntime { _private: () });
    Ok(())
}

/// Returns `true` if the isolated channels runtime has been initialized.
pub fn isolated_channels_initialized() -> bool {
    RUNTIME.get().is_some()
}

/// Returns the global isolated channels runtime if initialized.
pub fn isolated_channels_runtime() -> CoreResult<&'static OxideIsolatedChannelsRuntime> {
    RUNTIME.get().ok_or_else(|| OxideError::Internal {
        message: "isolated channels runtime not initialized; call init_isolated_channels() first"
            .into(),
    })
}

/// Returns an error if the isolated channels runtime has not been initialized.
///
/// Generated channel APIs use this guard so missing initialization produces an
/// explicit error instead of a panic.
pub fn ensure_isolated_channels_initialized() -> OxideChannelResult<()> {
    if isolated_channels_initialized() {
        Ok(())
    } else {
        Err(OxideChannelError::Unavailable)
    }
}
