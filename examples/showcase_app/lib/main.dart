import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:showcase_app/app.dart';
import 'package:showcase_app/oxide.dart';

Future<void> main() async {
  await runOxideApp(const ProviderScope(child: MyApp()));
}
