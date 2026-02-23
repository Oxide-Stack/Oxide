use serde::{Deserialize, Serialize};

#[oxide_generator_rs::oxide_route(path = "/bench/:id")]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BenchDetailRoute {
    #[route(kind = "param")]
    pub id: u64,
}
