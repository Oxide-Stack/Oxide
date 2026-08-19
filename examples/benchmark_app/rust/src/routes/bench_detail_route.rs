#[oxide_generator_rs::oxide_route(path = "/bench/:id")]
pub struct BenchDetailRoute {
    pub id: u64,
}
