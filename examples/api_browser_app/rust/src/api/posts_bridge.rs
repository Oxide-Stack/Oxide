use oxide_generator_rs::reducer;

use oxide_core::StateChange;

use crate::util;
use crate::state::common::LoadPhase;
use crate::state::posts_action::PostsAction;
use crate::state::posts_state::{Post, PostsState};

#[derive(Debug)]
#[flutter_rust_bridge::frb(ignore)]
enum PostsSideEffect {
    Fetch { user_id: u64 },
    Loaded { user_id: u64, posts: Vec<Post> },
    Failed { message: String },
}

#[derive(Default)]
#[flutter_rust_bridge::frb(ignore)]
struct PostsReducer {
    sideeffect_tx: Option<oxide_core::tokio::sync::mpsc::UnboundedSender<PostsSideEffect>>,
}

#[reducer(
    engine = PostsEngine,
    snapshot = PostsStateSnapshot,
    initial = PostsState {
        phase: LoadPhase::Idle,
        posts: Vec::new(),
        selected_user_id: None,
        selected_post_id: None,
    },
)]
impl oxide_core::Reducer for PostsReducer {
    type State = PostsState;
    type Action = PostsAction;
    type SideEffect = PostsSideEffect;

    async fn init(&mut self, ctx: oxide_core::InitContext<Self::SideEffect>) {
        self.sideeffect_tx = Some(ctx.sideeffect_tx);
    }

    fn reduce(
        &mut self,
        state: &mut Self::State,
        action: Self::Action,
    ) -> oxide_core::CoreResult<oxide_core::StateChange> {
        match action {
            PostsAction::LoadForUser { user_id } => {
                state.selected_user_id = Some(user_id);
                state.selected_post_id = None;
                if let Some(tx) = self.sideeffect_tx.as_ref() {
                    let _ = tx.send(PostsSideEffect::Fetch { user_id });
                }
                Ok(StateChange::FullUpdate)
            }
            PostsAction::Refresh => {
                if let (Some(user_id), Some(tx)) = (state.selected_user_id, self.sideeffect_tx.as_ref()) {
                    let _ = tx.send(PostsSideEffect::Fetch { user_id });
                }
                Ok(StateChange::None)
            }
            PostsAction::SelectPost { post_id } => {
                state.selected_post_id = Some(post_id);
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
            PostsSideEffect::Fetch { user_id } => {
                state.phase = LoadPhase::Loading;
                state.posts.clear();
                state.selected_post_id = None;

                let Some(tx) = self.sideeffect_tx.clone() else {
                    return Ok(StateChange::FullUpdate);
                };

                #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
                oxide_core::runtime::spawn_local(async move {
                    let path = format!("posts?userId={user_id}");
                    match util::get_json::<Vec<Post>>(&path).await {
                        Ok(posts) => {
                            let _ = tx.send(PostsSideEffect::Loaded { user_id, posts });
                        }
                        Err(err) => {
                            let _ = tx.send(PostsSideEffect::Failed { message: err.to_string() });
                        }
                    }
                });

                #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
                oxide_core::runtime::spawn(async move {
                    let path = format!("posts?userId={user_id}");
                    match util::get_json::<Vec<Post>>(&path).await {
                        Ok(posts) => {
                            let _ = tx.send(PostsSideEffect::Loaded { user_id, posts });
                        }
                        Err(err) => {
                            let _ = tx.send(PostsSideEffect::Failed { message: err.to_string() });
                        }
                    }
                });

                Ok(StateChange::FullUpdate)
            }
            PostsSideEffect::Loaded { user_id, posts } => {
                if state.selected_user_id != Some(user_id) {
                    return Ok(StateChange::None);
                }
                state.posts = posts;
                state.phase = LoadPhase::Ready;
                if state.selected_post_id.is_none() {
                    state.selected_post_id = state.posts.first().map(|p| p.id);
                }
                Ok(StateChange::FullUpdate)
            }
            PostsSideEffect::Failed { message } => {
                state.phase = LoadPhase::Error { message };
                Ok(StateChange::FullUpdate)
            }
        }
    }
}

