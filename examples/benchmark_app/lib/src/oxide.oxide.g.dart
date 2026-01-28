// GENERATED CODE - DO NOT MODIFY BY HAND
// dart format width=80

part of 'oxide.dart';

// **************************************************************************
// OxideStoreGenerator
// **************************************************************************

class BenchCounterRiverpodOxideActions {
  BenchCounterRiverpodOxideActions._(
    Future<void> Function(CounterAction action) dispatch,
  ) : _dispatch = dispatch;

  Future<void> Function(CounterAction action) _dispatch;

  void _bind(Future<void> Function(CounterAction action) dispatch) {
    _dispatch = dispatch;
  }

  Future<void> run({required int iterations}) async {
    await _dispatch(CounterAction.run(iterations: iterations));
  }
}

final benchCounterRiverpodOxideProvider =
    NotifierProvider.autoDispose<
      BenchCounterRiverpodOxideNotifier,
      OxideView<CounterState, BenchCounterRiverpodOxideActions>
    >(() => BenchCounterRiverpodOxideNotifier());

class BenchCounterRiverpodOxideNotifier
    extends
        Notifier<OxideView<CounterState, BenchCounterRiverpodOxideActions>> {
  BenchCounterRiverpodOxideNotifier();

  late final BenchCounterRiverpodOxideActions actions =
      BenchCounterRiverpodOxideActions._(_dispatch);
  StreamSubscription<CounterStateSnapshot>? _subscription;

  late final OxideStoreCore<
    CounterState,
    CounterAction,
    ArcCounterEngine,
    CounterStateSnapshot
  >
  _core =
      OxideStoreCore<
        CounterState,
        CounterAction,
        ArcCounterEngine,
        CounterStateSnapshot
      >(
        createEngine: (_) => counter_api.createEngine(),
        disposeEngine: (engine) => counter_api.disposeEngine(engine: engine),
        dispatch: (engine, action) =>
            counter_api.dispatch(engine: engine, action: action),
        current: (engine) => counter_api.current(engine: engine),
        stateStream: (engine) => counter_api.stateStream(engine: engine),
        stateFromSnapshot: (snap) => snap.state,
      );

  @override
  OxideView<CounterState, BenchCounterRiverpodOxideActions> build() {
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
    _subscription = _core.snapshots.listen((_) => state = _view());
    state = _view();
  }

  OxideView<CounterState, BenchCounterRiverpodOxideActions> _view() {
    return OxideView(
      state: _core.state,
      actions: actions,
      isLoading: _core.isLoading,
      error: _core.error,
    );
  }

  Future<void> _dispatch(CounterAction action) async {
    await _core.dispatchAction(action);
    state = _view();
  }
}

class BenchCounterBlocOxideActions {
  BenchCounterBlocOxideActions._(
    Future<void> Function(CounterAction action) dispatch,
  ) : _dispatch = dispatch;

  Future<void> Function(CounterAction action) _dispatch;

  void _bind(Future<void> Function(CounterAction action) dispatch) {
    _dispatch = dispatch;
  }

  Future<void> run({required int iterations}) async {
    await _dispatch(CounterAction.run(iterations: iterations));
  }
}

class BenchCounterBlocOxideCubit
    extends Cubit<OxideView<CounterState, BenchCounterBlocOxideActions>> {
  BenchCounterBlocOxideCubit()
    : actions = BenchCounterBlocOxideActions._(_noopDispatch),
      super(
        OxideView(
          state: null,
          actions: BenchCounterBlocOxideActions._(_noopDispatch),
          isLoading: true,
          error: null,
        ),
      ) {
    actions._bind(_dispatch);
    _subscription = _core.snapshots.listen((_) => _emit());
    unawaited(_initialize());
  }

  final BenchCounterBlocOxideActions actions;
  StreamSubscription<CounterStateSnapshot>? _subscription;

  late final OxideStoreCore<
    CounterState,
    CounterAction,
    ArcCounterEngine,
    CounterStateSnapshot
  >
  _core =
      OxideStoreCore<
        CounterState,
        CounterAction,
        ArcCounterEngine,
        CounterStateSnapshot
      >(
        createEngine: (_) => counter_api.createEngine(),
        disposeEngine: (engine) => counter_api.disposeEngine(engine: engine),
        dispatch: (engine, action) =>
            counter_api.dispatch(engine: engine, action: action),
        current: (engine) => counter_api.current(engine: engine),
        stateStream: (engine) => counter_api.stateStream(engine: engine),
        stateFromSnapshot: (snap) => snap.state,
      );

  static Future<void> _noopDispatch(CounterAction _) async {}

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

  Future<void> _dispatch(CounterAction action) async {
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

class BenchCounterBlocOxideScope extends StatefulWidget {
  const BenchCounterBlocOxideScope({
    super.key,
    required this.child,
    this.cubit,
  });

  final Widget child;
  final BenchCounterBlocOxideCubit? cubit;

  static BenchCounterBlocOxideCubit cubitOf(BuildContext context) {
    return BlocProvider.of<BenchCounterBlocOxideCubit>(context);
  }

  @override
  State<BenchCounterBlocOxideScope> createState() =>
      _BenchCounterBlocOxideScopeState();
}

class _BenchCounterBlocOxideScopeState extends State<BenchCounterBlocOxideScope>
    with AutomaticKeepAliveClientMixin {
  late final BenchCounterBlocOxideCubit _cubit;
  late final bool _ownsCubit;

  @override
  bool get wantKeepAlive => false;

  @override
  void initState() {
    super.initState();
    _ownsCubit = widget.cubit == null;
    _cubit = widget.cubit ?? BenchCounterBlocOxideCubit();
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

class BenchCounterHooksOxideActions {
  BenchCounterHooksOxideActions._(
    Future<void> Function(CounterAction action) dispatch,
  ) : _dispatch = dispatch;

  Future<void> Function(CounterAction action) _dispatch;

  void _bind(Future<void> Function(CounterAction action) dispatch) {
    _dispatch = dispatch;
  }

  Future<void> run({required int iterations}) async {
    await _dispatch(CounterAction.run(iterations: iterations));
  }
}

class BenchCounterHooksOxideController extends ChangeNotifier {
  BenchCounterHooksOxideController() {
    actions = BenchCounterHooksOxideActions._(_dispatch);
    actions._bind(_dispatch);
    _subscription = _core.snapshots.listen((_) => _notify());
    unawaited(_initialize());
  }

  late final BenchCounterHooksOxideActions actions;
  StreamSubscription<CounterStateSnapshot>? _subscription;

  late final OxideStoreCore<
    CounterState,
    CounterAction,
    ArcCounterEngine,
    CounterStateSnapshot
  >
  _core =
      OxideStoreCore<
        CounterState,
        CounterAction,
        ArcCounterEngine,
        CounterStateSnapshot
      >(
        createEngine: (_) => counter_api.createEngine(),
        disposeEngine: (engine) => counter_api.disposeEngine(engine: engine),
        dispatch: (engine, action) =>
            counter_api.dispatch(engine: engine, action: action),
        current: (engine) => counter_api.current(engine: engine),
        stateStream: (engine) => counter_api.stateStream(engine: engine),
        stateFromSnapshot: (snap) => snap.state,
      );

  bool _isDisposed = false;

  bool get isLoading => _core.isLoading;
  Object? get error => _core.error;
  CounterState? get state => _core.state;
  ArcCounterEngine? get engine => _core.engine;

  OxideView<CounterState, BenchCounterHooksOxideActions> get oxide => OxideView(
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

  Future<void> _dispatch(CounterAction action) async {
    await _core.dispatchAction(action);
    _notify();
  }
}

class BenchCounterHooksOxideScope extends StatefulWidget {
  const BenchCounterHooksOxideScope({
    super.key,
    required this.child,
    this.controller,
  });

  final Widget child;
  final BenchCounterHooksOxideController? controller;

  static BenchCounterHooksOxideController controllerOf(BuildContext context) {
    final inherited = context
        .dependOnInheritedWidgetOfExactType<_BenchCounterHooksOxideInherited>();
    if (inherited == null || inherited.notifier == null) {
      throw StateError(
        'BenchCounterHooksOxideScope is missing from the widget tree.',
      );
    }
    return inherited.notifier!;
  }

  static OxideView<CounterState, BenchCounterHooksOxideActions> useOxide(
    BuildContext context,
  ) {
    return controllerOf(context).oxide;
  }

  @override
  State<BenchCounterHooksOxideScope> createState() =>
      _BenchCounterHooksOxideScopeState();
}

class _BenchCounterHooksOxideScopeState
    extends State<BenchCounterHooksOxideScope>
    with AutomaticKeepAliveClientMixin {
  late final BenchCounterHooksOxideController _controller;
  late final bool _ownsController;

  @override
  bool get wantKeepAlive => false;

  @override
  void initState() {
    super.initState();
    _ownsController = widget.controller == null;
    _controller = widget.controller ?? BenchCounterHooksOxideController();
  }

  @override
  void dispose() {
    if (_ownsController) _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    super.build(context);
    return _BenchCounterHooksOxideInherited(
      notifier: _controller,
      child: widget.child,
    );
  }
}

class _BenchCounterHooksOxideInherited
    extends InheritedNotifier<BenchCounterHooksOxideController> {
  const _BenchCounterHooksOxideInherited({
    required super.notifier,
    required super.child,
  });
}

OxideView<CounterState, BenchCounterHooksOxideActions>
useBenchCounterHooksOxideOxide() {
  final context = useContext();
  return BenchCounterHooksOxideScope.useOxide(context);
}

class BenchJsonRiverpodOxideActions {
  BenchJsonRiverpodOxideActions._(
    Future<void> Function(JsonAction action) dispatch,
  ) : _dispatch = dispatch;

  Future<void> Function(JsonAction action) _dispatch;

  void _bind(Future<void> Function(JsonAction action) dispatch) {
    _dispatch = dispatch;
  }

  Future<void> runLight({required int iterations}) async {
    await _dispatch(JsonAction.runLight(iterations: iterations));
  }

  Future<void> runHeavy({required int iterations}) async {
    await _dispatch(JsonAction.runHeavy(iterations: iterations));
  }
}

final benchJsonRiverpodOxideProvider =
    NotifierProvider.autoDispose<
      BenchJsonRiverpodOxideNotifier,
      OxideView<JsonState, BenchJsonRiverpodOxideActions>
    >(() => BenchJsonRiverpodOxideNotifier());

class BenchJsonRiverpodOxideNotifier
    extends Notifier<OxideView<JsonState, BenchJsonRiverpodOxideActions>> {
  BenchJsonRiverpodOxideNotifier();

  late final BenchJsonRiverpodOxideActions actions =
      BenchJsonRiverpodOxideActions._(_dispatch);
  StreamSubscription<JsonStateSnapshot>? _subscription;

  late final OxideStoreCore<
    JsonState,
    JsonAction,
    ArcJsonEngine,
    JsonStateSnapshot
  >
  _core =
      OxideStoreCore<JsonState, JsonAction, ArcJsonEngine, JsonStateSnapshot>(
        createEngine: (_) => json_api.createEngine(),
        disposeEngine: (engine) => json_api.disposeEngine(engine: engine),
        dispatch: (engine, action) =>
            json_api.dispatch(engine: engine, action: action),
        current: (engine) => json_api.current(engine: engine),
        stateStream: (engine) => json_api.stateStream(engine: engine),
        stateFromSnapshot: (snap) => snap.state,
      );

  @override
  OxideView<JsonState, BenchJsonRiverpodOxideActions> build() {
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
    _subscription = _core.snapshots.listen((_) => state = _view());
    state = _view();
  }

  OxideView<JsonState, BenchJsonRiverpodOxideActions> _view() {
    return OxideView(
      state: _core.state,
      actions: actions,
      isLoading: _core.isLoading,
      error: _core.error,
    );
  }

  Future<void> _dispatch(JsonAction action) async {
    await _core.dispatchAction(action);
    state = _view();
  }
}

class BenchJsonBlocOxideActions {
  BenchJsonBlocOxideActions._(Future<void> Function(JsonAction action) dispatch)
    : _dispatch = dispatch;

  Future<void> Function(JsonAction action) _dispatch;

  void _bind(Future<void> Function(JsonAction action) dispatch) {
    _dispatch = dispatch;
  }

  Future<void> runLight({required int iterations}) async {
    await _dispatch(JsonAction.runLight(iterations: iterations));
  }

  Future<void> runHeavy({required int iterations}) async {
    await _dispatch(JsonAction.runHeavy(iterations: iterations));
  }
}

class BenchJsonBlocOxideCubit
    extends Cubit<OxideView<JsonState, BenchJsonBlocOxideActions>> {
  BenchJsonBlocOxideCubit()
    : actions = BenchJsonBlocOxideActions._(_noopDispatch),
      super(
        OxideView(
          state: null,
          actions: BenchJsonBlocOxideActions._(_noopDispatch),
          isLoading: true,
          error: null,
        ),
      ) {
    actions._bind(_dispatch);
    _subscription = _core.snapshots.listen((_) => _emit());
    unawaited(_initialize());
  }

  final BenchJsonBlocOxideActions actions;
  StreamSubscription<JsonStateSnapshot>? _subscription;

  late final OxideStoreCore<
    JsonState,
    JsonAction,
    ArcJsonEngine,
    JsonStateSnapshot
  >
  _core =
      OxideStoreCore<JsonState, JsonAction, ArcJsonEngine, JsonStateSnapshot>(
        createEngine: (_) => json_api.createEngine(),
        disposeEngine: (engine) => json_api.disposeEngine(engine: engine),
        dispatch: (engine, action) =>
            json_api.dispatch(engine: engine, action: action),
        current: (engine) => json_api.current(engine: engine),
        stateStream: (engine) => json_api.stateStream(engine: engine),
        stateFromSnapshot: (snap) => snap.state,
      );

  static Future<void> _noopDispatch(JsonAction _) async {}

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

  Future<void> _dispatch(JsonAction action) async {
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

class BenchJsonBlocOxideScope extends StatefulWidget {
  const BenchJsonBlocOxideScope({super.key, required this.child, this.cubit});

  final Widget child;
  final BenchJsonBlocOxideCubit? cubit;

  static BenchJsonBlocOxideCubit cubitOf(BuildContext context) {
    return BlocProvider.of<BenchJsonBlocOxideCubit>(context);
  }

  @override
  State<BenchJsonBlocOxideScope> createState() =>
      _BenchJsonBlocOxideScopeState();
}

class _BenchJsonBlocOxideScopeState extends State<BenchJsonBlocOxideScope>
    with AutomaticKeepAliveClientMixin {
  late final BenchJsonBlocOxideCubit _cubit;
  late final bool _ownsCubit;

  @override
  bool get wantKeepAlive => false;

  @override
  void initState() {
    super.initState();
    _ownsCubit = widget.cubit == null;
    _cubit = widget.cubit ?? BenchJsonBlocOxideCubit();
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

class BenchJsonHooksOxideActions {
  BenchJsonHooksOxideActions._(
    Future<void> Function(JsonAction action) dispatch,
  ) : _dispatch = dispatch;

  Future<void> Function(JsonAction action) _dispatch;

  void _bind(Future<void> Function(JsonAction action) dispatch) {
    _dispatch = dispatch;
  }

  Future<void> runLight({required int iterations}) async {
    await _dispatch(JsonAction.runLight(iterations: iterations));
  }

  Future<void> runHeavy({required int iterations}) async {
    await _dispatch(JsonAction.runHeavy(iterations: iterations));
  }
}

class BenchJsonHooksOxideController extends ChangeNotifier {
  BenchJsonHooksOxideController() {
    actions = BenchJsonHooksOxideActions._(_dispatch);
    actions._bind(_dispatch);
    _subscription = _core.snapshots.listen((_) => _notify());
    unawaited(_initialize());
  }

  late final BenchJsonHooksOxideActions actions;
  StreamSubscription<JsonStateSnapshot>? _subscription;

  late final OxideStoreCore<
    JsonState,
    JsonAction,
    ArcJsonEngine,
    JsonStateSnapshot
  >
  _core =
      OxideStoreCore<JsonState, JsonAction, ArcJsonEngine, JsonStateSnapshot>(
        createEngine: (_) => json_api.createEngine(),
        disposeEngine: (engine) => json_api.disposeEngine(engine: engine),
        dispatch: (engine, action) =>
            json_api.dispatch(engine: engine, action: action),
        current: (engine) => json_api.current(engine: engine),
        stateStream: (engine) => json_api.stateStream(engine: engine),
        stateFromSnapshot: (snap) => snap.state,
      );

  bool _isDisposed = false;

  bool get isLoading => _core.isLoading;
  Object? get error => _core.error;
  JsonState? get state => _core.state;
  ArcJsonEngine? get engine => _core.engine;

  OxideView<JsonState, BenchJsonHooksOxideActions> get oxide => OxideView(
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

  Future<void> _dispatch(JsonAction action) async {
    await _core.dispatchAction(action);
    _notify();
  }
}

class BenchJsonHooksOxideScope extends StatefulWidget {
  const BenchJsonHooksOxideScope({
    super.key,
    required this.child,
    this.controller,
  });

  final Widget child;
  final BenchJsonHooksOxideController? controller;

  static BenchJsonHooksOxideController controllerOf(BuildContext context) {
    final inherited = context
        .dependOnInheritedWidgetOfExactType<_BenchJsonHooksOxideInherited>();
    if (inherited == null || inherited.notifier == null) {
      throw StateError(
        'BenchJsonHooksOxideScope is missing from the widget tree.',
      );
    }
    return inherited.notifier!;
  }

  static OxideView<JsonState, BenchJsonHooksOxideActions> useOxide(
    BuildContext context,
  ) {
    return controllerOf(context).oxide;
  }

  @override
  State<BenchJsonHooksOxideScope> createState() =>
      _BenchJsonHooksOxideScopeState();
}

class _BenchJsonHooksOxideScopeState extends State<BenchJsonHooksOxideScope>
    with AutomaticKeepAliveClientMixin {
  late final BenchJsonHooksOxideController _controller;
  late final bool _ownsController;

  @override
  bool get wantKeepAlive => false;

  @override
  void initState() {
    super.initState();
    _ownsController = widget.controller == null;
    _controller = widget.controller ?? BenchJsonHooksOxideController();
  }

  @override
  void dispose() {
    if (_ownsController) _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    super.build(context);
    return _BenchJsonHooksOxideInherited(
      notifier: _controller,
      child: widget.child,
    );
  }
}

class _BenchJsonHooksOxideInherited
    extends InheritedNotifier<BenchJsonHooksOxideController> {
  const _BenchJsonHooksOxideInherited({
    required super.notifier,
    required super.child,
  });
}

OxideView<JsonState, BenchJsonHooksOxideActions> useBenchJsonHooksOxideOxide() {
  final context = useContext();
  return BenchJsonHooksOxideScope.useOxide(context);
}

class BenchSieveRiverpodOxideActions {
  BenchSieveRiverpodOxideActions._(
    Future<void> Function(SieveAction action) dispatch,
  ) : _dispatch = dispatch;

  Future<void> Function(SieveAction action) _dispatch;

  void _bind(Future<void> Function(SieveAction action) dispatch) {
    _dispatch = dispatch;
  }

  Future<void> run({required int iterations}) async {
    await _dispatch(SieveAction.run(iterations: iterations));
  }
}

final benchSieveRiverpodOxideProvider =
    NotifierProvider.autoDispose<
      BenchSieveRiverpodOxideNotifier,
      OxideView<SieveState, BenchSieveRiverpodOxideActions>
    >(() => BenchSieveRiverpodOxideNotifier());

class BenchSieveRiverpodOxideNotifier
    extends Notifier<OxideView<SieveState, BenchSieveRiverpodOxideActions>> {
  BenchSieveRiverpodOxideNotifier();

  late final BenchSieveRiverpodOxideActions actions =
      BenchSieveRiverpodOxideActions._(_dispatch);
  StreamSubscription<SieveStateSnapshot>? _subscription;

  late final OxideStoreCore<
    SieveState,
    SieveAction,
    ArcSieveEngine,
    SieveStateSnapshot
  >
  _core =
      OxideStoreCore<
        SieveState,
        SieveAction,
        ArcSieveEngine,
        SieveStateSnapshot
      >(
        createEngine: (_) => sieve_api.createEngine(),
        disposeEngine: (engine) => sieve_api.disposeEngine(engine: engine),
        dispatch: (engine, action) =>
            sieve_api.dispatch(engine: engine, action: action),
        current: (engine) => sieve_api.current(engine: engine),
        stateStream: (engine) => sieve_api.stateStream(engine: engine),
        stateFromSnapshot: (snap) => snap.state,
      );

  @override
  OxideView<SieveState, BenchSieveRiverpodOxideActions> build() {
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
    _subscription = _core.snapshots.listen((_) => state = _view());
    state = _view();
  }

  OxideView<SieveState, BenchSieveRiverpodOxideActions> _view() {
    return OxideView(
      state: _core.state,
      actions: actions,
      isLoading: _core.isLoading,
      error: _core.error,
    );
  }

  Future<void> _dispatch(SieveAction action) async {
    await _core.dispatchAction(action);
    state = _view();
  }
}

class BenchSieveBlocOxideActions {
  BenchSieveBlocOxideActions._(
    Future<void> Function(SieveAction action) dispatch,
  ) : _dispatch = dispatch;

  Future<void> Function(SieveAction action) _dispatch;

  void _bind(Future<void> Function(SieveAction action) dispatch) {
    _dispatch = dispatch;
  }

  Future<void> run({required int iterations}) async {
    await _dispatch(SieveAction.run(iterations: iterations));
  }
}

class BenchSieveBlocOxideCubit
    extends Cubit<OxideView<SieveState, BenchSieveBlocOxideActions>> {
  BenchSieveBlocOxideCubit()
    : actions = BenchSieveBlocOxideActions._(_noopDispatch),
      super(
        OxideView(
          state: null,
          actions: BenchSieveBlocOxideActions._(_noopDispatch),
          isLoading: true,
          error: null,
        ),
      ) {
    actions._bind(_dispatch);
    _subscription = _core.snapshots.listen((_) => _emit());
    unawaited(_initialize());
  }

  final BenchSieveBlocOxideActions actions;
  StreamSubscription<SieveStateSnapshot>? _subscription;

  late final OxideStoreCore<
    SieveState,
    SieveAction,
    ArcSieveEngine,
    SieveStateSnapshot
  >
  _core =
      OxideStoreCore<
        SieveState,
        SieveAction,
        ArcSieveEngine,
        SieveStateSnapshot
      >(
        createEngine: (_) => sieve_api.createEngine(),
        disposeEngine: (engine) => sieve_api.disposeEngine(engine: engine),
        dispatch: (engine, action) =>
            sieve_api.dispatch(engine: engine, action: action),
        current: (engine) => sieve_api.current(engine: engine),
        stateStream: (engine) => sieve_api.stateStream(engine: engine),
        stateFromSnapshot: (snap) => snap.state,
      );

  static Future<void> _noopDispatch(SieveAction _) async {}

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

  Future<void> _dispatch(SieveAction action) async {
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

class BenchSieveBlocOxideScope extends StatefulWidget {
  const BenchSieveBlocOxideScope({super.key, required this.child, this.cubit});

  final Widget child;
  final BenchSieveBlocOxideCubit? cubit;

  static BenchSieveBlocOxideCubit cubitOf(BuildContext context) {
    return BlocProvider.of<BenchSieveBlocOxideCubit>(context);
  }

  @override
  State<BenchSieveBlocOxideScope> createState() =>
      _BenchSieveBlocOxideScopeState();
}

class _BenchSieveBlocOxideScopeState extends State<BenchSieveBlocOxideScope>
    with AutomaticKeepAliveClientMixin {
  late final BenchSieveBlocOxideCubit _cubit;
  late final bool _ownsCubit;

  @override
  bool get wantKeepAlive => false;

  @override
  void initState() {
    super.initState();
    _ownsCubit = widget.cubit == null;
    _cubit = widget.cubit ?? BenchSieveBlocOxideCubit();
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

class BenchSieveHooksOxideActions {
  BenchSieveHooksOxideActions._(
    Future<void> Function(SieveAction action) dispatch,
  ) : _dispatch = dispatch;

  Future<void> Function(SieveAction action) _dispatch;

  void _bind(Future<void> Function(SieveAction action) dispatch) {
    _dispatch = dispatch;
  }

  Future<void> run({required int iterations}) async {
    await _dispatch(SieveAction.run(iterations: iterations));
  }
}

class BenchSieveHooksOxideController extends ChangeNotifier {
  BenchSieveHooksOxideController() {
    actions = BenchSieveHooksOxideActions._(_dispatch);
    actions._bind(_dispatch);
    _subscription = _core.snapshots.listen((_) => _notify());
    unawaited(_initialize());
  }

  late final BenchSieveHooksOxideActions actions;
  StreamSubscription<SieveStateSnapshot>? _subscription;

  late final OxideStoreCore<
    SieveState,
    SieveAction,
    ArcSieveEngine,
    SieveStateSnapshot
  >
  _core =
      OxideStoreCore<
        SieveState,
        SieveAction,
        ArcSieveEngine,
        SieveStateSnapshot
      >(
        createEngine: (_) => sieve_api.createEngine(),
        disposeEngine: (engine) => sieve_api.disposeEngine(engine: engine),
        dispatch: (engine, action) =>
            sieve_api.dispatch(engine: engine, action: action),
        current: (engine) => sieve_api.current(engine: engine),
        stateStream: (engine) => sieve_api.stateStream(engine: engine),
        stateFromSnapshot: (snap) => snap.state,
      );

  bool _isDisposed = false;

  bool get isLoading => _core.isLoading;
  Object? get error => _core.error;
  SieveState? get state => _core.state;
  ArcSieveEngine? get engine => _core.engine;

  OxideView<SieveState, BenchSieveHooksOxideActions> get oxide => OxideView(
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

  Future<void> _dispatch(SieveAction action) async {
    await _core.dispatchAction(action);
    _notify();
  }
}

class BenchSieveHooksOxideScope extends StatefulWidget {
  const BenchSieveHooksOxideScope({
    super.key,
    required this.child,
    this.controller,
  });

  final Widget child;
  final BenchSieveHooksOxideController? controller;

  static BenchSieveHooksOxideController controllerOf(BuildContext context) {
    final inherited = context
        .dependOnInheritedWidgetOfExactType<_BenchSieveHooksOxideInherited>();
    if (inherited == null || inherited.notifier == null) {
      throw StateError(
        'BenchSieveHooksOxideScope is missing from the widget tree.',
      );
    }
    return inherited.notifier!;
  }

  static OxideView<SieveState, BenchSieveHooksOxideActions> useOxide(
    BuildContext context,
  ) {
    return controllerOf(context).oxide;
  }

  @override
  State<BenchSieveHooksOxideScope> createState() =>
      _BenchSieveHooksOxideScopeState();
}

class _BenchSieveHooksOxideScopeState extends State<BenchSieveHooksOxideScope>
    with AutomaticKeepAliveClientMixin {
  late final BenchSieveHooksOxideController _controller;
  late final bool _ownsController;

  @override
  bool get wantKeepAlive => false;

  @override
  void initState() {
    super.initState();
    _ownsController = widget.controller == null;
    _controller = widget.controller ?? BenchSieveHooksOxideController();
  }

  @override
  void dispose() {
    if (_ownsController) _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    super.build(context);
    return _BenchSieveHooksOxideInherited(
      notifier: _controller,
      child: widget.child,
    );
  }
}

class _BenchSieveHooksOxideInherited
    extends InheritedNotifier<BenchSieveHooksOxideController> {
  const _BenchSieveHooksOxideInherited({
    required super.notifier,
    required super.child,
  });
}

OxideView<SieveState, BenchSieveHooksOxideActions>
useBenchSieveHooksOxideOxide() {
  final context = useContext();
  return BenchSieveHooksOxideScope.useOxide(context);
}
