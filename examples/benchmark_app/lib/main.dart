import 'package:flutter/widgets.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'oxide.dart';
import 'src/bench/bench_app.dart';

Future<void> main() async {
  // start without kicking off navigation immediately so we can clear any
  // persisted state that might refer to our charts route. leftover history
  // from previous runs was causing the benchmark to land on `/charts` at
  // startup with no args, which then triggered the bounce-to-root logic and
  // produced the flicker the user reported.
  await runOxideApp(
    const ProviderScope(
      child: BenchCounterHooksOxideScope(
        child: BenchJsonHooksOxideScope(
          child: BenchSieveHooksOxideScope(child: BenchApp()),
        ),
      ),
    ),
  );

  // start the navigation runtime manually and then immediately wipe any
  // state that Rust may have restored. we use a second post-frame callback to
  // give the runtime a chance to process its initial command stream before we
  // override it; without this, a pre-existing `/charts` reset would arrive
  // after our clear and reintroduce the bad state.
  WidgetsBinding.instance.addPostFrameCallback((_) {
    // start the runtime via the public API instead of calling the generated
    // helper directly; this avoids the need to import the generated file.
    OxideStack.navigation.start();
    WidgetsBinding.instance.addPostFrameCallback((_) {
      // debug log so we can spot when the cleanup occurs
      // ignore: avoid_print
      print('[Bench] clearing restored navigation state');
      // reset to home route explicitly, GoRouter handler ignores empty lists
      OxideStack.navigation.handler.reset([HomeRoute()]);
    });
  });
}
