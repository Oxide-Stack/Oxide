use oxide_generator_rs::actions;

#[actions]
pub enum CommentsAction {
    LoadForPost { post_id: u64 },
    Refresh,
}

