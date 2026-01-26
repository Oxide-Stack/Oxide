import 'package:flutter_test/flutter_test.dart';
import 'package:counter_app/src/rust/state/app_action.dart';
import 'package:counter_app/src/rust/state/app_state.dart';

void main() {
  test('Generated FRB models are usable in Dart', () {
    const action = AppAction.increment;
    expect(action, AppAction.increment);

    final state = AppState(counter: BigInt.from(0));
    expect(state.counter, BigInt.from(0));
  });
}
