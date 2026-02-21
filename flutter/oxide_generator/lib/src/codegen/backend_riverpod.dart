import 'helpers.dart';
import 'model.dart';

// Riverpod Notifier backend templates.
//
// Why: Riverpod is a common app-layer integration; this template wires the core
// store into a NotifierProvider while keeping the engine lifecycle consistent.
String buildRiverpodBackend(OxideCodegenConfig c, String coreInstantiation) {
  final snapshotsStream = (c.slices == null || c.slices!.isEmpty)
      ? '_core.snapshots'
      : 'filterSnapshotsBySlices<${c.snapshotType}, ${c.sliceType!}>('
          '_core.snapshots, '
          'const [${c.slices!.join(', ')}], '
          '(snap) => snap.slices'
          ')';
  return '''
final ${lowerFirst(c.prefix)}Provider = ${c.keepAlive ? 'NotifierProvider' : 'NotifierProvider.autoDispose'}<
    ${c.prefix}Notifier,
    OxideView<${c.stateType}, ${c.prefix}Actions>>(
  () => ${c.prefix}Notifier(),
);

class ${c.prefix}Notifier
    extends Notifier<OxideView<${c.stateType}, ${c.prefix}Actions>> {
  ${c.prefix}Notifier();

  late final ${c.prefix}Actions actions = ${c.prefix}Actions._(_dispatch);
  StreamSubscription<${c.snapshotType}>? _subscription;

$coreInstantiation

  @override
  OxideView<${c.stateType}, ${c.prefix}Actions> build() {
    actions._bind(_dispatch);
    ref.onDispose(() {
      _subscription?.cancel();
      unawaited(_core.dispose());
    });

    unawaited(_initialize());
    return OxideView(
      state: null,
      actions: actions,
      isLoading: true,
      error: null,
    );
  }

  Future<void> _initialize() async {
    await _core.initialize();
    if (!ref.mounted) return;
    _subscription = $snapshotsStream.listen((_) {
      if (!ref.mounted) return;
      state = _view();
    });
    if (!ref.mounted) return;
    state = _view();
  }

  OxideView<${c.stateType}, ${c.prefix}Actions> _view() {
    return OxideView(
      state: _core.state,
      actions: actions,
      isLoading: _core.isLoading,
      error: _core.error,
    );
  }

  Future<void> _dispatch(${c.actionsType} action) async {
    await _core.dispatchAction(action);
    if (!ref.mounted) return;
    state = _view();
  }
}
''';
}
