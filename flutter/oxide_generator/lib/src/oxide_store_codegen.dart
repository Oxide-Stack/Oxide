/// Configuration used to generate store glue code.
///
/// Instances of this config are produced by [OxideStoreGenerator] from the
/// `@OxideStore(...)` annotation, and then consumed by [generateOxideStoreSource].
final class OxideCodegenConfig {
  const OxideCodegenConfig({
    required this.prefix,
    required this.stateType,
    required this.snapshotType,
    required this.actionsType,
    required this.actionsIsEnum,
    required this.engineType,
    required this.backend,
    required this.keepAlive,
    required this.createEngine,
    required this.disposeEngine,
    required this.dispatch,
    required this.stateStream,
    required this.current,
    required this.initApp,
    required this.encodeCurrentState,
    required this.encodeState,
    required this.decodeState,
    required this.actionConstructors,
  });

  final String prefix;
  final String stateType;
  final String snapshotType;
  final String actionsType;
  final bool actionsIsEnum;
  final String engineType;
  final String backend;
  final bool keepAlive;

  final String createEngine;
  final String disposeEngine;
  final String dispatch;
  final String stateStream;
  final String current;
  final String? initApp;
  final String? encodeCurrentState;
  final String? encodeState;
  final String? decodeState;

  final List<OxideActionConstructor> actionConstructors;
}

/// Describes a generated action constructor helper.
final class OxideActionConstructor {
  const OxideActionConstructor({required this.name, required this.positionalParams, required this.namedParams});

  final String name;
  final List<OxideActionParam> positionalParams;
  final List<OxideActionParam> namedParams;
}

/// Describes a parameter for a generated action constructor helper.
final class OxideActionParam {
  const OxideActionParam({required this.name, required this.type, required this.isRequiredNamed});

  final String name;
  final String type;
  final bool isRequiredNamed;
}

/// Generates Dart source code for a store facade.
///
/// # Returns
/// A full Dart source string that is written into the generated `*.oxide.g.dart`
/// part file.
String generateOxideStoreSource(OxideCodegenConfig c) {
  final actionsMethods = _generateActionsMethods(actionsType: c.actionsType, actionsIsEnum: c.actionsIsEnum, constructors: c.actionConstructors);

  final initAppArg = (c.initApp == null || c.initApp!.isEmpty) ? '' : '    initApp: () => ${c.initApp}(),\n';
  final encodeCurrentStateArg = (c.encodeCurrentState == null || c.encodeCurrentState!.isEmpty)
      ? ''
      : '    encodeCurrentState: (engine) async => await ${c.encodeCurrentState}(engine: engine),\n';
  final coreInstantiation =
      '''
  late final OxideStoreCore<${c.stateType}, ${c.actionsType}, ${c.engineType}, ${c.snapshotType}> _core =
      OxideStoreCore<${c.stateType}, ${c.actionsType}, ${c.engineType}, ${c.snapshotType}>(
    createEngine: (_) => ${c.createEngine}(),
    disposeEngine: (engine) => ${c.disposeEngine}(engine: engine),
    dispatch: (engine, action) => ${c.dispatch}(engine: engine, action: action),
    current: (engine) => ${c.current}(engine: engine),
    stateStream: (engine) => ${c.stateStream}(engine: engine),
    stateFromSnapshot: (snap) => snap.state,
$initAppArg$encodeCurrentStateArg  );
''';

  final actionsClass =
      '''
class ${c.prefix}Actions {
  ${c.prefix}Actions._(Future<void> Function(${c.actionsType} action) dispatch)
      : _dispatch = dispatch;

  Future<void> Function(${c.actionsType} action) _dispatch;

  void _bind(Future<void> Function(${c.actionsType} action) dispatch) {
    _dispatch = dispatch;
  }

$actionsMethods
}
''';

  final inheritedBackend =
      '''
class ${c.prefix}Controller extends ChangeNotifier {
  ${c.prefix}Controller() {
    actions = ${c.prefix}Actions._(_dispatch);
    actions._bind(_dispatch);
    _subscription = _core.snapshots.listen((_) => _notify());
    unawaited(_initialize());
  }

  late final ${c.prefix}Actions actions;
  StreamSubscription<${c.snapshotType}>? _subscription;

$coreInstantiation

  bool _isDisposed = false;

  bool get isLoading => _core.isLoading;
  Object? get error => _core.error;
  ${c.stateType}? get state => _core.state;
  ${c.engineType}? get engine => _core.engine;

  OxideView<${c.stateType}, ${c.prefix}Actions> get oxide => OxideView(
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

  Future<void> _dispatch(${c.actionsType} action) async {
    await _core.dispatchAction(action);
    _notify();
  }
}

class ${c.prefix}Scope extends StatefulWidget {
  const ${c.prefix}Scope({super.key, required this.child, this.controller});

  final Widget child;
  final ${c.prefix}Controller? controller;

  static ${c.prefix}Controller controllerOf(BuildContext context) {
    final inherited =
        context.dependOnInheritedWidgetOfExactType<_${c.prefix}Inherited>();
    if (inherited == null || inherited.notifier == null) {
      throw StateError('${c.prefix}Scope is missing from the widget tree.');
    }
    return inherited.notifier!;
  }

  static OxideView<${c.stateType}, ${c.prefix}Actions> useOxide(
    BuildContext context,
  ) {
    return controllerOf(context).oxide;
  }

  @override
  State<${c.prefix}Scope> createState() => _${c.prefix}ScopeState();
}

class _${c.prefix}ScopeState extends State<${c.prefix}Scope>
    with AutomaticKeepAliveClientMixin {
  late final ${c.prefix}Controller _controller;
  late final bool _ownsController;

  @override
  bool get wantKeepAlive => ${c.keepAlive};

  @override
  void initState() {
    super.initState();
    _ownsController = widget.controller == null;
    _controller = widget.controller ?? ${c.prefix}Controller();
  }

  @override
  void dispose() {
    if (_ownsController) _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    super.build(context);
    return _${c.prefix}Inherited(
      notifier: _controller,
      child: widget.child,
    );
  }
}

class _${c.prefix}Inherited extends InheritedNotifier<${c.prefix}Controller> {
  const _${c.prefix}Inherited({
    required super.notifier,
    required super.child,
  });
}
''';

  final inheritedHooksBackend =
      '''
$inheritedBackend

OxideView<${c.stateType}, ${c.prefix}Actions> use${c.prefix}Oxide() {
  final context = useContext();
  return ${c.prefix}Scope.useOxide(context);
}
''';

  final riverpodBackend =
      '''
final ${_lowerFirst(c.prefix)}Provider = ${c.keepAlive ? 'NotifierProvider' : 'AutoDisposeNotifierProvider'}<
    ${c.prefix}Notifier,
    OxideView<${c.stateType}, ${c.prefix}Actions>>(
  () => ${c.prefix}Notifier(),
);

class ${c.prefix}Notifier
    extends ${c.keepAlive ? 'Notifier' : 'AutoDisposeNotifier'}<OxideView<${c.stateType}, ${c.prefix}Actions>> {
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
    _subscription = _core.snapshots.listen((_) => state = _view());
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
    state = _view();
  }
}
''';

  final blocBackend =
      '''
class ${c.prefix}Cubit extends Cubit<OxideView<${c.stateType}, ${c.prefix}Actions>> {
  ${c.prefix}Cubit()
      : actions = ${c.prefix}Actions._(_noopDispatch),
        super(OxideView(
          state: null,
          actions: ${c.prefix}Actions._(_noopDispatch),
          isLoading: true,
          error: null,
        )) {
    actions._bind(_dispatch);
    _subscription = _core.snapshots.listen((_) => _emit());
    unawaited(_initialize());
  }

  final ${c.prefix}Actions actions;
  StreamSubscription<${c.snapshotType}>? _subscription;

$coreInstantiation

  static Future<void> _noopDispatch(${c.actionsType} _) async {}

  Future<void> _initialize() async {
    await _core.initialize();
    _emit();
  }

  void _emit() {
    emit(OxideView(
      state: _core.state,
      actions: actions,
      isLoading: _core.isLoading,
      error: _core.error,
    ));
  }

  Future<void> _dispatch(${c.actionsType} action) async {
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

class ${c.prefix}Scope extends StatefulWidget {
  const ${c.prefix}Scope({super.key, required this.child, this.cubit});

  final Widget child;
  final ${c.prefix}Cubit? cubit;

  static ${c.prefix}Cubit cubitOf(BuildContext context) {
    return BlocProvider.of<${c.prefix}Cubit>(context);
  }

  @override
  State<${c.prefix}Scope> createState() => _${c.prefix}ScopeState();
}

class _${c.prefix}ScopeState extends State<${c.prefix}Scope>
    with AutomaticKeepAliveClientMixin {
  late final ${c.prefix}Cubit _cubit;
  late final bool _ownsCubit;

  @override
  bool get wantKeepAlive => ${c.keepAlive};

  @override
  void initState() {
    super.initState();
    _ownsCubit = widget.cubit == null;
    _cubit = widget.cubit ?? ${c.prefix}Cubit();
  }

  @override
  void dispose() {
    if (_ownsCubit) unawaited(_cubit.close());
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    super.build(context);
    return BlocProvider.value(
      value: _cubit,
      child: widget.child,
    );
  }
}
''';

  final backendCode = switch (c.backend) {
    'inherited' => inheritedBackend,
    'inheritedHooks' => inheritedHooksBackend,
    'riverpod' => riverpodBackend,
    'bloc' => blocBackend,
    _ => inheritedBackend,
  };

  return '''
$actionsClass

$backendCode
''';
}

String _lowerFirst(String value) {
  if (value.isEmpty) return value;
  return value[0].toLowerCase() + value.substring(1);
}

String _generateActionsMethods({required String actionsType, required bool actionsIsEnum, required List<OxideActionConstructor> constructors}) {
  final buffer = StringBuffer();
  for (final ctor in constructors) {
    if (ctor.name.isEmpty) continue;

    final signature = StringBuffer();
    signature.write('  Future<void> ${ctor.name}(');

    if (ctor.positionalParams.isNotEmpty) {
      signature.write(ctor.positionalParams.map(_formatPositional).join(', '));
    }
    if (ctor.namedParams.isNotEmpty) {
      if (ctor.positionalParams.isNotEmpty) signature.write(', ');
      signature.write('{');
      signature.write(ctor.namedParams.map(_formatNamed).join(', '));
      signature.write('}');
    }
    signature.write(') async {\n');

    final ctorArgs = StringBuffer();
    if (ctor.positionalParams.isNotEmpty) {
      ctorArgs.write(ctor.positionalParams.map((p) => p.name).join(', '));
    }
    if (ctor.namedParams.isNotEmpty) {
      if (ctor.positionalParams.isNotEmpty) ctorArgs.write(', ');
      ctorArgs.write(ctor.namedParams.map((p) => '${p.name}: ${p.name}').join(', '));
    }

    buffer.write(signature);
    if (actionsIsEnum) {
      buffer.write('    await _dispatch($actionsType.${ctor.name});\n');
    } else {
      buffer.write('    await _dispatch($actionsType.${ctor.name}($ctorArgs));\n');
    }
    buffer.write('  }\n\n');
  }

  return buffer.toString().trimRight();
}

String _formatPositional(OxideActionParam p) => '${p.type} ${p.name}';

String _formatNamed(OxideActionParam p) {
  if (p.isRequiredNamed) return 'required ${p.type} ${p.name}';
  return '${p.type} ${p.name}';
}
