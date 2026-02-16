use oxide_generator_rs::state;
use serde::{Deserialize, Serialize};

use crate::state::common::LoadPhase;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub username: String,
}

#[state]
pub struct UsersState {
    pub phase: LoadPhase,
    pub users: Vec<User>,
    pub selected_user_id: Option<u64>,
}

