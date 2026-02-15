// GENERATED CODE - DO NOT MODIFY BY HAND
// dart format width=80

part of 'oxide.dart';

// **************************************************************************
// OxideStoreGenerator
// **************************************************************************

class TickerControlInheritedOxideActions {
  TickerControlInheritedOxideActions._(
    Future<void> Function(AppAction action) dispatch,
  ) : _dispatch = dispatch;

  Future<void> Function(AppAction action) _dispatch;

  void _bind(Future<void> Function(AppAction action) dispatch) {
    _dispatch = dispatch;
  }

  Future<void> startTicker({required BigInt intervalMs}) async {
    await _dispatch(AppAction.startTicker(intervalMs: intervalMs));
  }

  Future<void> stopTicker() async {
    await _dispatch(AppAction.stopTicker());
  }

  Future<void> autoTick() async {
    await _dispatch(AppAction.autoTick());
  }

  Future<void> manualTick() async {
    await _dispatch(AppAction.manualTick());
  }

  Future<void> emitSideEffectTick() async {
    await _dispatch(AppAction.emitSideEffectTick());
  }

  Future<void> reset() async {
    await _dispatch(AppAction.reset());
  }
}

class TickerControlInheritedOxideController extends ChangeNotifier {
  TickerControlInheritedOxideController() {
    actions = TickerControlInheritedOxideActions._(_dispatch);
    actions._bind(_dispatch);
    _subscription = filterSnapshotsBySlices<AppStateSnapshot, AppStateSlice>(
      _core.snapshots,
      const [AppStateSlice.control],
      (snap) => snap.slices,
    ).listen((_) => _notify());
    unawaited(_initialize());
  }

  late final TickerControlInheritedOxideActions actions;
  StreamSubscription<AppStateSnapshot>? _subscription;

  late final OxideStoreCore<AppState, AppAction, ArcAppEngine, AppStateSnapshot>
  _core = OxideStoreCore<AppState, AppAction, ArcAppEngine, AppStateSnapshot>(
    createEngine: (_) => createSharedEngine(),
    disposeEngine: (engine) => disposeEngine(engine: engine),
    dispatch: (engine, action) => dispatch(engine: engine, action: action),
    current: (engine) => current(engine: engine),
    stateStream: (engine) => stateStream(engine: engine),
    stateFromSnapshot: (snap) => snap.state,
  );

  bool _isDisposed = false;

  bool get isLoading => _core.isLoading;
  Object? get error => _core.error;
  AppState? get state => _core.state;
  ArcAppEngine? get engine => _core.engine;

  OxideView<AppState, TickerControlInheritedOxideActions> get oxide =>
      OxideView(
        state: state,
        actions: actions,
        isLoading: isLoading,
        error: error,
      );

  @override
  void dispose() {
    _isDisposed = true;
    _subscription?.cancel();
    unawaited(_core.dispose());
    super.dispose();
  }

  void _notify() {
    if (_isDisposed) return;
    notifyListeners();
  }

  Future<void> _initialize() async {
    await _core.initialize();
    _notify();
  }

  Future<void> _dispatch(AppAction action) async {
    await _core.dispatchAction(action);
    _notify();
  }
}

class TickerControlInheritedOxideScope extends StatefulWidget {
  const TickerControlInheritedOxideScope({
    super.key,
    required this.child,
    this.controller,
  });

  final Widget child;
  final TickerControlInheritedOxideController? controller;

  static TickerControlInheritedOxideController controllerOf(
    BuildContext context,
  ) {
    final inherited = context
        .dependOnInheritedWidgetOfExactType<
          _TickerControlInheritedOxideInherited
        >();
    if (inherited == null || inherited.notifier == null) {
      throw StateError(
        'TickerControlInheritedOxideScope is missing from the widget tree.',
      );
    }
    return inherited.notifier!;
  }

  static OxideView<AppState, TickerControlInheritedOxideActions> useOxide(
    BuildContext context,
  ) {
    return controllerOf(context).oxide;
  }

  @override
  State<TickerControlInheritedOxideScope> createState() =>
      _TickerControlInheritedOxideScopeState();
}

class _TickerControlInheritedOxideScopeState
    extends State<TickerControlInheritedOxideScope>
    with AutomaticKeepAliveClientMixin {
  late final TickerControlInheritedOxideController _controller;
  late final bool _ownsController;

  @override
  bool get wantKeepAlive => false;

  @override
  void initState() {
    super.initState();
    _ownsController = widget.controller == null;
    _controller = widget.controller ?? TickerControlInheritedOxideController();
  }

  @override
  void dispose() {
    if (_ownsController) _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    super.build(context);
    return _TickerControlInheritedOxideInherited(
      notifier: _controller,
      child: widget.child,
    );
  }
}

class _TickerControlInheritedOxideInherited
    extends InheritedNotifier<TickerControlInheritedOxideController> {
  const _TickerControlInheritedOxideInherited({
    required super.notifier,
    required super.child,
  });
}

class TickerTickInheritedOxideActions {
  TickerTickInheritedOxideActions._(
    Future<void> Function(AppAction action) dispatch,
  ) : _dispatch = dispatch;

  Future<void> Function(AppAction action) _dispatch;

  void _bind(Future<void> Function(AppAction action) dispatch) {
    _dispatch = dispatch;
  }

  Future<void> startTicker({required BigInt intervalMs}) async {
    await _dispatch(AppAction.startTicker(intervalMs: intervalMs));
  }

  Future<void> stopTicker() async {
    await _dispatch(AppAction.stopTicker());
  }

  Future<void> autoTick() async {
    await _dispatch(AppAction.autoTick());
  }

  Future<void> manualTick() async {
    await _dispatch(AppAction.manualTick());
  }

  Future<void> emitSideEffectTick() async {
    await _dispatch(AppAction.emitSideEffectTick());
  }

  Future<void> reset() async {
    await _dispatch(AppAction.reset());
  }
}

class TickerTickInheritedOxideController extends ChangeNotifier {
  TickerTickInheritedOxideController() {
    actions = TickerTickInheritedOxideActions._(_dispatch);
    actions._bind(_dispatch);
    _subscription = filterSnapshotsBySlices<AppStateSnapshot, AppStateSlice>(
      _core.snapshots,
      const [AppStateSlice.tick],
      (snap) => snap.slices,
    ).listen((_) => _notify());
    unawaited(_initialize());
  }

  late final TickerTickInheritedOxideActions actions;
  StreamSubscription<AppStateSnapshot>? _subscription;

  late final OxideStoreCore<AppState, AppAction, ArcAppEngine, AppStateSnapshot>
  _core = OxideStoreCore<AppState, AppAction, ArcAppEngine, AppStateSnapshot>(
    createEngine: (_) => createSharedEngine(),
    disposeEngine: (engine) => disposeEngine(engine: engine),
    dispatch: (engine, action) => dispatch(engine: engine, action: action),
    current: (engine) => current(engine: engine),
    stateStream: (engine) => stateStream(engine: engine),
    stateFromSnapshot: (snap) => snap.state,
  );

  bool _isDisposed = false;

  bool get isLoading => _core.isLoading;
  Object? get error => _core.error;
  AppState? get state => _core.state;
  ArcAppEngine? get engine => _core.engine;

  OxideView<AppState, TickerTickInheritedOxideActions> get oxide => OxideView(
    state: state,
    actions: actions,
    isLoading: isLoading,
    error: error,
  );

  @override
  void dispose() {
    _isDisposed = true;
    _subscription?.cancel();
    unawaited(_core.dispose());
    super.dispose();
  }

  void _notify() {
    if (_isDisposed) return;
    notifyListeners();
  }

  Future<void> _initialize() async {
    await _core.initialize();
    _notify();
  }

  Future<void> _dispatch(AppAction action) async {
    await _core.dispatchAction(action);
    _notify();
  }
}

class TickerTickInheritedOxideScope extends StatefulWidget {
  const TickerTickInheritedOxideScope({
    super.key,
    required this.child,
    this.controller,
  });

  final Widget child;
  final TickerTickInheritedOxideController? controller;

  static TickerTickInheritedOxideController controllerOf(BuildContext context) {
    final inherited = context
        .dependOnInheritedWidgetOfExactType<
          _TickerTickInheritedOxideInherited
        >();
    if (inherited == null || inherited.notifier == null) {
      throw StateError(
        'TickerTickInheritedOxideScope is missing from the widget tree.',
      );
    }
    return inherited.notifier!;
  }

  static OxideView<AppState, TickerTickInheritedOxideActions> useOxide(
    BuildContext context,
  ) {
    return controllerOf(context).oxide;
  }

  @override
  State<TickerTickInheritedOxideScope> createState() =>
      _TickerTickInheritedOxideScopeState();
}

class _TickerTickInheritedOxideScopeState
    extends State<TickerTickInheritedOxideScope>
    with AutomaticKeepAliveClientMixin {
  late final TickerTickInheritedOxideController _controller;
  late final bool _ownsController;

  @override
  bool get wantKeepAlive => false;

  @override
  void initState() {
    super.initState();
    _ownsController = widget.controller == null;
    _controller = widget.controller ?? TickerTickInheritedOxideController();
  }

  @override
  void dispose() {
    if (_ownsController) _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    super.build(context);
    return _TickerTickInheritedOxideInherited(
      notifier: _controller,
      child: widget.child,
    );
  }
}

class _TickerTickInheritedOxideInherited
    extends InheritedNotifier<TickerTickInheritedOxideController> {
  const _TickerTickInheritedOxideInherited({
    required super.notifier,
    required super.child,
  });
}

class TickerControlHooksOxideActions {
  TickerControlHooksOxideActions._(
    Future<void> Function(AppAction action) dispatch,
  ) : _dispatch = dispatch;

  Future<void> Function(AppAction action) _dispatch;

  void _bind(Future<void> Function(AppAction action) dispatch) {
    _dispatch = dispatch;
  }

  Future<void> startTicker({required BigInt intervalMs}) async {
    await _dispatch(AppAction.startTicker(intervalMs: intervalMs));
  }

  Future<void> stopTicker() async {
    await _dispatch(AppAction.stopTicker());
  }

  Future<void> autoTick() async {
    await _dispatch(AppAction.autoTick());
  }

  Future<void> manualTick() async {
    await _dispatch(AppAction.manualTick());
  }

  Future<void> emitSideEffectTick() async {
    await _dispatch(AppAction.emitSideEffectTick());
  }

  Future<void> reset() async {
    await _dispatch(AppAction.reset());
  }
}

class TickerControlHooksOxideController extends ChangeNotifier {
  TickerControlHooksOxideController() {
    actions = TickerControlHooksOxideActions._(_dispatch);
    actions._bind(_dispatch);
    _subscription = filterSnapshotsBySlices<AppStateSnapshot, AppStateSlice>(
      _core.snapshots,
      const [AppStateSlice.control],
      (snap) => snap.slices,
    ).listen((_) => _notify());
    unawaited(_initialize());
  }

  late final TickerControlHooksOxideActions actions;
  StreamSubscription<AppStateSnapshot>? _subscription;

  late final OxideStoreCore<AppState, AppAction, ArcAppEngine, AppStateSnapshot>
  _core = OxideStoreCore<AppState, AppAction, ArcAppEngine, AppStateSnapshot>(
    createEngine: (_) => createSharedEngine(),
    disposeEngine: (engine) => disposeEngine(engine: engine),
    dispatch: (engine, action) => dispatch(engine: engine, action: action),
    current: (engine) => current(engine: engine),
    stateStream: (engine) => stateStream(engine: engine),
    stateFromSnapshot: (snap) => snap.state,
  );

  bool _isDisposed = false;

  bool get isLoading => _core.isLoading;
  Object? get error => _core.error;
  AppState? get state => _core.state;
  ArcAppEngine? get engine => _core.engine;

  OxideView<AppState, TickerControlHooksOxideActions> get oxide => OxideView(
    state: state,
    actions: actions,
    isLoading: isLoading,
    error: error,
  );

  @override
  void dispose() {
    _isDisposed = true;
    _subscription?.cancel();
    unawaited(_core.dispose());
    super.dispose();
  }

  void _notify() {
    if (_isDisposed) return;
    notifyListeners();
  }

  Future<void> _initialize() async {
    await _core.initialize();
    _notify();
  }

  Future<void> _dispatch(AppAction action) async {
    await _core.dispatchAction(action);
    _notify();
  }
}

class TickerControlHooksOxideScope extends StatefulWidget {
  const TickerControlHooksOxideScope({
    super.key,
    required this.child,
    this.controller,
  });

  final Widget child;
  final TickerControlHooksOxideController? controller;

  static TickerControlHooksOxideController controllerOf(BuildContext context) {
    final inherited = context
        .dependOnInheritedWidgetOfExactType<
          _TickerControlHooksOxideInherited
        >();
    if (inherited == null || inherited.notifier == null) {
      throw StateError(
        'TickerControlHooksOxideScope is missing from the widget tree.',
      );
    }
    return inherited.notifier!;
  }

  static OxideView<AppState, TickerControlHooksOxideActions> useOxide(
    BuildContext context,
  ) {
    return controllerOf(context).oxide;
  }

  @override
  State<TickerControlHooksOxideScope> createState() =>
      _TickerControlHooksOxideScopeState();
}

class _TickerControlHooksOxideScopeState
    extends State<TickerControlHooksOxideScope>
    with AutomaticKeepAliveClientMixin {
  late final TickerControlHooksOxideController _controller;
  late final bool _ownsController;

  @override
  bool get wantKeepAlive => false;

  @override
  void initState() {
    super.initState();
    _ownsController = widget.controller == null;
    _controller = widget.controller ?? TickerControlHooksOxideController();
  }

  @override
  void dispose() {
    if (_ownsController) _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    super.build(context);
    return _TickerControlHooksOxideInherited(
      notifier: _controller,
      child: widget.child,
    );
  }
}

class _TickerControlHooksOxideInherited
    extends InheritedNotifier<TickerControlHooksOxideController> {
  const _TickerControlHooksOxideInherited({
    required super.notifier,
    required super.child,
  });
}

OxideView<AppState, TickerControlHooksOxideActions>
useTickerControlHooksOxideOxide() {
  final context = useContext();
  return TickerControlHooksOxideScope.useOxide(context);
}

class TickerTickHooksOxideActions {
  TickerTickHooksOxideActions._(
    Future<void> Function(AppAction action) dispatch,
  ) : _dispatch = dispatch;

  Future<void> Function(AppAction action) _dispatch;

  void _bind(Future<void> Function(AppAction action) dispatch) {
    _dispatch = dispatch;
  }

  Future<void> startTicker({required BigInt intervalMs}) async {
    await _dispatch(AppAction.startTicker(intervalMs: intervalMs));
  }

  Future<void> stopTicker() async {
    await _dispatch(AppAction.stopTicker());
  }

  Future<void> autoTick() async {
    await _dispatch(AppAction.autoTick());
  }

  Future<void> manualTick() async {
    await _dispatch(AppAction.manualTick());
  }

  Future<void> emitSideEffectTick() async {
    await _dispatch(AppAction.emitSideEffectTick());
  }

  Future<void> reset() async {
    await _dispatch(AppAction.reset());
  }
}

class TickerTickHooksOxideController extends ChangeNotifier {
  TickerTickHooksOxideController() {
    actions = TickerTickHooksOxideActions._(_dispatch);
    actions._bind(_dispatch);
    _subscription = filterSnapshotsBySlices<AppStateSnapshot, AppStateSlice>(
      _core.snapshots,
      const [AppStateSlice.tick],
      (snap) => snap.slices,
    ).listen((_) => _notify());
    unawaited(_initialize());
  }

  late final TickerTickHooksOxideActions actions;
  StreamSubscription<AppStateSnapshot>? _subscription;

  late final OxideStoreCore<AppState, AppAction, ArcAppEngine, AppStateSnapshot>
  _core = OxideStoreCore<AppState, AppAction, ArcAppEngine, AppStateSnapshot>(
    createEngine: (_) => createSharedEngine(),
    disposeEngine: (engine) => disposeEngine(engine: engine),
    dispatch: (engine, action) => dispatch(engine: engine, action: action),
    current: (engine) => current(engine: engine),
    stateStream: (engine) => stateStream(engine: engine),
    stateFromSnapshot: (snap) => snap.state,
  );

  bool _isDisposed = false;

  bool get isLoading => _core.isLoading;
  Object? get error => _core.error;
  AppState? get state => _core.state;
  ArcAppEngine? get engine => _core.engine;

  OxideView<AppState, TickerTickHooksOxideActions> get oxide => OxideView(
    state: state,
    actions: actions,
    isLoading: isLoading,
    error: error,
  );

  @override
  void dispose() {
    _isDisposed = true;
    _subscription?.cancel();
    unawaited(_core.dispose());
    super.dispose();
  }

  void _notify() {
    if (_isDisposed) return;
    notifyListeners();
  }

  Future<void> _initialize() async {
    await _core.initialize();
    _notify();
  }

  Future<void> _dispatch(AppAction action) async {
    await _core.dispatchAction(action);
    _notify();
  }
}

class TickerTickHooksOxideScope extends StatefulWidget {
  const TickerTickHooksOxideScope({
    super.key,
    required this.child,
    this.controller,
  });

  final Widget child;
  final TickerTickHooksOxideController? controller;

  static TickerTickHooksOxideController controllerOf(BuildContext context) {
    final inherited = context
        .dependOnInheritedWidgetOfExactType<_TickerTickHooksOxideInherited>();
    if (inherited == null || inherited.notifier == null) {
      throw StateError(
        'TickerTickHooksOxideScope is missing from the widget tree.',
      );
    }
    return inherited.notifier!;
  }

  static OxideView<AppState, TickerTickHooksOxideActions> useOxide(
    BuildContext context,
  ) {
    return controllerOf(context).oxide;
  }

  @override
  State<TickerTickHooksOxideScope> createState() =>
      _TickerTickHooksOxideScopeState();
}

class _TickerTickHooksOxideScopeState extends State<TickerTickHooksOxideScope>
    with AutomaticKeepAliveClientMixin {
  late final TickerTickHooksOxideController _controller;
  late final bool _ownsController;

  @override
  bool get wantKeepAlive => false;

  @override
  void initState() {
    super.initState();
    _ownsController = widget.controller == null;
    _controller = widget.controller ?? TickerTickHooksOxideController();
  }

  @override
  void dispose() {
    if (_ownsController) _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    super.build(context);
    return _TickerTickHooksOxideInherited(
      notifier: _controller,
      child: widget.child,
    );
  }
}

class _TickerTickHooksOxideInherited
    extends InheritedNotifier<TickerTickHooksOxideController> {
  const _TickerTickHooksOxideInherited({
    required super.notifier,
    required super.child,
  });
}

OxideView<AppState, TickerTickHooksOxideActions>
useTickerTickHooksOxideOxide() {
  final context = useContext();
  return TickerTickHooksOxideScope.useOxide(context);
}

class TickerControlRiverpodOxideActions {
  TickerControlRiverpodOxideActions._(
    Future<void> Function(AppAction action) dispatch,
  ) : _dispatch = dispatch;

  Future<void> Function(AppAction action) _dispatch;

  void _bind(Future<void> Function(AppAction action) dispatch) {
    _dispatch = dispatch;
  }

  Future<void> startTicker({required BigInt intervalMs}) async {
    await _dispatch(AppAction.startTicker(intervalMs: intervalMs));
  }

  Future<void> stopTicker() async {
    await _dispatch(AppAction.stopTicker());
  }

  Future<void> autoTick() async {
    await _dispatch(AppAction.autoTick());
  }

  Future<void> manualTick() async {
    await _dispatch(AppAction.manualTick());
  }

  Future<void> emitSideEffectTick() async {
    await _dispatch(AppAction.emitSideEffectTick());
  }

  Future<void> reset() async {
    await _dispatch(AppAction.reset());
  }
}

final tickerControlRiverpodOxideProvider =
    NotifierProvider.autoDispose<
      TickerControlRiverpodOxideNotifier,
      OxideView<AppState, TickerControlRiverpodOxideActions>
    >(() => TickerControlRiverpodOxideNotifier());

class TickerControlRiverpodOxideNotifier
    extends Notifier<OxideView<AppState, TickerControlRiverpodOxideActions>> {
  TickerControlRiverpodOxideNotifier();

  late final TickerControlRiverpodOxideActions actions =
      TickerControlRiverpodOxideActions._(_dispatch);
  StreamSubscription<AppStateSnapshot>? _subscription;

  late final OxideStoreCore<AppState, AppAction, ArcAppEngine, AppStateSnapshot>
  _core = OxideStoreCore<AppState, AppAction, ArcAppEngine, AppStateSnapshot>(
    createEngine: (_) => createSharedEngine(),
    disposeEngine: (engine) => disposeEngine(engine: engine),
    dispatch: (engine, action) => dispatch(engine: engine, action: action),
    current: (engine) => current(engine: engine),
    stateStream: (engine) => stateStream(engine: engine),
    stateFromSnapshot: (snap) => snap.state,
  );

  @override
  OxideView<AppState, TickerControlRiverpodOxideActions> build() {
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
    _subscription = filterSnapshotsBySlices<AppStateSnapshot, AppStateSlice>(
      _core.snapshots,
      const [AppStateSlice.control],
      (snap) => snap.slices,
    ).listen((_) => state = _view());
    state = _view();
  }

  OxideView<AppState, TickerControlRiverpodOxideActions> _view() {
    return OxideView(
      state: _core.state,
      actions: actions,
      isLoading: _core.isLoading,
      error: _core.error,
    );
  }

  Future<void> _dispatch(AppAction action) async {
    await _core.dispatchAction(action);
    state = _view();
  }
}

class TickerTickRiverpodOxideActions {
  TickerTickRiverpodOxideActions._(
    Future<void> Function(AppAction action) dispatch,
  ) : _dispatch = dispatch;

  Future<void> Function(AppAction action) _dispatch;

  void _bind(Future<void> Function(AppAction action) dispatch) {
    _dispatch = dispatch;
  }

  Future<void> startTicker({required BigInt intervalMs}) async {
    await _dispatch(AppAction.startTicker(intervalMs: intervalMs));
  }

  Future<void> stopTicker() async {
    await _dispatch(AppAction.stopTicker());
  }

  Future<void> autoTick() async {
    await _dispatch(AppAction.autoTick());
  }

  Future<void> manualTick() async {
    await _dispatch(AppAction.manualTick());
  }

  Future<void> emitSideEffectTick() async {
    await _dispatch(AppAction.emitSideEffectTick());
  }

  Future<void> reset() async {
    await _dispatch(AppAction.reset());
  }
}

final tickerTickRiverpodOxideProvider =
    NotifierProvider.autoDispose<
      TickerTickRiverpodOxideNotifier,
      OxideView<AppState, TickerTickRiverpodOxideActions>
    >(() => TickerTickRiverpodOxideNotifier());

class TickerTickRiverpodOxideNotifier
    extends Notifier<OxideView<AppState, TickerTickRiverpodOxideActions>> {
  TickerTickRiverpodOxideNotifier();

  late final TickerTickRiverpodOxideActions actions =
      TickerTickRiverpodOxideActions._(_dispatch);
  StreamSubscription<AppStateSnapshot>? _subscription;

  late final OxideStoreCore<AppState, AppAction, ArcAppEngine, AppStateSnapshot>
  _core = OxideStoreCore<AppState, AppAction, ArcAppEngine, AppStateSnapshot>(
    createEngine: (_) => createSharedEngine(),
    disposeEngine: (engine) => disposeEngine(engine: engine),
    dispatch: (engine, action) => dispatch(engine: engine, action: action),
    current: (engine) => current(engine: engine),
    stateStream: (engine) => stateStream(engine: engine),
    stateFromSnapshot: (snap) => snap.state,
  );

  @override
  OxideView<AppState, TickerTickRiverpodOxideActions> build() {
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
    _subscription = filterSnapshotsBySlices<AppStateSnapshot, AppStateSlice>(
      _core.snapshots,
      const [AppStateSlice.tick],
      (snap) => snap.slices,
    ).listen((_) => state = _view());
    state = _view();
  }

  OxideView<AppState, TickerTickRiverpodOxideActions> _view() {
    return OxideView(
      state: _core.state,
      actions: actions,
      isLoading: _core.isLoading,
      error: _core.error,
    );
  }

  Future<void> _dispatch(AppAction action) async {
    await _core.dispatchAction(action);
    state = _view();
  }
}

class TickerControlBlocOxideActions {
  TickerControlBlocOxideActions._(
    Future<void> Function(AppAction action) dispatch,
  ) : _dispatch = dispatch;

  Future<void> Function(AppAction action) _dispatch;

  void _bind(Future<void> Function(AppAction action) dispatch) {
    _dispatch = dispatch;
  }

  Future<void> startTicker({required BigInt intervalMs}) async {
    await _dispatch(AppAction.startTicker(intervalMs: intervalMs));
  }

  Future<void> stopTicker() async {
    await _dispatch(AppAction.stopTicker());
  }

  Future<void> autoTick() async {
    await _dispatch(AppAction.autoTick());
  }

  Future<void> manualTick() async {
    await _dispatch(AppAction.manualTick());
  }

  Future<void> emitSideEffectTick() async {
    await _dispatch(AppAction.emitSideEffectTick());
  }

  Future<void> reset() async {
    await _dispatch(AppAction.reset());
  }
}

class TickerControlBlocOxideCubit
    extends Cubit<OxideView<AppState, TickerControlBlocOxideActions>> {
  TickerControlBlocOxideCubit()
    : actions = TickerControlBlocOxideActions._(_noopDispatch),
      super(
        OxideView(
          state: null,
          actions: TickerControlBlocOxideActions._(_noopDispatch),
          isLoading: true,
          error: null,
        ),
      ) {
    actions._bind(_dispatch);
    _subscription = filterSnapshotsBySlices<AppStateSnapshot, AppStateSlice>(
      _core.snapshots,
      const [AppStateSlice.control],
      (snap) => snap.slices,
    ).listen((_) => _emit());
    unawaited(_initialize());
  }

  final TickerControlBlocOxideActions actions;
  StreamSubscription<AppStateSnapshot>? _subscription;

  late final OxideStoreCore<AppState, AppAction, ArcAppEngine, AppStateSnapshot>
  _core = OxideStoreCore<AppState, AppAction, ArcAppEngine, AppStateSnapshot>(
    createEngine: (_) => createSharedEngine(),
    disposeEngine: (engine) => disposeEngine(engine: engine),
    dispatch: (engine, action) => dispatch(engine: engine, action: action),
    current: (engine) => current(engine: engine),
    stateStream: (engine) => stateStream(engine: engine),
    stateFromSnapshot: (snap) => snap.state,
  );

  static Future<void> _noopDispatch(AppAction _) async {}

  Future<void> _initialize() async {
    await _core.initialize();
    _emit();
  }

  void _emit() {
    emit(
      OxideView(
        state: _core.state,
        actions: actions,
        isLoading: _core.isLoading,
        error: _core.error,
      ),
    );
  }

  Future<void> _dispatch(AppAction action) async {
    await _core.dispatchAction(action);
    _emit();
  }

  @override
  Future<void> close() async {
    await _subscription?.cancel();
    await _core.dispose();
    return super.close();
  }
}

class TickerControlBlocOxideScope extends StatefulWidget {
  const TickerControlBlocOxideScope({
    super.key,
    required this.child,
    this.cubit,
  });

  final Widget child;
  final TickerControlBlocOxideCubit? cubit;

  static TickerControlBlocOxideCubit cubitOf(BuildContext context) {
    return BlocProvider.of<TickerControlBlocOxideCubit>(context);
  }

  @override
  State<TickerControlBlocOxideScope> createState() =>
      _TickerControlBlocOxideScopeState();
}

class _TickerControlBlocOxideScopeState
    extends State<TickerControlBlocOxideScope>
    with AutomaticKeepAliveClientMixin {
  late final TickerControlBlocOxideCubit _cubit;
  late final bool _ownsCubit;

  @override
  bool get wantKeepAlive => false;

  @override
  void initState() {
    super.initState();
    _ownsCubit = widget.cubit == null;
    _cubit = widget.cubit ?? TickerControlBlocOxideCubit();
  }

  @override
  void dispose() {
    if (_ownsCubit) unawaited(_cubit.close());
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    super.build(context);
    return BlocProvider.value(value: _cubit, child: widget.child);
  }
}

class TickerTickBlocOxideActions {
  TickerTickBlocOxideActions._(Future<void> Function(AppAction action) dispatch)
    : _dispatch = dispatch;

  Future<void> Function(AppAction action) _dispatch;

  void _bind(Future<void> Function(AppAction action) dispatch) {
    _dispatch = dispatch;
  }

  Future<void> startTicker({required BigInt intervalMs}) async {
    await _dispatch(AppAction.startTicker(intervalMs: intervalMs));
  }

  Future<void> stopTicker() async {
    await _dispatch(AppAction.stopTicker());
  }

  Future<void> autoTick() async {
    await _dispatch(AppAction.autoTick());
  }

  Future<void> manualTick() async {
    await _dispatch(AppAction.manualTick());
  }

  Future<void> emitSideEffectTick() async {
    await _dispatch(AppAction.emitSideEffectTick());
  }

  Future<void> reset() async {
    await _dispatch(AppAction.reset());
  }
}

class TickerTickBlocOxideCubit
    extends Cubit<OxideView<AppState, TickerTickBlocOxideActions>> {
  TickerTickBlocOxideCubit()
    : actions = TickerTickBlocOxideActions._(_noopDispatch),
      super(
        OxideView(
          state: null,
          actions: TickerTickBlocOxideActions._(_noopDispatch),
          isLoading: true,
          error: null,
        ),
      ) {
    actions._bind(_dispatch);
    _subscription = filterSnapshotsBySlices<AppStateSnapshot, AppStateSlice>(
      _core.snapshots,
      const [AppStateSlice.tick],
      (snap) => snap.slices,
    ).listen((_) => _emit());
    unawaited(_initialize());
  }

  final TickerTickBlocOxideActions actions;
  StreamSubscription<AppStateSnapshot>? _subscription;

  late final OxideStoreCore<AppState, AppAction, ArcAppEngine, AppStateSnapshot>
  _core = OxideStoreCore<AppState, AppAction, ArcAppEngine, AppStateSnapshot>(
    createEngine: (_) => createSharedEngine(),
    disposeEngine: (engine) => disposeEngine(engine: engine),
    dispatch: (engine, action) => dispatch(engine: engine, action: action),
    current: (engine) => current(engine: engine),
    stateStream: (engine) => stateStream(engine: engine),
    stateFromSnapshot: (snap) => snap.state,
  );

  static Future<void> _noopDispatch(AppAction _) async {}

  Future<void> _initialize() async {
    await _core.initialize();
    _emit();
  }

  void _emit() {
    emit(
      OxideView(
        state: _core.state,
        actions: actions,
        isLoading: _core.isLoading,
        error: _core.error,
      ),
    );
  }

  Future<void> _dispatch(AppAction action) async {
    await _core.dispatchAction(action);
    _emit();
  }

  @override
  Future<void> close() async {
    await _subscription?.cancel();
    await _core.dispose();
    return super.close();
  }
}

class TickerTickBlocOxideScope extends StatefulWidget {
  const TickerTickBlocOxideScope({super.key, required this.child, this.cubit});

  final Widget child;
  final TickerTickBlocOxideCubit? cubit;

  static TickerTickBlocOxideCubit cubitOf(BuildContext context) {
    return BlocProvider.of<TickerTickBlocOxideCubit>(context);
  }

  @override
  State<TickerTickBlocOxideScope> createState() =>
      _TickerTickBlocOxideScopeState();
}

class _TickerTickBlocOxideScopeState extends State<TickerTickBlocOxideScope>
    with AutomaticKeepAliveClientMixin {
  late final TickerTickBlocOxideCubit _cubit;
  late final bool _ownsCubit;

  @override
  bool get wantKeepAlive => false;

  @override
  void initState() {
    super.initState();
    _ownsCubit = widget.cubit == null;
    _cubit = widget.cubit ?? TickerTickBlocOxideCubit();
  }

  @override
  void dispose() {
    if (_ownsCubit) unawaited(_cubit.close());
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    super.build(context);
    return BlocProvider.value(value: _cubit, child: widget.child);
  }
}
