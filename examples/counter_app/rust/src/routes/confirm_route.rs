use oxide_core::navigation::{NoExtra, Route};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConfirmRoute {
    pub title: String,
}

impl Route for ConfirmRoute {
    type Return = bool;
    type Extra = NoExtra;
}

