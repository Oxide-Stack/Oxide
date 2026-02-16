use oxide_generator_rs::state;
use serde::{Deserialize, Serialize};

use crate::state::common::LoadPhase;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Post {
    pub id: u64,
    #[serde(rename = "userId")]
    pub user_id: u64,
    pub title: String,
}

#[state]
pub struct PostsState {
    pub phase: LoadPhase,
    pub posts: Vec<Post>,
    pub selected_user_id: Option<u64>,
    pub selected_post_id: Option<u64>,
}

