use serde::{Deserialize, Serialize};

#[oxide_generator_rs::oxide_route(path = "/routing")]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RoutingBenchRoute {}
