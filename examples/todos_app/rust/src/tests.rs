use crate::state::AppAction;
use crate::api::bridge::AppRootReducer;
use crate::state::AppState;
use oxide_core::OxideError;
use oxide_core::ReducerEngine;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

static TEST_KEY_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn test_persistence_key(test_name: &str) -> String {
    let id = TEST_KEY_COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("oxide.todos.state.v1.test.{test_name}.{}.{}", std::process::id(), id)
}

fn reset_persistence_file(key: &str) {
    let path = oxide_core::persistence::default_persistence_path(key);
    let _ = std::fs::remove_file(path);
}

async fn create_test_engine(key: &str) -> ReducerEngine<AppRootReducer> {
    crate::api::bridge::init_oxide().await.unwrap();
    ReducerEngine::<AppRootReducer>::new_persistent(
        AppRootReducer::default(),
        AppState::new(),
        oxide_core::persistence::PersistenceConfig {
            key: key.to_string(),
            min_interval: Duration::from_millis(0),
        },
    )
    .await
    .unwrap()
}

async fn wait_for_persistence_file(key: &str) {
    let path = oxide_core::persistence::default_persistence_path(key);
    let deadline = tokio::time::Instant::now() + Duration::from_secs(2);

    // Persistence is written by a background worker thread; polling avoids flaky sleeps in CI.
    loop {
        if let Ok(meta) = std::fs::metadata(&path) {
            if meta.len() > 0 {
                return;
            }
        }

        if tokio::time::Instant::now() >= deadline {
            panic!("timed out waiting for persistence file to be written: {}", path.display());
        }

        tokio::time::sleep(Duration::from_millis(10)).await;
    }
}

#[tokio::test]
async fn add_todo_creates_item() {
    let key = test_persistence_key("add_todo_creates_item");
    reset_persistence_file(&key);
    let engine = create_test_engine(&key).await;
    let snapshot = engine
        .dispatch(
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
    let key = test_persistence_key("add_empty_title_returns_validation_error_and_preserves_state");
    reset_persistence_file(&key);
    let engine = create_test_engine(&key).await;
    let before = engine.current().await;

    let err = engine
        .dispatch(
            AppAction::AddTodo {
                title: "   ".to_string(),
            },
        )
        .await;
    assert!(matches!(err, Err(OxideError::Validation { .. })));

    let after = engine.current().await;
    assert_eq!(after.revision, before.revision);
    assert_eq!(after.state, before.state);
}

#[tokio::test]
async fn persistence_restores_state_across_engines() {
    let key = test_persistence_key("persistence_restores_state_across_engines");
    reset_persistence_file(&key);

    {
        let engine = create_test_engine(&key).await;
        let _ = engine
            .dispatch(
                AppAction::AddTodo {
                    title: "persist me".to_string(),
                },
            )
            .await
            .expect("dispatch");
    }

    wait_for_persistence_file(&key).await;

    let engine = create_test_engine(&key).await;
    let snapshot = engine.current().await;
    assert_eq!(snapshot.state.todos.len(), 1);
    assert_eq!(snapshot.state.todos[0].title, "persist me");
}
