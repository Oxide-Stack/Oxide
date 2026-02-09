use oxide_core::navigation::{NoExtra, NoReturn, Route};
use serde::{Deserialize, Serialize};

/// Initial splash route for the counter app.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SplashRoute {}

impl Route for SplashRoute {
    type Return = NoReturn;
    type Extra = NoExtra;
}
