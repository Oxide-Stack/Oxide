import 'model.dart';

// InheritedWidget/ChangeNotifier backend templates.
//
// Why: this backend provides a dependency-free integration for Flutter apps
// that don't want an additional state management dependency.
String buildInheritedBackend(OxideCodegenConfig c, String coreInstantiation) {
  return '''
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
}
