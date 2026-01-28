// GENERATED CODE - DO NOT MODIFY BY HAND
// dart format width=80

part of 'oxide.dart';

// **************************************************************************
// OxideStoreGenerator
// **************************************************************************

class StateBridgeOxideActions {
  StateBridgeOxideActions._(Future<void> Function(AppAction action) dispatch)
    : _dispatch = dispatch;

  Future<void> Function(AppAction action) _dispatch;

  void _bind(Future<void> Function(AppAction action) dispatch) {
    _dispatch = dispatch;
  }

  Future<void> addTodo({required String title}) async {
    await _dispatch(AppAction.addTodo(title: title));
  }

  Future<void> toggleTodo({required String id}) async {
    await _dispatch(AppAction.toggleTodo(id: id));
  }

  Future<void> deleteTodo({required String id}) async {
    await _dispatch(AppAction.deleteTodo(id: id));
  }
}

class StateBridgeOxideController extends ChangeNotifier {
  StateBridgeOxideController() {
    actions = StateBridgeOxideActions._(_dispatch);
    actions._bind(_dispatch);
    _subscription = _core.snapshots.listen((_) => _notify());
    unawaited(_initialize());
  }

  late final StateBridgeOxideActions actions;
  StreamSubscription<AppStateSnapshot>? _subscription;

  late final OxideStoreCore<AppState, AppAction, ArcAppEngine, AppStateSnapshot>
  _core = OxideStoreCore<AppState, AppAction, ArcAppEngine, AppStateSnapshot>(
    createEngine: (_) => createEngine(),
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

  OxideView<AppState, StateBridgeOxideActions> get oxide => OxideView(
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

class StateBridgeOxideScope extends StatefulWidget {
  const StateBridgeOxideScope({
    super.key,
    required this.child,
    this.controller,
  });

  final Widget child;
  final StateBridgeOxideController? controller;

  static StateBridgeOxideController controllerOf(BuildContext context) {
    final inherited = context
        .dependOnInheritedWidgetOfExactType<_StateBridgeOxideInherited>();
    if (inherited == null || inherited.notifier == null) {
      throw StateError(
        'StateBridgeOxideScope is missing from the widget tree.',
      );
    }
    return inherited.notifier!;
  }

  static OxideView<AppState, StateBridgeOxideActions> useOxide(
    BuildContext context,
  ) {
    return controllerOf(context).oxide;
  }

  @override
  State<StateBridgeOxideScope> createState() => _StateBridgeOxideScopeState();
}

class _StateBridgeOxideScopeState extends State<StateBridgeOxideScope>
    with AutomaticKeepAliveClientMixin {
  late final StateBridgeOxideController _controller;
  late final bool _ownsController;

  @override
  bool get wantKeepAlive => false;

  @override
  void initState() {
    super.initState();
    _ownsController = widget.controller == null;
    _controller = widget.controller ?? StateBridgeOxideController();
  }

  @override
  void dispose() {
    if (_ownsController) _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    super.build(context);
    return _StateBridgeOxideInherited(
      notifier: _controller,
      child: widget.child,
    );
  }
}

class _StateBridgeOxideInherited
    extends InheritedNotifier<StateBridgeOxideController> {
  const _StateBridgeOxideInherited({
    required super.notifier,
    required super.child,
  });
}

class StateBridgeHooksOxideActions {
  StateBridgeHooksOxideActions._(
    Future<void> Function(AppAction action) dispatch,
  ) : _dispatch = dispatch;

  Future<void> Function(AppAction action) _dispatch;

  void _bind(Future<void> Function(AppAction action) dispatch) {
    _dispatch = dispatch;
  }

  Future<void> addTodo({required String title}) async {
    await _dispatch(AppAction.addTodo(title: title));
  }

  Future<void> toggleTodo({required String id}) async {
    await _dispatch(AppAction.toggleTodo(id: id));
  }

  Future<void> deleteTodo({required String id}) async {
    await _dispatch(AppAction.deleteTodo(id: id));
  }
}

class StateBridgeHooksOxideController extends ChangeNotifier {
  StateBridgeHooksOxideController() {
    actions = StateBridgeHooksOxideActions._(_dispatch);
    actions._bind(_dispatch);
    _subscription = _core.snapshots.listen((_) => _notify());
    unawaited(_initialize());
  }

  late final StateBridgeHooksOxideActions actions;
  StreamSubscription<AppStateSnapshot>? _subscription;

  late final OxideStoreCore<AppState, AppAction, ArcAppEngine, AppStateSnapshot>
  _core = OxideStoreCore<AppState, AppAction, ArcAppEngine, AppStateSnapshot>(
    createEngine: (_) => createEngine(),
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

  OxideView<AppState, StateBridgeHooksOxideActions> get oxide => OxideView(
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

class StateBridgeHooksOxideScope extends StatefulWidget {
  const StateBridgeHooksOxideScope({
    super.key,
    required this.child,
    this.controller,
  });

  final Widget child;
  final StateBridgeHooksOxideController? controller;

  static StateBridgeHooksOxideController controllerOf(BuildContext context) {
    final inherited = context
        .dependOnInheritedWidgetOfExactType<_StateBridgeHooksOxideInherited>();
    if (inherited == null || inherited.notifier == null) {
      throw StateError(
        'StateBridgeHooksOxideScope is missing from the widget tree.',
      );
    }
    return inherited.notifier!;
  }

  static OxideView<AppState, StateBridgeHooksOxideActions> useOxide(
    BuildContext context,
  ) {
    return controllerOf(context).oxide;
  }

  @override
  State<StateBridgeHooksOxideScope> createState() =>
      _StateBridgeHooksOxideScopeState();
}

class _StateBridgeHooksOxideScopeState extends State<StateBridgeHooksOxideScope>
    with AutomaticKeepAliveClientMixin {
  late final StateBridgeHooksOxideController _controller;
  late final bool _ownsController;

  @override
  bool get wantKeepAlive => false;

  @override
  void initState() {
    super.initState();
    _ownsController = widget.controller == null;
    _controller = widget.controller ?? StateBridgeHooksOxideController();
  }

  @override
  void dispose() {
    if (_ownsController) _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    super.build(context);
    return _StateBridgeHooksOxideInherited(
      notifier: _controller,
      child: widget.child,
    );
  }
}

class _StateBridgeHooksOxideInherited
    extends InheritedNotifier<StateBridgeHooksOxideController> {
  const _StateBridgeHooksOxideInherited({
    required super.notifier,
    required super.child,
  });
}

OxideView<AppState, StateBridgeHooksOxideActions>
useStateBridgeHooksOxideOxide() {
  final context = useContext();
  return StateBridgeHooksOxideScope.useOxide(context);
}

class StateBridgeRiverpodOxideActions {
  StateBridgeRiverpodOxideActions._(
    Future<void> Function(AppAction action) dispatch,
  ) : _dispatch = dispatch;

  Future<void> Function(AppAction action) _dispatch;

  void _bind(Future<void> Function(AppAction action) dispatch) {
    _dispatch = dispatch;
  }

  Future<void> addTodo({required String title}) async {
    await _dispatch(AppAction.addTodo(title: title));
  }

  Future<void> toggleTodo({required String id}) async {
    await _dispatch(AppAction.toggleTodo(id: id));
  }

  Future<void> deleteTodo({required String id}) async {
    await _dispatch(AppAction.deleteTodo(id: id));
  }
}

final stateBridgeRiverpodOxideProvider =
    NotifierProvider.autoDispose<
      StateBridgeRiverpodOxideNotifier,
      OxideView<AppState, StateBridgeRiverpodOxideActions>
    >(() => StateBridgeRiverpodOxideNotifier());

class StateBridgeRiverpodOxideNotifier
    extends Notifier<OxideView<AppState, StateBridgeRiverpodOxideActions>> {
  StateBridgeRiverpodOxideNotifier();

  late final StateBridgeRiverpodOxideActions actions =
      StateBridgeRiverpodOxideActions._(_dispatch);
  StreamSubscription<AppStateSnapshot>? _subscription;

  late final OxideStoreCore<AppState, AppAction, ArcAppEngine, AppStateSnapshot>
  _core = OxideStoreCore<AppState, AppAction, ArcAppEngine, AppStateSnapshot>(
    createEngine: (_) => createEngine(),
    disposeEngine: (engine) => disposeEngine(engine: engine),
    dispatch: (engine, action) => dispatch(engine: engine, action: action),
    current: (engine) => current(engine: engine),
    stateStream: (engine) => stateStream(engine: engine),
    stateFromSnapshot: (snap) => snap.state,
  );

  @override
  OxideView<AppState, StateBridgeRiverpodOxideActions> build() {
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

  OxideView<AppState, StateBridgeRiverpodOxideActions> _view() {
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

class StateBridgeBlocOxideActions {
  StateBridgeBlocOxideActions._(
    Future<void> Function(AppAction action) dispatch,
  ) : _dispatch = dispatch;

  Future<void> Function(AppAction action) _dispatch;

  void _bind(Future<void> Function(AppAction action) dispatch) {
    _dispatch = dispatch;
  }

  Future<void> addTodo({required String title}) async {
    await _dispatch(AppAction.addTodo(title: title));
  }

  Future<void> toggleTodo({required String id}) async {
    await _dispatch(AppAction.toggleTodo(id: id));
  }

  Future<void> deleteTodo({required String id}) async {
    await _dispatch(AppAction.deleteTodo(id: id));
  }
}

class StateBridgeBlocOxideCubit
    extends Cubit<OxideView<AppState, StateBridgeBlocOxideActions>> {
  StateBridgeBlocOxideCubit()
    : actions = StateBridgeBlocOxideActions._(_noopDispatch),
      super(
        OxideView(
          state: null,
          actions: StateBridgeBlocOxideActions._(_noopDispatch),
          isLoading: true,
          error: null,
        ),
      ) {
    actions._bind(_dispatch);
    _subscription = _core.snapshots.listen((_) => _emit());
    unawaited(_initialize());
  }

  final StateBridgeBlocOxideActions actions;
  StreamSubscription<AppStateSnapshot>? _subscription;

  late final OxideStoreCore<AppState, AppAction, ArcAppEngine, AppStateSnapshot>
  _core = OxideStoreCore<AppState, AppAction, ArcAppEngine, AppStateSnapshot>(
    createEngine: (_) => createEngine(),
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

class StateBridgeBlocOxideScope extends StatefulWidget {
  const StateBridgeBlocOxideScope({super.key, required this.child, this.cubit});

  final Widget child;
  final StateBridgeBlocOxideCubit? cubit;

  static StateBridgeBlocOxideCubit cubitOf(BuildContext context) {
    return BlocProvider.of<StateBridgeBlocOxideCubit>(context);
  }

  @override
  State<StateBridgeBlocOxideScope> createState() =>
      _StateBridgeBlocOxideScopeState();
}

class _StateBridgeBlocOxideScopeState extends State<StateBridgeBlocOxideScope>
    with AutomaticKeepAliveClientMixin {
  late final StateBridgeBlocOxideCubit _cubit;
  late final bool _ownsCubit;

  @override
  bool get wantKeepAlive => false;

  @override
  void initState() {
    super.initState();
    _ownsCubit = widget.cubit == null;
    _cubit = widget.cubit ?? StateBridgeBlocOxideCubit();
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
