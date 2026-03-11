use serde::{Deserialize, Serialize};

#[oxide_generator_rs::oxide_route(return = bool)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConfirmRoute {
    pub title: String,
}
