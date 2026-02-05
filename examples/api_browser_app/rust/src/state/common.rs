use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoadPhase {
    Idle,
    Loading,
    Ready,
    Error { message: String },
}

