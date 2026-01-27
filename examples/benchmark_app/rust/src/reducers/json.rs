const LIGHT_JSON: &str = include_str!("../../assets/light.json");
const HEAVY_JSON: &str = include_str!("../../assets/heavy.json");
use crate::reducers::util::{canonicalize_json, count_entries, fnv1a64, fnv1a_mix_u64};
use crate::state::json_action::JsonAction;
use crate::state::json_state::JsonState;

pub struct JsonReducer;

impl JsonReducer {
    pub fn reduce(
        state: &mut JsonState,
        action: JsonAction,
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

        Ok(oxide_core::StateChange::FullUpdate)
    }
}

fn run_json_once(state: &mut JsonState, json: &str) -> oxide_core::CoreResult<()> {
    let mut value: serde_json::Value =
        serde_json::from_str(json).map_err(|e| oxide_core::CoreError::Validation { message: e.to_string() })?;

    canonicalize_json(&mut value);
    let entries = count_entries(&value);
    let serialized =
        serde_json::to_string(&value).map_err(|e| oxide_core::CoreError::Validation { message: e.to_string() })?;
    let json_len = serialized.len() as u64;
    let hash = fnv1a64(serialized.as_bytes());

    state.counter = state.counter.saturating_add(1);
    state.checksum = fnv1a_mix_u64(state.checksum, hash);
    state.checksum = fnv1a_mix_u64(state.checksum, entries);
    state.checksum = fnv1a_mix_u64(state.checksum, json_len);
    Ok(())
}
