import 'package:oxide_generator/src/oxide_store_codegen.dart';
import 'package:test/test.dart';

void main() {
  test('Enum actions generate dispatch using enum constant', () {
    final src = generateOxideStoreSource(
      OxideCodegenConfig(
        prefix: 'Counter',
        stateType: 'AppState',
        snapshotType: 'AppSnapshot',
        actionsType: 'AppAction',
        actionsIsEnum: true,
        engineType: 'AppEngine',
        backend: 'inherited',
        createEngine: 'createEngine',
        disposeEngine: 'disposeEngine',
        dispatch: 'dispatch',
        stateStream: 'stateStream',
        current: 'current',
        initApp: 'initApp',
        encodeCurrentState: null,
        encodeState: null,
        decodeState: null,
        actionConstructors: const [OxideActionConstructor(name: 'increment', positionalParams: [], namedParams: [])],
      ),
    );

    expect(src, contains('await _dispatch(AppAction.increment);'));
    expect(src, isNot(contains('AppAction.increment(')));
  });

  test('Class actions generate dispatch using factory constructor', () {
    final src = generateOxideStoreSource(
      OxideCodegenConfig(
        prefix: 'Todos',
        stateType: 'AppState',
        snapshotType: 'AppSnapshot',
        actionsType: 'AppAction',
        actionsIsEnum: false,
        engineType: 'AppEngine',
        backend: 'inherited',
        createEngine: 'createEngine',
        disposeEngine: 'disposeEngine',
        dispatch: 'dispatch',
        stateStream: 'stateStream',
        current: 'current',
        initApp: null,
        encodeCurrentState: null,
        encodeState: null,
        decodeState: null,
        actionConstructors: const [
          OxideActionConstructor(
            name: 'addTodo',
            positionalParams: [],
            namedParams: [OxideActionParam(name: 'title', type: 'String', isRequiredNamed: true)],
          ),
        ],
      ),
    );

    expect(src, contains('Future<void> addTodo({required String title})'));
    expect(src, contains('await _dispatch(AppAction.addTodo(title: title));'));
  });
}
