use oxide_core::navigation::{NoExtra, NoReturn, Route};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BenchDetailRoute {
    pub id: u64,
}

impl Route for BenchDetailRoute {
    fn path() -> Option<&'static str> {
        Some("/bench/:id")
    }

    fn params(&self) -> HashMap<&'static str, String> {
        HashMap::from([("id", self.id.to_string())])
    }

    type Return = NoReturn;
    type Extra = NoExtra;
}

