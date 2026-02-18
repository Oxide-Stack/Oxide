import 'dart:async';

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:oxide_runtime/oxide_runtime.dart';

import 'oxide.dart';
import 'rust/state/comments_state.dart';
import 'rust/state/posts_state.dart';
import 'rust/state/users_state.dart';

class ApiBrowserCoordinator extends ConsumerStatefulWidget {
  const ApiBrowserCoordinator({required this.child, super.key});

  final Widget child;

  @override
  ConsumerState<ApiBrowserCoordinator> createState() => _ApiBrowserCoordinatorState();
}

class _ApiBrowserCoordinatorState extends ConsumerState<ApiBrowserCoordinator> {
  BigInt? _selectedUserId;
  BigInt? _selectedPostId;
  ProviderSubscription<OxideView<UsersState, UsersRiverpodOxideActions>>? _usersSubscription;
  ProviderSubscription<OxideView<PostsState, PostsRiverpodOxideActions>>? _postsSubscription;

  @override
  void initState() {
    super.initState();

    _usersSubscription = ref.listenManual(usersRiverpodOxideProvider, (previous, next) {
      final userId = next.state?.selectedUserId;
      if (userId == null || userId == _selectedUserId) return;
      _selectedUserId = userId;
      unawaited(ref.read(postsRiverpodOxideProvider).actions.loadForUser(userId: userId));
    });

    _postsSubscription = ref.listenManual(postsRiverpodOxideProvider, (previous, next) {
      final postId = next.state?.selectedPostId;
      if (postId == null || postId == _selectedPostId) return;
      _selectedPostId = postId;
      unawaited(ref.read(commentsRiverpodOxideProvider).actions.loadForPost(postId: postId));
    });
  }

  @override
  void dispose() {
    _usersSubscription?.close();
    _postsSubscription?.close();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) => widget.child;
}

class ApiBrowserView extends ConsumerWidget {
  const ApiBrowserView({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final users = ref.watch(usersRiverpodOxideProvider);
    final posts = ref.watch(postsRiverpodOxideProvider);
    final comments = ref.watch(commentsRiverpodOxideProvider);

    return SafeArea(
      child: Row(
        children: [
          Expanded(
            child: _UsersPane(view: users, onRefresh: users.actions.refresh, onSelectUser: users.actions.selectUser),
          ),
          const VerticalDivider(width: 1),
          Expanded(
            child: _PostsPane(view: posts, onRefresh: posts.actions.refresh, onSelectPost: posts.actions.selectPost),
          ),
          const VerticalDivider(width: 1),
          Expanded(
            child: _CommentsPane(view: comments, onRefresh: comments.actions.refresh),
          ),
        ],
      ),
    );
  }
}

class _UsersPane extends StatelessWidget {
  const _UsersPane({required this.view, required this.onRefresh, required this.onSelectUser});

  final OxideView<UsersState, UsersRiverpodOxideActions> view;
  final Future<void> Function() onRefresh;
  final Future<void> Function({required BigInt userId}) onSelectUser;

  @override
  Widget build(BuildContext context) {
    return _PaneScaffold(
      title: 'Users',
      isLoading: view.isLoading,
      error: view.error,
      onRefresh: onRefresh,
      child: view.state == null
          ? const SizedBox.shrink()
          : ListView.builder(
              itemCount: view.state!.users.length,
              itemBuilder: (context, index) {
                final user = view.state!.users[index];
                final selected = view.state!.selectedUserId == user.id;
                return ListTile(
                  title: Text(user.name),
                  subtitle: Text('@${user.username}'),
                  selected: selected,
                  onTap: () {
                    unawaited(onSelectUser(userId: user.id));
                  },
                );
              },
            ),
    );
  }
}

class _PostsPane extends StatelessWidget {
  const _PostsPane({required this.view, required this.onRefresh, required this.onSelectPost});

  final OxideView<PostsState, PostsRiverpodOxideActions> view;
  final Future<void> Function() onRefresh;
  final Future<void> Function({required BigInt postId}) onSelectPost;

  @override
  Widget build(BuildContext context) {
    return _PaneScaffold(
      title: 'Posts',
      isLoading: view.isLoading,
      error: view.error,
      onRefresh: onRefresh,
      child: view.state == null
          ? const Center(child: Text('Select a user'))
          : ListView.builder(
              itemCount: view.state!.posts.length,
              itemBuilder: (context, index) {
                final post = view.state!.posts[index];
                final selected = view.state!.selectedPostId == post.id;
                return ListTile(
                  title: Text(post.title),
                  selected: selected,
                  onTap: () => unawaited(onSelectPost(postId: post.id)),
                );
              },
            ),
    );
  }
}

class _CommentsPane extends StatelessWidget {
  const _CommentsPane({required this.view, required this.onRefresh});

  final OxideView<CommentsState, CommentsRiverpodOxideActions> view;
  final Future<void> Function() onRefresh;

  @override
  Widget build(BuildContext context) {
    return _PaneScaffold(
      title: 'Comments',
      isLoading: view.isLoading,
      error: view.error,
      onRefresh: onRefresh,
      child: view.state == null
          ? const Center(child: Text('Select a post'))
          : ListView.builder(
              itemCount: view.state!.comments.length,
              itemBuilder: (context, index) {
                final comment = view.state!.comments[index];
                return ListTile(title: Text(comment.name));
              },
            ),
    );
  }
}

class _PaneScaffold extends StatelessWidget {
  const _PaneScaffold({required this.title, required this.child, required this.isLoading, required this.error, required this.onRefresh});

  final String title;
  final Widget child;
  final bool isLoading;
  final Object? error;
  final Future<void> Function() onRefresh;

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        Material(
          color: Theme.of(context).colorScheme.surfaceContainerHighest,
          child: Padding(
            padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
            child: Row(
              children: [
                Text(title, style: Theme.of(context).textTheme.titleMedium),
                const Spacer(),
                if (isLoading) const SizedBox(width: 18, height: 18, child: CircularProgressIndicator(strokeWidth: 2)),
                const SizedBox(width: 8),
                IconButton(onPressed: () => unawaited(onRefresh()), icon: const Icon(Icons.refresh), tooltip: 'Refresh'),
              ],
            ),
          ),
        ),
        if (error != null)
          Material(
            color: Theme.of(context).colorScheme.errorContainer,
            child: Padding(
              padding: const EdgeInsets.all(8),
              child: Text(error.toString(), style: TextStyle(color: Theme.of(context).colorScheme.onErrorContainer)),
            ),
          ),
        Expanded(child: child),
      ],
    );
  }
}
