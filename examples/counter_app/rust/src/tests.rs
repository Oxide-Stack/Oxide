use crate::api::bridge::{create_engine, current, dispatch};
use crate::state::app_action::AppAction;

#[tokio::test]
async fn dispatch_updates_counter_and_revision() {
    let engine = create_engine();
    let before = current(&engine).await;
    assert_eq!(before.state.counter, 0);
    assert_eq!(before.revision, 0);

    let after = dispatch(&engine, AppAction::Increment).await.expect("dispatch");
    assert_eq!(after.state.counter, 1);
    assert_eq!(after.revision, 1);
}

#[tokio::test]
async fn reset_sets_counter_to_zero() {
    let engine = create_engine();
    let _ = dispatch(&engine, AppAction::Increment).await.expect("dispatch");
    let after = dispatch(&engine, AppAction::Reset).await.expect("dispatch");
    assert_eq!(after.state.counter, 0);
}
