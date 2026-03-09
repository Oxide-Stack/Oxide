import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'oxide.dart';
import 'src/app_shell.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await OxideStack.init(startNavigation: false);
  await resetApiBaseUrl();
  await runOxideApp(const ProviderScope(child: ApiBrowserApp()));
}
