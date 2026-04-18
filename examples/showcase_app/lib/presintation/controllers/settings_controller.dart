import 'dart:async';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:oxide_runtime/oxide_runtime.dart';
import 'package:showcase_app/src/rust/api/settings/actions.dart'
    show SettingsAction;
import 'package:showcase_app/src/rust/api/settings/reducer.dart'
    show
        SettingsStateSnapshot,
        ArcSettingsEngine,
        createEngine,
        disposeEngine,
        dispatch,
        current,
        stateStream;
import 'package:showcase_app/src/rust/api/settings/state.dart'
    show SettingsState;
import 'package:showcase_app/src/rust/config/enums/theme_type.dart';
part 'settings_controller.oxide.g.dart';

@OxideStore(
  state: SettingsState,
  snapshot: SettingsStateSnapshot,
  actions: SettingsAction,
  engine: ArcSettingsEngine,
  backend: OxideBackend.riverpod,
  keepAlive: true,
)
class SettingsController {}
