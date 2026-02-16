use oxide_generator_rs::state;
use serde::{Deserialize, Serialize};

use crate::state::common::LoadPhase;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Comment {
    pub id: u64,
    #[serde(rename = "postId")]
    pub post_id: u64,
    pub name: String,
}

#[state]
pub struct CommentsState {
    pub phase: LoadPhase,
    pub comments: Vec<Comment>,
    pub selected_post_id: Option<u64>,
}

