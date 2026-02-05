use oxide_generator_rs::reducer;

use oxide_core::StateChange;

use crate::util;
use crate::state::common::LoadPhase;
use crate::state::users_action::UsersAction;
use crate::state::users_state::{User, UsersState};

#[derive(Debug)]
#[flutter_rust_bridge::frb(ignore)]
enum UsersSideEffect {
    Fetch,
    Loaded { users: Vec<User> },
    Failed { message: String },
}

#[derive(Default)]
#[flutter_rust_bridge::frb(ignore)]
struct UsersReducer {
    sideeffect_tx: Option<oxide_core::tokio::sync::mpsc::UnboundedSender<UsersSideEffect>>,
}

#[reducer(
    engine = UsersEngine,
    snapshot = UsersStateSnapshot,
    initial = UsersState {
        phase: LoadPhase::Idle,
        users: Vec::new(),
        selected_user_id: None,
    },
)]
impl oxide_core::Reducer for UsersReducer {
    type State = UsersState;
    type Action = UsersAction;
    type SideEffect = UsersSideEffect;

    async fn init(&mut self, ctx: oxide_core::InitContext<Self::SideEffect>) {
        self.sideeffect_tx = Some(ctx.sideeffect_tx);
        if let Some(tx) = self.sideeffect_tx.as_ref() {
            let _ = tx.send(UsersSideEffect::Fetch);
        }
    }

    fn reduce(
        &mut self,
        state: &mut Self::State,
        action: Self::Action,
    ) -> oxide_core::CoreResult<oxide_core::StateChange> {
        match action {
            UsersAction::Refresh => {
                if let Some(tx) = self.sideeffect_tx.as_ref() {
                    let _ = tx.send(UsersSideEffect::Fetch);
                }
                Ok(StateChange::None)
            }
            UsersAction::SelectUser { user_id } => {
                state.selected_user_id = Some(user_id);
                Ok(StateChange::FullUpdate)
            }
        }
    }

    fn effect(
        &mut self,
        state: &mut Self::State,
        effect: Self::SideEffect,
    ) -> oxide_core::CoreResult<oxide_core::StateChange> {
        match effect {
            UsersSideEffect::Fetch => {
                state.phase = LoadPhase::Loading;

                let Some(tx) = self.sideeffect_tx.clone() else {
                    return Ok(StateChange::FullUpdate);
                };

                #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
                oxide_core::runtime::spawn_local(async move {
                    match util::get_json::<Vec<User>>("users").await {
                        Ok(users) => {
                            let _ = tx.send(UsersSideEffect::Loaded { users });
                        }
                        Err(err) => {
                            let _ = tx.send(UsersSideEffect::Failed { message: err.to_string() });
                        }
                    }
                });

                #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
                oxide_core::runtime::spawn(async move {
                    match util::get_json::<Vec<User>>("users").await {
                        Ok(users) => {
                            let _ = tx.send(UsersSideEffect::Loaded { users });
                        }
                        Err(err) => {
                            let _ = tx.send(UsersSideEffect::Failed { message: err.to_string() });
                        }
                    }
                });

                Ok(StateChange::FullUpdate)
            }
            UsersSideEffect::Loaded { users } => {
                state.users = users;
                state.phase = LoadPhase::Ready;
                if state.selected_user_id.is_none() {
                    state.selected_user_id = state.users.first().map(|u| u.id);
                }
                Ok(StateChange::FullUpdate)
            }
            UsersSideEffect::Failed { message } => {
                state.phase = LoadPhase::Error { message };
                Ok(StateChange::FullUpdate)
            }
        }
    }
}

