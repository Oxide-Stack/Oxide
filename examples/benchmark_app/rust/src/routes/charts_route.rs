use oxide_core::navigation::{NoExtra, NoReturn, Route};
use serde::{Deserialize, Serialize};

/// Route that displays benchmark charts.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChartsRoute {}

impl Route for ChartsRoute {
    type Return = NoReturn;
    type Extra = NoExtra;
}
