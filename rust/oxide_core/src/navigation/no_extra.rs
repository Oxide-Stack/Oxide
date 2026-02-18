use serde::{Deserialize, Serialize};

use crate::navigation::DefaultExtra;

/// Default extra type used by routes that do not define extras.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct NoExtra;

impl DefaultExtra for NoExtra {
    fn default_extra() -> Self {
        Self
    }
}
