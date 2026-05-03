import 'dart:async';
import 'package:flutter/widgets.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
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
    show SettingsState, SettingsStateSlice;
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

@OxideStore(
  state: SettingsState,
  snapshot: SettingsStateSnapshot,
  actions: SettingsAction,
  engine: ArcSettingsEngine,
  backend: OxideBackend.riverpod,
  keepAlive: true,
  slices: [SettingsStateSlice.theme],
)
class SettingsThemeSliceController {}

@OxideStore(
  state: SettingsState,
  snapshot: SettingsStateSnapshot,
  actions: SettingsAction,
  engine: ArcSettingsEngine,
  backend: OxideBackend.riverpod,
  keepAlive: true,
  slices: [SettingsStateSlice.mainColor],
)
class SettingsColorSliceController {}

@OxideStore(
  state: SettingsState,
  snapshot: SettingsStateSnapshot,
  actions: SettingsAction,
  engine: ArcSettingsEngine,
  backend: OxideBackend.inherited,
  keepAlive: true,
  name: 'SettingsInheritedBridge',
)
class SettingsInheritedController {}

@OxideStore(
  state: SettingsState,
  snapshot: SettingsStateSnapshot,
  actions: SettingsAction,
  engine: ArcSettingsEngine,
  backend: OxideBackend.inheritedHooks,
  keepAlive: true,
  name: 'SettingsHooksBridge',
)
class SettingsHooksController {}

@OxideStore(
  state: SettingsState,
  snapshot: SettingsStateSnapshot,
  actions: SettingsAction,
  engine: ArcSettingsEngine,
  backend: OxideBackend.bloc,
  keepAlive: true,
  name: 'SettingsBlocBridge',
)
class SettingsBlocController {}
