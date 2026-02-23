import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'oxide.dart';
import 'src/app_shell.dart';
import 'src/rust/api/bridge.dart' as api;

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await OxideStack.init();
  api.resetApiBaseUrl();

  runApp(const ProviderScope(child: ApiBrowserApp()));
}
