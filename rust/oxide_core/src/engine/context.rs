use crate::engine::StateSnapshot;

/// Context object passed to reducers/effects.
///
/// Why: reducers/effects frequently need access to the current route and a stable view of
/// state while processing an input (action or side-effect).
///
/// How: the engine constructs this context for each invocation and passes it into reducer
/// calls as an immutable view of the invocation environment.
pub struct Context<'a, Input, State, StateSlice>
where
    StateSlice: Copy + PartialEq + Eq + Send + Sync + 'static,
{
    /// Action or side-effect currently being processed.
    pub input: &'a Input,
    /// Snapshot of state at the time the input is processed.
    pub state_snapshot: &'a StateSnapshot<State, StateSlice>,
    /// Navigation capability surface + current route snapshot.
    ///
    /// This field is only available when the `navigation-binding` feature is enabled.
    #[cfg(feature = "navigation-binding")]
    pub nav: crate::engine::NavigationCtx<'a>,
}
