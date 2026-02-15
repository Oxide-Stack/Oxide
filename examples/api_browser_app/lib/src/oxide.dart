import 'dart:async';

import 'package:flutter/widgets.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:oxide_annotations/oxide_annotations.dart';
import 'package:oxide_runtime/oxide_runtime.dart';

import 'rust/api/comments_bridge.dart' as comments_api;
import 'rust/api/comments_bridge.dart' show ArcCommentsEngine, CommentsStateSnapshot;
import 'rust/api/posts_bridge.dart' as posts_api;
import 'rust/api/posts_bridge.dart' show ArcPostsEngine, PostsStateSnapshot;
import 'rust/api/users_bridge.dart' as users_api;
import 'rust/api/users_bridge.dart' show ArcUsersEngine, UsersStateSnapshot;
import 'rust/state/comments_action.dart';
import 'rust/state/comments_state.dart';
import 'rust/state/posts_action.dart';
import 'rust/state/posts_state.dart';
import 'rust/state/users_action.dart';
import 'rust/state/users_state.dart';

part 'oxide.oxide.g.dart';

@OxideStore(
  state: UsersState,
  snapshot: UsersStateSnapshot,
  actions: UsersAction,
  engine: ArcUsersEngine,
  backend: OxideBackend.riverpod,
  bindings: 'users_api',
  keepAlive: true,
)
class UsersRiverpodOxide {}

@OxideStore(
  state: PostsState,
  snapshot: PostsStateSnapshot,
  actions: PostsAction,
  engine: ArcPostsEngine,
  backend: OxideBackend.riverpod,
  bindings: 'posts_api',
  keepAlive: true,
)
class PostsRiverpodOxide {}

@OxideStore(
  state: CommentsState,
  snapshot: CommentsStateSnapshot,
  actions: CommentsAction,
  engine: ArcCommentsEngine,
  backend: OxideBackend.riverpod,
  bindings: 'comments_api',
  keepAlive: true,
)
class CommentsRiverpodOxide {}
