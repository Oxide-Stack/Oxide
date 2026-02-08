use flutter_rust_bridge::frb;
use oxide_generator_rs::reducer;

use crate::util::{canonicalize_json, count_entries, fnv1a64, fnv1a_mix_u64};
use crate::state::json_action::JsonAction;
use crate::state::json_state::JsonState;
pub use crate::OxideError;

const LIGHT_JSON: &str = include_str!("../../assets/light.json");
const HEAVY_JSON: &str = include_str!("../../assets/heavy.json");

#[reducer(
    engine = JsonEngine,
    snapshot = JsonStateSnapshot,
    initial = JsonState::new(),
)]
impl oxide_core::Reducer for JsonRootReducer {
    type State = JsonState;
    type Action = JsonAction;
    type SideEffect = JsonSideEffect;

    async fn init(&mut self, _ctx: oxide_core::InitContext<Self::SideEffect>) {}

    fn reduce(
        &mut self,
        state: &mut Self::State,
        action: Self::Action,
    ) -> oxide_core::CoreResult<oxide_core::StateChange> {
        let (json, iterations) = match action {
            JsonAction::RunLight { iterations } => (LIGHT_JSON, iterations),
            JsonAction::RunHeavy { iterations } => (HEAVY_JSON, iterations),
        };

        if iterations == 0 {
            return Ok(oxide_core::StateChange::None);
        }

        for _ in 0..iterations {
            run_json_once(state, json)?;
        }

        Ok(oxide_core::StateChange::Full)
    }

    fn effect(
        &mut self,
        _state: &mut Self::State,
        _effect: Self::SideEffect,
    ) -> oxide_core::CoreResult<oxide_core::StateChange> {
        Ok(oxide_core::StateChange::None)
    }
}

#[frb(ignore)]

 enum JsonSideEffect {}

#[frb(ignore)]

 #[derive(Default)]
 struct JsonRootReducer {}

fn run_json_once(state: &mut JsonState, json: &str) -> oxide_core::CoreResult<()> {
    let mut value: serde_json::Value = serde_json::from_str(json)
        .map_err(|e| oxide_core::OxideError::Validation { message: e.to_string() })?;

    canonicalize_json(&mut value);
    let entries = count_entries(&value);
    let serialized = serde_json::to_string(&value)
        .map_err(|e| oxide_core::OxideError::Validation { message: e.to_string() })?;
    let json_len = serialized.len() as u64;
    let hash = fnv1a64(serialized.as_bytes());

    state.counter = state.counter.saturating_add(1);
    state.checksum = fnv1a_mix_u64(state.checksum, hash);
    state.checksum = fnv1a_mix_u64(state.checksum, entries);
    state.checksum = fnv1a_mix_u64(state.checksum, json_len);
    Ok(())
}
