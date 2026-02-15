import 'model.dart';

// flutter_bloc Cubit backend templates.
//
// Why: BLoC-style apps integrate naturally with a Cubit that owns the Oxide core
// and emits a derived `OxideView` state for widgets.
String buildBlocBackend(OxideCodegenConfig c, String coreInstantiation) {
  final snapshotsStream = (c.slices == null || c.slices!.isEmpty)
      ? '_core.snapshots'
      : 'filterSnapshotsBySlices<${c.snapshotType}, ${c.sliceType!}>('
          '_core.snapshots, '
          'const [${c.slices!.join(', ')}], '
          '(snap) => snap.slices'
          ')';
  return '''
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
    _subscription = $snapshotsStream.listen((_) => _emit());
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
}
