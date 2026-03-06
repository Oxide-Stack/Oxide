use serde::{Deserialize, Serialize};

/// Main/home route for the API browser app.
#[oxide_generator_rs::oxide_route()]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HomeRoute {}
