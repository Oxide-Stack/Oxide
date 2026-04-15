#[oxide_generator_rs::oxide_route(path = "/users/:userId")]
pub struct UserDetailRoute {
    pub user_id: u64,
}
