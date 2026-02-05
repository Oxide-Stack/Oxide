// GENERATED CODE - DO NOT MODIFY BY HAND
// dart format width=80

part of 'oxide.dart';

// **************************************************************************
// OxideStoreGenerator
// **************************************************************************

class UsersRiverpodOxideActions {
  UsersRiverpodOxideActions._(
    Future<void> Function(UsersAction action) dispatch,
  ) : _dispatch = dispatch;

  Future<void> Function(UsersAction action) _dispatch;

  void _bind(Future<void> Function(UsersAction action) dispatch) {
    _dispatch = dispatch;
  }

  Future<void> refresh() async {
    await _dispatch(UsersAction.refresh());
  }

  Future<void> selectUser({required BigInt userId}) async {
    await _dispatch(UsersAction.selectUser(userId: userId));
  }
}

final usersRiverpodOxideProvider =
    NotifierProvider<
      UsersRiverpodOxideNotifier,
      OxideView<UsersState, UsersRiverpodOxideActions>
    >(() => UsersRiverpodOxideNotifier());

class UsersRiverpodOxideNotifier
    extends Notifier<OxideView<UsersState, UsersRiverpodOxideActions>> {
  UsersRiverpodOxideNotifier();

  late final UsersRiverpodOxideActions actions = UsersRiverpodOxideActions._(
    _dispatch,
  );
  StreamSubscription<UsersStateSnapshot>? _subscription;

  late final OxideStoreCore<
    UsersState,
    UsersAction,
    ArcUsersEngine,
    UsersStateSnapshot
  >
  _core =
      OxideStoreCore<
        UsersState,
        UsersAction,
        ArcUsersEngine,
        UsersStateSnapshot
      >(
        createEngine: (_) => users_api.createEngine(),
        disposeEngine: (engine) => users_api.disposeEngine(engine: engine),
        dispatch: (engine, action) =>
            users_api.dispatch(engine: engine, action: action),
        current: (engine) => users_api.current(engine: engine),
        stateStream: (engine) => users_api.stateStream(engine: engine),
        stateFromSnapshot: (snap) => snap.state,
      );

  @override
  OxideView<UsersState, UsersRiverpodOxideActions> build() {
    actions._bind(_dispatch);
    ref.onDispose(() {
      _subscription?.cancel();
      unawaited(_core.dispose());
    });

    unawaited(_initialize());
    return OxideView(
      state: null,
      actions: actions,
      isLoading: true,
      error: null,
    );
  }

  Future<void> _initialize() async {
    await _core.initialize();
    _subscription = _core.snapshots.listen((_) => state = _view());
    state = _view();
  }

  OxideView<UsersState, UsersRiverpodOxideActions> _view() {
    return OxideView(
      state: _core.state,
      actions: actions,
      isLoading: _core.isLoading,
      error: _core.error,
    );
  }

  Future<void> _dispatch(UsersAction action) async {
    await _core.dispatchAction(action);
    state = _view();
  }
}

class PostsRiverpodOxideActions {
  PostsRiverpodOxideActions._(
    Future<void> Function(PostsAction action) dispatch,
  ) : _dispatch = dispatch;

  Future<void> Function(PostsAction action) _dispatch;

  void _bind(Future<void> Function(PostsAction action) dispatch) {
    _dispatch = dispatch;
  }

  Future<void> loadForUser({required BigInt userId}) async {
    await _dispatch(PostsAction.loadForUser(userId: userId));
  }

  Future<void> refresh() async {
    await _dispatch(PostsAction.refresh());
  }

  Future<void> selectPost({required BigInt postId}) async {
    await _dispatch(PostsAction.selectPost(postId: postId));
  }
}

final postsRiverpodOxideProvider =
    NotifierProvider<
      PostsRiverpodOxideNotifier,
      OxideView<PostsState, PostsRiverpodOxideActions>
    >(() => PostsRiverpodOxideNotifier());

class PostsRiverpodOxideNotifier
    extends Notifier<OxideView<PostsState, PostsRiverpodOxideActions>> {
  PostsRiverpodOxideNotifier();

  late final PostsRiverpodOxideActions actions = PostsRiverpodOxideActions._(
    _dispatch,
  );
  StreamSubscription<PostsStateSnapshot>? _subscription;

  late final OxideStoreCore<
    PostsState,
    PostsAction,
    ArcPostsEngine,
    PostsStateSnapshot
  >
  _core =
      OxideStoreCore<
        PostsState,
        PostsAction,
        ArcPostsEngine,
        PostsStateSnapshot
      >(
        createEngine: (_) => posts_api.createEngine(),
        disposeEngine: (engine) => posts_api.disposeEngine(engine: engine),
        dispatch: (engine, action) =>
            posts_api.dispatch(engine: engine, action: action),
        current: (engine) => posts_api.current(engine: engine),
        stateStream: (engine) => posts_api.stateStream(engine: engine),
        stateFromSnapshot: (snap) => snap.state,
      );

  @override
  OxideView<PostsState, PostsRiverpodOxideActions> build() {
    actions._bind(_dispatch);
    ref.onDispose(() {
      _subscription?.cancel();
      unawaited(_core.dispose());
    });

    unawaited(_initialize());
    return OxideView(
      state: null,
      actions: actions,
      isLoading: true,
      error: null,
    );
  }

  Future<void> _initialize() async {
    await _core.initialize();
    _subscription = _core.snapshots.listen((_) => state = _view());
    state = _view();
  }

  OxideView<PostsState, PostsRiverpodOxideActions> _view() {
    return OxideView(
      state: _core.state,
      actions: actions,
      isLoading: _core.isLoading,
      error: _core.error,
    );
  }

  Future<void> _dispatch(PostsAction action) async {
    await _core.dispatchAction(action);
    state = _view();
  }
}

class CommentsRiverpodOxideActions {
  CommentsRiverpodOxideActions._(
    Future<void> Function(CommentsAction action) dispatch,
  ) : _dispatch = dispatch;

  Future<void> Function(CommentsAction action) _dispatch;

  void _bind(Future<void> Function(CommentsAction action) dispatch) {
    _dispatch = dispatch;
  }

  Future<void> loadForPost({required BigInt postId}) async {
    await _dispatch(CommentsAction.loadForPost(postId: postId));
  }

  Future<void> refresh() async {
    await _dispatch(CommentsAction.refresh());
  }
}

final commentsRiverpodOxideProvider =
    NotifierProvider<
      CommentsRiverpodOxideNotifier,
      OxideView<CommentsState, CommentsRiverpodOxideActions>
    >(() => CommentsRiverpodOxideNotifier());

class CommentsRiverpodOxideNotifier
    extends Notifier<OxideView<CommentsState, CommentsRiverpodOxideActions>> {
  CommentsRiverpodOxideNotifier();

  late final CommentsRiverpodOxideActions actions =
      CommentsRiverpodOxideActions._(_dispatch);
  StreamSubscription<CommentsStateSnapshot>? _subscription;

  late final OxideStoreCore<
    CommentsState,
    CommentsAction,
    ArcCommentsEngine,
    CommentsStateSnapshot
  >
  _core =
      OxideStoreCore<
        CommentsState,
        CommentsAction,
        ArcCommentsEngine,
        CommentsStateSnapshot
      >(
        createEngine: (_) => comments_api.createEngine(),
        disposeEngine: (engine) => comments_api.disposeEngine(engine: engine),
        dispatch: (engine, action) =>
            comments_api.dispatch(engine: engine, action: action),
        current: (engine) => comments_api.current(engine: engine),
        stateStream: (engine) => comments_api.stateStream(engine: engine),
        stateFromSnapshot: (snap) => snap.state,
      );

  @override
  OxideView<CommentsState, CommentsRiverpodOxideActions> build() {
    actions._bind(_dispatch);
    ref.onDispose(() {
      _subscription?.cancel();
      unawaited(_core.dispose());
    });

    unawaited(_initialize());
    return OxideView(
      state: null,
      actions: actions,
      isLoading: true,
      error: null,
    );
  }

  Future<void> _initialize() async {
    await _core.initialize();
    _subscription = _core.snapshots.listen((_) => state = _view());
    state = _view();
  }

  OxideView<CommentsState, CommentsRiverpodOxideActions> _view() {
    return OxideView(
      state: _core.state,
      actions: actions,
      isLoading: _core.isLoading,
      error: _core.error,
    );
  }

  Future<void> _dispatch(CommentsAction action) async {
    await _core.dispatchAction(action);
    state = _view();
  }
}
