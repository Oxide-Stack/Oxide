use oxide_core::navigation::{NoExtra, NoReturn, Route};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserDetailRoute {
    pub user_id: u64,
}

impl Route for UserDetailRoute {
    fn path() -> Option<&'static str> {
        Some("/users/:userId")
    }

    fn params(&self) -> HashMap<&'static str, String> {
        HashMap::from([("userId", self.user_id.to_string())])
    }

    type Return = NoReturn;
    type Extra = NoExtra;
}

