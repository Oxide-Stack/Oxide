import 'package:flutter/material.dart';

import '../../oxide_generated/routes/route_models.g.dart';

final class ApiBrowserUserDetailScreen extends StatelessWidget {
  const ApiBrowserUserDetailScreen({super.key, required this.route});

  final UserDetailRoute route;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: Text('User ${route.userId}')),
      body: Center(child: Text('User detail: ${route.userId}')),
    );
  }
}

