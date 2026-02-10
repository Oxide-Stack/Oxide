import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'oxide_generated/navigation/navigation_runtime.g.dart';
import 'src/app.dart';
import 'src/rust/api/bridge.dart' as api;
import 'src/rust/frb_generated.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await RustLib.init();
  await api.initOxide();
  oxideNavStart();
  api.resetApiBaseUrl();

  runApp(const ProviderScope(child: ApiBrowserApp()));
}
