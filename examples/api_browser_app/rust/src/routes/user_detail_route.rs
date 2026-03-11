use serde::{Deserialize, Serialize};

#[oxide_generator_rs::oxide_route(path = "/users/:userId")]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserDetailRoute {
    pub user_id: u64,
}
