use oxide_core::navigation::{NoExtra, NoReturn, Route};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CounterDetailRoute {
    pub start: u64,
}

impl Route for CounterDetailRoute {
    type Return = NoReturn;
    type Extra = NoExtra;
}

