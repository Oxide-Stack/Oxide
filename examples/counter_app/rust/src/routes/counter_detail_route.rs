use serde::{Deserialize, Serialize};

#[oxide_generator_rs::oxide_route]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CounterDetailRoute {
    pub start: u64,
}
