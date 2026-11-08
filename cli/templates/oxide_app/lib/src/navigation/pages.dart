import 'package:flutter/widgets.dart';
import 'package:oxide_runtime/oxide_runtime.dart';

import '../../oxide.dart';
import '../counter_page.dart';

/// Route bindings for this app.
///
/// The Rust `#[oxide_route]` declarations in `rust/src/routes/` are turned into
/// the generated `RouteKind` enum. Every kind needs exactly one `@OxideRoutePage`
/// binding here, otherwise the generated route table throws
/// `UnimplementedError` when that route is pushed.
///
/// Add a route with `oxide generate route <Kind>`, which creates the Rust
/// struct, registers it, and adds the matching binding below.
@OxideRoutePage(RouteKind.home)
final class HomePage extends StatelessWidget {
  const HomePage({super.key, required this.route});

  final HomeRoute route;

  @override
  Widget build(BuildContext context) {
    return const {{page_class}}();
  }
}
