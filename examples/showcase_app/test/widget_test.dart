import 'package:flutter_test/flutter_test.dart';
import 'package:showcase_app/src/rust/api/settings/actions.dart';
import 'package:showcase_app/src/rust/api/settings/state.dart';
import 'package:showcase_app/src/rust/config/enums/theme_type.dart';

void main() {
  test('Generated FRB models are usable in Dart', () {
    const action = SettingsAction.openSettings;
    expect(action, SettingsAction.openSettings);

    final state = SettingsState(theme: Theme.light, mainColor: '#FFFFFF');
    expect(state.theme, Theme.light);
    expect(state.mainColor, '#FFFFFF');
  });
}
