use oxide_core::navigation::{NoExtra, NoReturn, Route};
use serde::{Deserialize, Serialize};

/// Main/home route for the ticker app.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HomeRoute {}

impl Route for HomeRoute {
    type Return = NoReturn;
    type Extra = NoExtra;
}
