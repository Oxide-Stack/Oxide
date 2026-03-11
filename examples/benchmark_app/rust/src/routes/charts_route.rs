use serde::{Deserialize, Serialize};

/// Route that displays benchmark charts.
#[oxide_generator_rs::oxide_route()]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChartsRoute {}
