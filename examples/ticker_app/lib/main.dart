import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:oxide_runtime/oxide_runtime.dart';

import 'oxide.dart';
import 'src/app/app.dart';
import 'src/rust/api/bridge.dart';

Future<void> main() async {
  await OxideStack.init();
  OxideLogger.attachRustStream(setupRustLogs());
  
  await runOxideApp(const ProviderScope(child: MyApp()));
}
