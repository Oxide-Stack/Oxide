import 'package:flutter/widgets.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'src/bench/bench_screen.dart';
import 'src/oxide.dart';
import 'src/rust/frb_generated.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await RustLib.init();
  runApp(
    const ProviderScope(
      child: BenchCounterHooksOxideScope(
        child: BenchJsonHooksOxideScope(child: BenchSieveHooksOxideScope(child: BenchApp())),
      ),
    ),
  );
}
