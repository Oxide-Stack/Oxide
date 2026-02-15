// Generated store UI contract.
//
// Why: generated backends need a small, serializable view object that widgets
// can consume without depending on the underlying engine types.
/// Lightweight view model used by some Oxide backends.
///
/// This bundles the current derived state, a set of actions, and basic
/// loading/error flags in a single immutable object.
final class OxideView<S, A> {
  /// Creates a view value.
  const OxideView({required this.state, required this.actions, required this.isLoading, required this.error});

  /// The current state value, or `null` when not initialized.
  final S? state;
  /// Actions object used to dispatch store actions.
  final A actions;
  /// Whether the store is currently loading.
  final bool isLoading;
  /// The most recent error, if any.
  final Object? error;
}
