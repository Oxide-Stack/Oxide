import 'package:flutter/widgets.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'oxide_generated/navigation/navigation_runtime.g.dart';
import 'src/bench/bench_app.dart';
import 'src/oxide.dart';
import 'src/rust/api/bridge.dart' show initOxide;
import 'src/rust/frb_generated.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await RustLib.init();
  await initOxide();
  oxideNavStart();
  runApp(
    const ProviderScope(
      child: BenchCounterHooksOxideScope(
        child: BenchJsonHooksOxideScope(child: BenchSieveHooksOxideScope(child: BenchApp())),
      ),
    ),
  );
}
