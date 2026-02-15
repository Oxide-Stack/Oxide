use oxide_core::navigation::{NoExtra, NoReturn, Route};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RoutingBenchRoute {}

impl Route for RoutingBenchRoute {
    fn path() -> Option<&'static str> {
        Some("/routing")
    }

    type Return = NoReturn;
    type Extra = NoExtra;
}

