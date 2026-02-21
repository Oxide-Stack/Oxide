import 'package:flutter_test/flutter_test.dart';
import 'package:todos_app/src/rust/state/app_action.dart';
import 'package:todos_app/src/rust/state/app_state.dart';

void main() {
  test('Generated FRB models are usable in Dart', () {
    final action = AppAction.addTodo(title: 'buy milk');
    final title = action.maybeWhen(addTodo: (t) => t, orElse: () => '');
    expect(title, 'buy milk');

    final state = AppState(todos: const [], nextId: BigInt.from(1));
    expect(state.todos, isEmpty);
  });
}
