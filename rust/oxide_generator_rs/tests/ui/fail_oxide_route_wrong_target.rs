use oxide_generator_rs::oxide_route;

#[oxide_route(path = "/x")]
pub enum NotAStruct {
    V,
}

fn main() {}
