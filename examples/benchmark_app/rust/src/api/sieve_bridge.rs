use flutter_rust_bridge::frb;
use oxide_generator_rs::reducer;

use crate::util::fnv1a_mix_u64;
use crate::state::sieve_action::SieveAction;
use crate::state::sieve_state::SieveState;
pub use crate::OxideError;

pub const SIEVE_LIMIT: usize = 50_000;

#[reducer(
    engine = SieveEngine,
    snapshot = SieveStateSnapshot,
    initial = SieveState::new(),
)]
impl oxide_core::Reducer for SieveRootReducer {
    type State = SieveState;
    type Action = SieveAction;
    type SideEffect = SieveSideEffect;

    async fn init(&mut self, _ctx: oxide_core::InitContext<Self::SideEffect>) {}

    fn reduce(
        &mut self,
        state: &mut Self::State,
        ctx: oxide_core::Context<'_, Self::Action, Self::State, ()>,
    ) -> oxide_core::CoreResult<oxide_core::StateChange> {
        let SieveAction::Run { iterations } = ctx.input;
        if *iterations == 0 {
            return Ok(oxide_core::StateChange::None);
        }

        for _ in 0..*iterations {
            let primes = run_sieve(SIEVE_LIMIT);
            state.counter = state.counter.saturating_add(1);
            state.checksum = fnv1a_mix_u64(state.checksum, primes);
            state.checksum = fnv1a_mix_u64(state.checksum, SIEVE_LIMIT as u64);
        }

        Ok(oxide_core::StateChange::Full)
    }

    fn effect(
        &mut self,
        _state: &mut Self::State,
        _ctx: oxide_core::Context<'_, Self::SideEffect, Self::State, ()>,
    ) -> oxide_core::CoreResult<oxide_core::StateChange> {
        Ok(oxide_core::StateChange::None)
    }
}
#[frb(ignore)]

 enum SieveSideEffect {}

#[frb(ignore)]
#[derive(Default)]
 struct SieveRootReducer {}

fn run_sieve(limit: usize) -> u64 {
    let mut is_prime = vec![true; limit.saturating_add(1)];
    if !is_prime.is_empty() {
        is_prime[0] = false;
    }
    if is_prime.len() > 1 {
        is_prime[1] = false;
    }

    let sqrt = (limit as f64).sqrt() as usize;
    for p in 2..=sqrt {
        if !is_prime[p] {
            continue;
        }
        let mut multiple = p.saturating_mul(p);
        while multiple <= limit {
            is_prime[multiple] = false;
            multiple = multiple.saturating_add(p);
        }
    }

    is_prime.iter().filter(|&&v| v).count() as u64
}
