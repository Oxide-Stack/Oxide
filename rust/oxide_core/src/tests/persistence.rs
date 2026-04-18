use super::*;
use tokio_stream::{StreamExt, wrappers::WatchStream};

#[tokio::test]
async fn engine_reports_persistence_encode_failures() {
    use crate::persistence::PersistenceConfig;
    use crate::serde::{Deserialize, Serialize};

    #[derive(Clone)]
    struct BadState;

    impl Serialize for BadState {
        fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
        where
            S: crate::serde::Serializer,
        {
            use crate::serde::ser::Error as _;
            Err(S::Error::custom("boom"))
        }
    }

    impl<'de> Deserialize<'de> for BadState {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: crate::serde::Deserializer<'de>,
        {
            let _ = crate::serde::de::IgnoredAny::deserialize(deserializer)?;
            Ok(BadState)
        }
    }

    #[derive(Default)]
    struct BadReducer;

    impl Reducer for BadReducer {
        type State = BadState;
        type Action = ();
        type SideEffect = ();

        async fn init(&mut self, _ctx: InitContext<Self::SideEffect>) {}

        fn reduce(
            &mut self,
            _state: &mut Self::State,
            _ctx: crate::Context<'_, Self::Action, Self::State, ()>,
        ) -> CoreResult<StateChange> {
            Ok(StateChange::Full)
        }

        fn effect(
            &mut self,
            _state: &mut Self::State,
            _ctx: crate::Context<'_, Self::SideEffect, Self::State, ()>,
        ) -> CoreResult<StateChange> {
            Ok(StateChange::None)
        }
    }

    init_test_runtime();

    let engine = ReducerEngine::<BadReducer>::new_persistent(
        BadReducer::default(),
        BadState,
        PersistenceConfig {
            key: "oxide_core_test_bad_state".to_string(),
            min_interval: std::time::Duration::from_millis(0),
        },
    )
    .await
    .unwrap();

    let mut error_stream = WatchStream::new(engine.subscribe_errors());
    let first = error_stream.next().await.expect("first error value");
    assert!(first.is_none());

    let _ = engine.dispatch(()).await.unwrap();
    let err = tokio::time::timeout(std::time::Duration::from_secs(1), error_stream.next())
        .await
        .expect("persistence error")
        .expect("error update")
        .expect("some error");
    assert!(matches!(err, OxideError::Persistence { .. }));
    assert!(err.to_string().contains("boom"));
}

#[tokio::test]
async fn sideeffect_loop_reports_persistence_encode_failures() {
    use crate::persistence::PersistenceConfig;
    use crate::serde::{Deserialize, Serialize};

    #[derive(Clone)]
    struct BadState;

    impl Serialize for BadState {
        fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
        where
            S: crate::serde::Serializer,
        {
            use crate::serde::ser::Error as _;
            Err(S::Error::custom("boom-sideeffect"))
        }
    }

    impl<'de> Deserialize<'de> for BadState {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: crate::serde::Deserializer<'de>,
        {
            let _ = crate::serde::de::IgnoredAny::deserialize(deserializer)?;
            Ok(BadState)
        }
    }

    #[derive(Default)]
    struct BadReducer;

    impl Reducer for BadReducer {
        type State = BadState;
        type Action = ();
        type SideEffect = ();

        async fn init(&mut self, _ctx: InitContext<Self::SideEffect>) {}

        fn reduce(
            &mut self,
            _state: &mut Self::State,
            _ctx: crate::Context<'_, Self::Action, Self::State, ()>,
        ) -> CoreResult<StateChange> {
            Ok(StateChange::None)
        }

        fn effect(
            &mut self,
            _state: &mut Self::State,
            _ctx: crate::Context<'_, Self::SideEffect, Self::State, ()>,
        ) -> CoreResult<StateChange> {
            Ok(StateChange::Full)
        }
    }

    init_test_runtime();

    let engine = ReducerEngine::<BadReducer>::new_persistent(
        BadReducer::default(),
        BadState,
        PersistenceConfig {
            key: "oxide_core_test_bad_state_sideeffect".to_string(),
            min_interval: std::time::Duration::from_millis(0),
        },
    )
    .await
    .unwrap();

    let mut error_stream = WatchStream::new(engine.subscribe_errors());
    let first = error_stream.next().await.expect("first error value");
    assert!(first.is_none());

    engine.sideeffect_sender().send(()).unwrap();
    let err = tokio::time::timeout(std::time::Duration::from_secs(1), error_stream.next())
        .await
        .expect("persistence side-effect error")
        .expect("error update")
        .expect("some error");
    assert!(matches!(err, OxideError::Persistence { .. }));
    assert!(err.to_string().contains("boom-sideeffect"));
}

#[tokio::test]
async fn sideeffect_loop_persistence_success_writes_latest_state() {
    use crate::persistence::{PersistenceConfig, decode, default_persistence_path};
    use crate::serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    struct PersistState {
        value: u32,
    }

    #[derive(Default)]
    struct PersistReducer;

    impl Reducer for PersistReducer {
        type State = PersistState;
        type Action = ();
        type SideEffect = ();

        async fn init(&mut self, _ctx: InitContext<Self::SideEffect>) {}

        fn reduce(
            &mut self,
            _state: &mut Self::State,
            _ctx: crate::Context<'_, Self::Action, Self::State, ()>,
        ) -> CoreResult<StateChange> {
            Ok(StateChange::None)
        }

        fn effect(
            &mut self,
            state: &mut Self::State,
            _ctx: crate::Context<'_, Self::SideEffect, Self::State, ()>,
        ) -> CoreResult<StateChange> {
            state.value = state.value.saturating_add(1);
            Ok(StateChange::Full)
        }
    }

    init_test_runtime();

    let key = format!(
        "oxide_core_test_sideeffect_ok_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );
    let path = default_persistence_path(&key);
    let _ = std::fs::remove_file(&path);

    let engine = ReducerEngine::<PersistReducer>::new_persistent(
        PersistReducer,
        PersistState { value: 0 },
        PersistenceConfig {
            key,
            min_interval: std::time::Duration::from_millis(0),
        },
    )
    .await
    .unwrap();

    let mut snapshots = WatchStream::new(engine.subscribe());
    let initial = snapshots.next().await.expect("initial snapshot");
    assert_eq!(initial.state.value, 0);

    engine.sideeffect_sender().send(()).unwrap();
    let next = tokio::time::timeout(std::time::Duration::from_secs(1), snapshots.next())
        .await
        .expect("side-effect snapshot")
        .expect("snapshot value");
    assert_eq!(next.state.value, 1);

    tokio::time::sleep(std::time::Duration::from_millis(75)).await;
    let bytes = std::fs::read(&path).expect("persistence file should exist");
    let restored: PersistState = decode(&bytes).expect("decode persisted state");
    assert_eq!(restored.value, 1);

    let _ = std::fs::remove_file(&path);
}
