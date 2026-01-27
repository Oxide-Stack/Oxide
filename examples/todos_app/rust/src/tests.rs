use crate::api::bridge::{create_engine, current, dispatch};
use crate::state::AppAction;
use oxide_core::OxideError;

fn reset_persistence_file() {
    let path = oxide_core::persistence::default_persistence_path("oxide.todos.state.v1");
    let _ = std::fs::remove_file(path);
}

#[tokio::test]
async fn add_todo_creates_item() {
    reset_persistence_file();
    let engine = create_engine();
    let snapshot = dispatch(
        &engine,
        AppAction::AddTodo {
            title: "buy milk".to_string(),
        },
    )
    .await
    .expect("dispatch");
    assert_eq!(snapshot.state.todos.len(), 1);
    assert_eq!(snapshot.state.todos[0].title, "buy milk");
    assert!(!snapshot.state.todos[0].completed);
}

#[tokio::test]
async fn add_empty_title_returns_validation_error_and_preserves_state() {
    reset_persistence_file();
    let engine = create_engine();
    let before = current(&engine).await;

    let err = dispatch(
        &engine,
        AppAction::AddTodo {
            title: "   ".to_string(),
        },
    )
    .await;
    assert!(matches!(err, Err(OxideError::Validation { .. })));

    let after = current(&engine).await;
    assert_eq!(after.revision, before.revision);
    assert_eq!(after.state, before.state);
}

#[tokio::test]
async fn persistence_restores_state_across_engines() {
    reset_persistence_file();

    {
        let engine = create_engine();
        let _ = dispatch(
            &engine,
            AppAction::AddTodo {
                title: "persist me".to_string(),
            },
        )
        .await
        .expect("dispatch");
    }

    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    let engine = create_engine();
    let snapshot = current(&engine).await;
    assert_eq!(snapshot.state.todos.len(), 1);
    assert_eq!(snapshot.state.todos[0].title, "persist me");
}
