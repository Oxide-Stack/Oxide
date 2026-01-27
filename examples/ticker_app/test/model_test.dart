import 'package:flutter_test/flutter_test.dart';
import 'package:ticker_app/src/rust/state/app_action.dart';
import 'package:ticker_app/src/rust/state/app_state.dart';

void main() {
  test('Generated FRB models are usable in Dart', () {
    const action = AppAction.manualTick();
    expect(action, const AppAction.manualTick());

    final state = AppState(ticks: BigInt.from(0), isRunning: false, intervalMs: BigInt.from(1000), lastTickSource: 'manual');
    expect(state.ticks, BigInt.from(0));
  });
}
