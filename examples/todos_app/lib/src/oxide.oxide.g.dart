// GENERATED CODE - DO NOT MODIFY BY HAND
// dart format width=80

part of 'oxide.dart';

// **************************************************************************
// OxideStoreGenerator
// **************************************************************************

class TodosListInheritedOxideActions {
  TodosListInheritedOxideActions._(
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

class TodosListInheritedOxideController extends ChangeNotifier {
  TodosListInheritedOxideController() {
    actions = TodosListInheritedOxideActions._(_dispatch);
    actions._bind(_dispatch);
    _subscription = filterSnapshotsBySlices<AppStateSnapshot, AppStateSlice>(
      _core.snapshots,
      const [AppStateSlice.todos],
      (snap) => snap.slices,
    ).listen((_) => _notify());
    unawaited(_initialize());
  }

  late final TodosListInheritedOxideActions actions;
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

  OxideView<AppState, TodosListInheritedOxideActions> get oxide => OxideView(
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

class TodosListInheritedOxideScope extends StatefulWidget {
  const TodosListInheritedOxideScope({
    super.key,
    required this.child,
    this.controller,
  });

  final Widget child;
  final TodosListInheritedOxideController? controller;

  static TodosListInheritedOxideController controllerOf(BuildContext context) {
    final inherited = context
        .dependOnInheritedWidgetOfExactType<
          _TodosListInheritedOxideInherited
        >();
    if (inherited == null || inherited.notifier == null) {
      throw StateError(
        'TodosListInheritedOxideScope is missing from the widget tree.',
      );
    }
    return inherited.notifier!;
  }

  static OxideView<AppState, TodosListInheritedOxideActions> useOxide(
    BuildContext context,
  ) {
    return controllerOf(context).oxide;
  }

  @override
  State<TodosListInheritedOxideScope> createState() =>
      _TodosListInheritedOxideScopeState();
}

class _TodosListInheritedOxideScopeState
    extends State<TodosListInheritedOxideScope>
    with AutomaticKeepAliveClientMixin {
  late final TodosListInheritedOxideController _controller;
  late final bool _ownsController;

  @override
  bool get wantKeepAlive => false;

  @override
  void initState() {
    super.initState();
    _ownsController = widget.controller == null;
    _controller = widget.controller ?? TodosListInheritedOxideController();
  }

  @override
  void dispose() {
    if (_ownsController) _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    super.build(context);
    return _TodosListInheritedOxideInherited(
      notifier: _controller,
      child: widget.child,
    );
  }
}

class _TodosListInheritedOxideInherited
    extends InheritedNotifier<TodosListInheritedOxideController> {
  const _TodosListInheritedOxideInherited({
    required super.notifier,
    required super.child,
  });
}

class TodosNextIdInheritedOxideActions {
  TodosNextIdInheritedOxideActions._(
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

class TodosNextIdInheritedOxideController extends ChangeNotifier {
  TodosNextIdInheritedOxideController() {
    actions = TodosNextIdInheritedOxideActions._(_dispatch);
    actions._bind(_dispatch);
    _subscription = filterSnapshotsBySlices<AppStateSnapshot, AppStateSlice>(
      _core.snapshots,
      const [AppStateSlice.nextId],
      (snap) => snap.slices,
    ).listen((_) => _notify());
    unawaited(_initialize());
  }

  late final TodosNextIdInheritedOxideActions actions;
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

  OxideView<AppState, TodosNextIdInheritedOxideActions> get oxide => OxideView(
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

class TodosNextIdInheritedOxideScope extends StatefulWidget {
  const TodosNextIdInheritedOxideScope({
    super.key,
    required this.child,
    this.controller,
  });

  final Widget child;
  final TodosNextIdInheritedOxideController? controller;

  static TodosNextIdInheritedOxideController controllerOf(
    BuildContext context,
  ) {
    final inherited = context
        .dependOnInheritedWidgetOfExactType<
          _TodosNextIdInheritedOxideInherited
        >();
    if (inherited == null || inherited.notifier == null) {
      throw StateError(
        'TodosNextIdInheritedOxideScope is missing from the widget tree.',
      );
    }
    return inherited.notifier!;
  }

  static OxideView<AppState, TodosNextIdInheritedOxideActions> useOxide(
    BuildContext context,
  ) {
    return controllerOf(context).oxide;
  }

  @override
  State<TodosNextIdInheritedOxideScope> createState() =>
      _TodosNextIdInheritedOxideScopeState();
}

class _TodosNextIdInheritedOxideScopeState
    extends State<TodosNextIdInheritedOxideScope>
    with AutomaticKeepAliveClientMixin {
  late final TodosNextIdInheritedOxideController _controller;
  late final bool _ownsController;

  @override
  bool get wantKeepAlive => false;

  @override
  void initState() {
    super.initState();
    _ownsController = widget.controller == null;
    _controller = widget.controller ?? TodosNextIdInheritedOxideController();
  }

  @override
  void dispose() {
    if (_ownsController) _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    super.build(context);
    return _TodosNextIdInheritedOxideInherited(
      notifier: _controller,
      child: widget.child,
    );
  }
}

class _TodosNextIdInheritedOxideInherited
    extends InheritedNotifier<TodosNextIdInheritedOxideController> {
  const _TodosNextIdInheritedOxideInherited({
    required super.notifier,
    required super.child,
  });
}

class TodosListHooksOxideActions {
  TodosListHooksOxideActions._(Future<void> Function(AppAction action) dispatch)
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

class TodosListHooksOxideController extends ChangeNotifier {
  TodosListHooksOxideController() {
    actions = TodosListHooksOxideActions._(_dispatch);
    actions._bind(_dispatch);
    _subscription = filterSnapshotsBySlices<AppStateSnapshot, AppStateSlice>(
      _core.snapshots,
      const [AppStateSlice.todos],
      (snap) => snap.slices,
    ).listen((_) => _notify());
    unawaited(_initialize());
  }

  late final TodosListHooksOxideActions actions;
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

  OxideView<AppState, TodosListHooksOxideActions> get oxide => OxideView(
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

class TodosListHooksOxideScope extends StatefulWidget {
  const TodosListHooksOxideScope({
    super.key,
    required this.child,
    this.controller,
  });

  final Widget child;
  final TodosListHooksOxideController? controller;

  static TodosListHooksOxideController controllerOf(BuildContext context) {
    final inherited = context
        .dependOnInheritedWidgetOfExactType<_TodosListHooksOxideInherited>();
    if (inherited == null || inherited.notifier == null) {
      throw StateError(
        'TodosListHooksOxideScope is missing from the widget tree.',
      );
    }
    return inherited.notifier!;
  }

  static OxideView<AppState, TodosListHooksOxideActions> useOxide(
    BuildContext context,
  ) {
    return controllerOf(context).oxide;
  }

  @override
  State<TodosListHooksOxideScope> createState() =>
      _TodosListHooksOxideScopeState();
}

class _TodosListHooksOxideScopeState extends State<TodosListHooksOxideScope>
    with AutomaticKeepAliveClientMixin {
  late final TodosListHooksOxideController _controller;
  late final bool _ownsController;

  @override
  bool get wantKeepAlive => false;

  @override
  void initState() {
    super.initState();
    _ownsController = widget.controller == null;
    _controller = widget.controller ?? TodosListHooksOxideController();
  }

  @override
  void dispose() {
    if (_ownsController) _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    super.build(context);
    return _TodosListHooksOxideInherited(
      notifier: _controller,
      child: widget.child,
    );
  }
}

class _TodosListHooksOxideInherited
    extends InheritedNotifier<TodosListHooksOxideController> {
  const _TodosListHooksOxideInherited({
    required super.notifier,
    required super.child,
  });
}

OxideView<AppState, TodosListHooksOxideActions> useTodosListHooksOxideOxide() {
  final context = useContext();
  return TodosListHooksOxideScope.useOxide(context);
}

class TodosNextIdHooksOxideActions {
  TodosNextIdHooksOxideActions._(
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

class TodosNextIdHooksOxideController extends ChangeNotifier {
  TodosNextIdHooksOxideController() {
    actions = TodosNextIdHooksOxideActions._(_dispatch);
    actions._bind(_dispatch);
    _subscription = filterSnapshotsBySlices<AppStateSnapshot, AppStateSlice>(
      _core.snapshots,
      const [AppStateSlice.nextId],
      (snap) => snap.slices,
    ).listen((_) => _notify());
    unawaited(_initialize());
  }

  late final TodosNextIdHooksOxideActions actions;
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

  OxideView<AppState, TodosNextIdHooksOxideActions> get oxide => OxideView(
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

class TodosNextIdHooksOxideScope extends StatefulWidget {
  const TodosNextIdHooksOxideScope({
    super.key,
    required this.child,
    this.controller,
  });

  final Widget child;
  final TodosNextIdHooksOxideController? controller;

  static TodosNextIdHooksOxideController controllerOf(BuildContext context) {
    final inherited = context
        .dependOnInheritedWidgetOfExactType<_TodosNextIdHooksOxideInherited>();
    if (inherited == null || inherited.notifier == null) {
      throw StateError(
        'TodosNextIdHooksOxideScope is missing from the widget tree.',
      );
    }
    return inherited.notifier!;
  }

  static OxideView<AppState, TodosNextIdHooksOxideActions> useOxide(
    BuildContext context,
  ) {
    return controllerOf(context).oxide;
  }

  @override
  State<TodosNextIdHooksOxideScope> createState() =>
      _TodosNextIdHooksOxideScopeState();
}

class _TodosNextIdHooksOxideScopeState extends State<TodosNextIdHooksOxideScope>
    with AutomaticKeepAliveClientMixin {
  late final TodosNextIdHooksOxideController _controller;
  late final bool _ownsController;

  @override
  bool get wantKeepAlive => false;

  @override
  void initState() {
    super.initState();
    _ownsController = widget.controller == null;
    _controller = widget.controller ?? TodosNextIdHooksOxideController();
  }

  @override
  void dispose() {
    if (_ownsController) _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    super.build(context);
    return _TodosNextIdHooksOxideInherited(
      notifier: _controller,
      child: widget.child,
    );
  }
}

class _TodosNextIdHooksOxideInherited
    extends InheritedNotifier<TodosNextIdHooksOxideController> {
  const _TodosNextIdHooksOxideInherited({
    required super.notifier,
    required super.child,
  });
}

OxideView<AppState, TodosNextIdHooksOxideActions>
useTodosNextIdHooksOxideOxide() {
  final context = useContext();
  return TodosNextIdHooksOxideScope.useOxide(context);
}

class TodosListRiverpodOxideActions {
  TodosListRiverpodOxideActions._(
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

final todosListRiverpodOxideProvider =
    NotifierProvider.autoDispose<
      TodosListRiverpodOxideNotifier,
      OxideView<AppState, TodosListRiverpodOxideActions>
    >(() => TodosListRiverpodOxideNotifier());

class TodosListRiverpodOxideNotifier
    extends Notifier<OxideView<AppState, TodosListRiverpodOxideActions>> {
  TodosListRiverpodOxideNotifier();

  late final TodosListRiverpodOxideActions actions =
      TodosListRiverpodOxideActions._(_dispatch);
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
  OxideView<AppState, TodosListRiverpodOxideActions> build() {
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
      const [AppStateSlice.todos],
      (snap) => snap.slices,
    ).listen((_) => state = _view());
    state = _view();
  }

  OxideView<AppState, TodosListRiverpodOxideActions> _view() {
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

class TodosNextIdRiverpodOxideActions {
  TodosNextIdRiverpodOxideActions._(
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

final todosNextIdRiverpodOxideProvider =
    NotifierProvider.autoDispose<
      TodosNextIdRiverpodOxideNotifier,
      OxideView<AppState, TodosNextIdRiverpodOxideActions>
    >(() => TodosNextIdRiverpodOxideNotifier());

class TodosNextIdRiverpodOxideNotifier
    extends Notifier<OxideView<AppState, TodosNextIdRiverpodOxideActions>> {
  TodosNextIdRiverpodOxideNotifier();

  late final TodosNextIdRiverpodOxideActions actions =
      TodosNextIdRiverpodOxideActions._(_dispatch);
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
  OxideView<AppState, TodosNextIdRiverpodOxideActions> build() {
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
      const [AppStateSlice.nextId],
      (snap) => snap.slices,
    ).listen((_) => state = _view());
    state = _view();
  }

  OxideView<AppState, TodosNextIdRiverpodOxideActions> _view() {
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

class TodosListBlocOxideActions {
  TodosListBlocOxideActions._(Future<void> Function(AppAction action) dispatch)
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

class TodosListBlocOxideCubit
    extends Cubit<OxideView<AppState, TodosListBlocOxideActions>> {
  TodosListBlocOxideCubit()
    : actions = TodosListBlocOxideActions._(_noopDispatch),
      super(
        OxideView(
          state: null,
          actions: TodosListBlocOxideActions._(_noopDispatch),
          isLoading: true,
          error: null,
        ),
      ) {
    actions._bind(_dispatch);
    _subscription = filterSnapshotsBySlices<AppStateSnapshot, AppStateSlice>(
      _core.snapshots,
      const [AppStateSlice.todos],
      (snap) => snap.slices,
    ).listen((_) => _emit());
    unawaited(_initialize());
  }

  final TodosListBlocOxideActions actions;
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

class TodosListBlocOxideScope extends StatefulWidget {
  const TodosListBlocOxideScope({super.key, required this.child, this.cubit});

  final Widget child;
  final TodosListBlocOxideCubit? cubit;

  static TodosListBlocOxideCubit cubitOf(BuildContext context) {
    return BlocProvider.of<TodosListBlocOxideCubit>(context);
  }

  @override
  State<TodosListBlocOxideScope> createState() =>
      _TodosListBlocOxideScopeState();
}

class _TodosListBlocOxideScopeState extends State<TodosListBlocOxideScope>
    with AutomaticKeepAliveClientMixin {
  late final TodosListBlocOxideCubit _cubit;
  late final bool _ownsCubit;

  @override
  bool get wantKeepAlive => false;

  @override
  void initState() {
    super.initState();
    _ownsCubit = widget.cubit == null;
    _cubit = widget.cubit ?? TodosListBlocOxideCubit();
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

class TodosNextIdBlocOxideActions {
  TodosNextIdBlocOxideActions._(
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

class TodosNextIdBlocOxideCubit
    extends Cubit<OxideView<AppState, TodosNextIdBlocOxideActions>> {
  TodosNextIdBlocOxideCubit()
    : actions = TodosNextIdBlocOxideActions._(_noopDispatch),
      super(
        OxideView(
          state: null,
          actions: TodosNextIdBlocOxideActions._(_noopDispatch),
          isLoading: true,
          error: null,
        ),
      ) {
    actions._bind(_dispatch);
    _subscription = filterSnapshotsBySlices<AppStateSnapshot, AppStateSlice>(
      _core.snapshots,
      const [AppStateSlice.nextId],
      (snap) => snap.slices,
    ).listen((_) => _emit());
    unawaited(_initialize());
  }

  final TodosNextIdBlocOxideActions actions;
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

class TodosNextIdBlocOxideScope extends StatefulWidget {
  const TodosNextIdBlocOxideScope({super.key, required this.child, this.cubit});

  final Widget child;
  final TodosNextIdBlocOxideCubit? cubit;

  static TodosNextIdBlocOxideCubit cubitOf(BuildContext context) {
    return BlocProvider.of<TodosNextIdBlocOxideCubit>(context);
  }

  @override
  State<TodosNextIdBlocOxideScope> createState() =>
      _TodosNextIdBlocOxideScopeState();
}

class _TodosNextIdBlocOxideScopeState extends State<TodosNextIdBlocOxideScope>
    with AutomaticKeepAliveClientMixin {
  late final TodosNextIdBlocOxideCubit _cubit;
  late final bool _ownsCubit;

  @override
  bool get wantKeepAlive => false;

  @override
  void initState() {
    super.initState();
    _ownsCubit = widget.cubit == null;
    _cubit = widget.cubit ?? TodosNextIdBlocOxideCubit();
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
