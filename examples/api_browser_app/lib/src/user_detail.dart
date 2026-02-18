import 'package:flutter/material.dart';

final class UserDetailScreen extends StatelessWidget {
  const UserDetailScreen({super.key, required this.userId});

  final int userId;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('User $userId'),
        leading: IconButton(
          icon: const Icon(Icons.arrow_back),
          onPressed: () => Navigator.of(context).maybePop(),
        ),
      ),
      body: Center(child: Text('Deep link param userId=$userId')),
    );
  }
}

