use oxide_generator_rs::actions;

#[actions]
pub enum PostsAction {
    LoadForUser { user_id: u64 },
    Refresh,
    SelectPost { post_id: u64 },
}

