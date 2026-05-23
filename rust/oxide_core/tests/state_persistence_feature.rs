#![cfg(feature = "state-persistence")]

use oxide_core::persistence;
use oxide_core::persistence::{
    PersistenceConfig, default_persistence_debug_json_path, default_persistence_path,
};
use oxide_core::{CoreResult, InitContext, Reducer, ReducerEngine, StateChange};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
struct Model {
    counter: u64,
    label: String,
}

#[derive(Debug, Clone)]
enum Action {
    Set(u64),
}

enum SideEffect {}

#[derive(Default)]
struct ReducerImpl;

impl Reducer for ReducerImpl {
    type State = Model;
    type Action = Action;
    type SideEffect = SideEffect;

    async fn init(&mut self, _ctx: InitContext<Self::SideEffect>) {}

    fn reduce(
        &mut self,
        state: &mut Self::State,
        ctx: oxide_core::Context<'_, Self::Action, Self::State, ()>,
    ) -> CoreResult<StateChange> {
        match ctx.input {
            Action::Set(value) => {
                state.counter = *value;
                Ok(StateChange::Full)
            }
        }
    }

    fn effect(
        &mut self,
        _state: &mut Self::State,
        _ctx: oxide_core::Context<'_, Self::SideEffect, Self::State, ()>,
    ) -> CoreResult<StateChange> {
        Ok(StateChange::None)
    }
}

#[test]
fn persistence_round_trip_model() {
    let value = Model {
        counter: 42,
        label: "hello".to_string(),
    };

    let bytes = persistence::encode(&value).expect("encode");
    let decoded: Model = persistence::decode(&bytes).expect("decode");
    assert_eq!(decoded, value);
}

#[test]
fn persistence_decode_rejects_invalid_payload() {
    let bytes = vec![0_u8, 1, 2, 3];
    let decoded: Result<Model, _> = persistence::decode(&bytes);
    assert!(decoded.is_err());
}

#[tokio::test]
async fn debug_json_copy_matches_bincode_payload() {
    struct DebugJsonFlagGuard;
    impl Drop for DebugJsonFlagGuard {
        fn drop(&mut self) {
            persistence::set_debug_json_enabled(false);
        }
    }

    fn thread_pool() -> &'static flutter_rust_bridge::SimpleThreadPool {
        static POOL: std::sync::OnceLock<flutter_rust_bridge::SimpleThreadPool> =
            std::sync::OnceLock::new();
        POOL.get_or_init(flutter_rust_bridge::SimpleThreadPool::default)
    }
    let _ = oxide_core::runtime::init(thread_pool);
    #[cfg(feature = "navigation-binding")]
    let _ = oxide_core::init_navigation();

    persistence::set_debug_json_enabled(true);
    let _debug_json_guard = DebugJsonFlagGuard;

    let key = format!(
        "oxide_core.test.debug_json_copy.v1.{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system clock before unix epoch")
            .as_nanos()
    );
    let bin_path = default_persistence_path(&key);
    let json_path = default_persistence_debug_json_path(&key);
    let _ = std::fs::remove_file(&bin_path);
    let _ = std::fs::remove_file(&json_path);

    let engine = ReducerEngine::<ReducerImpl>::new_persistent(
        ReducerImpl::default(),
        Model {
            counter: 0,
            label: "init".to_string(),
        },
        PersistenceConfig {
            key: key.clone(),
            min_interval: std::time::Duration::from_millis(0),
        },
    )
    .await
    .unwrap();

    let error_rx = engine.subscribe_errors();

    let deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(8);
    for value in 1..=12 {
        let _ = engine
            .dispatch(Action::Set(value))
            .await
            .expect("dispatch");

        let mut poll_count = 0;
        while poll_count < 8 {
            if std::fs::metadata(&bin_path)
                .map(|m| m.len() > 0)
                .unwrap_or(false)
                && std::fs::metadata(&json_path)
                    .map(|m| m.len() > 0)
                    .unwrap_or(false)
            {
                break;
            }
            if tokio::time::Instant::now() >= deadline {
                break;
            }
            poll_count += 1;
            tokio::time::sleep(std::time::Duration::from_millis(20)).await;
        }

        if std::fs::metadata(&bin_path)
            .map(|m| m.len() > 0)
            .unwrap_or(false)
            && std::fs::metadata(&json_path)
                .map(|m| m.len() > 0)
                .unwrap_or(false)
        {
            break;
        }
        if tokio::time::Instant::now() >= deadline {
            break;
        }
    }

    if let Some(err) = error_rx.borrow().as_ref() {
        panic!("persistence error while writing debug JSON copy: {err}");
    }

    assert!(
        std::fs::metadata(&bin_path)
            .map(|m| m.len() > 0)
            .unwrap_or(false),
        "bincode persistence file was not written: {}",
        bin_path.display()
    );
    assert!(
        std::fs::metadata(&json_path)
            .map(|m| m.len() > 0)
            .unwrap_or(false),
        "debug json persistence file was not written: {}",
        json_path.display()
    );

    let bin_bytes = std::fs::read(&bin_path).expect("read bincode");
    let json_bytes = std::fs::read(&json_path).expect("read json");
    let bin_state: Model = persistence::decode(&bin_bytes).expect("decode bincode");
    let json_state: Model = serde_json::from_slice(&json_bytes).expect("decode json");
    assert_eq!(bin_state, json_state);

    let _ = std::fs::remove_file(&bin_path);
    let _ = std::fs::remove_file(&json_path);
}
