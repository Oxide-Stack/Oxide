import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'oxide.dart';
import 'src/app_shell.dart';
// bridge import removed; use helpers exposed by `oxide.dart`

Future<void> main() async {
  // keep custom network configuration separate from boilerplate
  // initialize network configuration via public API helper
  resetApiBaseUrl();
  await runOxideApp(const ProviderScope(child: ApiBrowserApp()));
}
