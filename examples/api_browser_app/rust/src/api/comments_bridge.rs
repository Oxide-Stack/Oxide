use oxide_generator_rs::reducer;

use oxide_core::StateChange;

use crate::util;
use crate::state::comments_action::CommentsAction;
use crate::state::comments_state::{Comment, CommentsState};
use crate::state::common::LoadPhase;

#[derive(Debug)]
#[flutter_rust_bridge::frb(ignore)]
enum CommentsSideEffect {
    Fetch { post_id: u64 },
    Loaded { post_id: u64, comments: Vec<Comment> },
    Failed { message: String },
}

#[derive(Default)]
#[flutter_rust_bridge::frb(ignore)]
struct CommentsReducer {
    sideeffect_tx: Option<oxide_core::tokio::sync::mpsc::UnboundedSender<CommentsSideEffect>>,
}

#[reducer(
    engine = CommentsEngine,
    snapshot = CommentsStateSnapshot,
    initial = CommentsState {
        phase: LoadPhase::Idle,
        comments: Vec::new(),
        selected_post_id: None,
    },
)]
impl oxide_core::Reducer for CommentsReducer {
    type State = CommentsState;
    type Action = CommentsAction;
    type SideEffect = CommentsSideEffect;

    async fn init(&mut self, ctx: oxide_core::InitContext<Self::SideEffect>) {
        self.sideeffect_tx = Some(ctx.sideeffect_tx);
    }

    fn reduce(
        &mut self,
        state: &mut Self::State,
        action: Self::Action,
    ) -> oxide_core::CoreResult<oxide_core::StateChange> {
        match action {
            CommentsAction::LoadForPost { post_id } => {
                state.selected_post_id = Some(post_id);
                if let Some(tx) = self.sideeffect_tx.as_ref() {
                    let _ = tx.send(CommentsSideEffect::Fetch { post_id });
                }
                Ok(StateChange::Full)
            }
            CommentsAction::Refresh => {
                if let (Some(post_id), Some(tx)) = (state.selected_post_id, self.sideeffect_tx.as_ref()) {
                    let _ = tx.send(CommentsSideEffect::Fetch { post_id });
                }
                Ok(StateChange::None)
            }
        }
    }

    fn effect(
        &mut self,
        state: &mut Self::State,
        effect: Self::SideEffect,
    ) -> oxide_core::CoreResult<oxide_core::StateChange> {
        match effect {
            CommentsSideEffect::Fetch { post_id } => {
                state.phase = LoadPhase::Loading;
                state.comments.clear();

                let Some(tx) = self.sideeffect_tx.clone() else {
                    return Ok(StateChange::Full);
                };

                oxide_core::runtime::safe_spawn(async move {
                    let path = format!("comments?postId={post_id}");
                    match util::get_json::<Vec<Comment>>(&path).await {
                        Ok(comments) => {
                            let _ = tx.send(CommentsSideEffect::Loaded { post_id, comments });
                        }
                        Err(err) => {
                            let _ = tx.send(CommentsSideEffect::Failed { message: err.to_string() });
                        }
                    }
                });

              

                Ok(StateChange::Full)
            }
            CommentsSideEffect::Loaded { post_id, comments } => {
                if state.selected_post_id != Some(post_id) {
                    return Ok(StateChange::None);
                }
                state.comments = comments;
                state.phase = LoadPhase::Ready;
                Ok(StateChange::Full)
            }
            CommentsSideEffect::Failed { message } => {
                state.phase = LoadPhase::Error { message };
                Ok(StateChange::Full)
            }
        }
    }
}

