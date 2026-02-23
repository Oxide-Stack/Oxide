import 'package:flutter/widgets.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'oxide.dart';
import 'src/bench/bench_app.dart';
import 'src/oxide.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await OxideStack.init();
  runApp(
    const ProviderScope(
      child: BenchCounterHooksOxideScope(
        child: BenchJsonHooksOxideScope(child: BenchSieveHooksOxideScope(child: BenchApp())),
      ),
    ),
  );
}
