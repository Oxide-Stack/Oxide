use oxide_generator_rs::oxide_route;

#[oxide_route(path = "/x" return = i32)]
pub struct MyRoute;

fn main() {}
