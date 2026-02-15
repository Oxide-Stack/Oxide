use oxide_generator_rs::actions;

#[actions]
pub enum UsersAction {
    Refresh,
    SelectUser { user_id: u64 },
}

