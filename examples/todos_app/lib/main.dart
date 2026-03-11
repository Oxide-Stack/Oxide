import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'oxide.dart';
import 'src/app/app.dart';

Future<void> main() async {
  await runOxideApp(const ProviderScope(child: MyApp()));
}
