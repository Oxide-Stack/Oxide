use oxide_macros::actions;

/// Actions that can be dispatched in the ticker example.
#[actions]
pub enum AppAction {
    /// Start generating periodic ticks from the Flutter side.
    StartTicker { interval_ms: u64 },
    /// Stop generating periodic ticks from the Flutter side.
    StopTicker,
    /// Tick produced by a Flutter-driven periodic timer.
    AutoTick,
    /// Tick explicitly requested by the user.
    ManualTick,
    /// Emits a Rust side-effect that will apply a tick through `effect()`.
    EmitSideEffectTick,
    /// Reset the tick counter back to zero.
    Reset,
}
