import 'oxide.dart';
import 'src/app.dart';

Future<void> main() async {
  // Oxide + FRB initialization and binding boilerplate is handled by runOxideApp.
  await runOxideApp(const MyApp());
}
