import 'dart:async';
import 'dart:io';

import 'package:integration_test/integration_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';

import 'package:api_browser_app/src/app.dart';
import 'package:api_browser_app/src/rust/api/bridge.dart' as api;
import 'package:api_browser_app/src/rust/frb_generated.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();
  setUpAll(() async {
    await RustLib.init();
    await api.initOxide();
  });

  testWidgets('Loads users, posts, and comments via local API server', (WidgetTester tester) async {
    const usersJson =
        '[{"id":1,"name":"Leanne Graham","username":"Bret"},{"id":2,"name":"Ervin Howell","username":"Antonette"}]';
    const postsUser1Json =
        '[{"id":10,"userId":1,"title":"hello world"},{"id":11,"userId":1,"title":"second post"}]';
    const postsUser2Json =
        '[{"id":20,"userId":2,"title":"user two post"}]';
    const commentsPost10Json =
        '[{"id":100,"postId":10,"name":"comment one"},{"id":101,"postId":10,"name":"comment two"}]';
    const commentsPost20Json = '[{"id":200,"postId":20,"name":"user two comment"}]';

    final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
    unawaited(() async {
      await for (final request in server) {
        final path = request.uri.path;
        if (path == '/users') {
          request.response
            ..statusCode = 200
            ..headers.contentType = ContentType.json
            ..write(usersJson);
        } else if (path == '/posts') {
          final userId = request.uri.queryParameters['userId'];
          request.response
            ..statusCode = 200
            ..headers.contentType = ContentType.json
            ..write(userId == '2' ? postsUser2Json : postsUser1Json);
        } else if (path == '/comments') {
          final postId = request.uri.queryParameters['postId'];
          request.response
            ..statusCode = 200
            ..headers.contentType = ContentType.json
            ..write(postId == '20' ? commentsPost20Json : commentsPost10Json);
        } else {
          request.response.statusCode = 404;
        }
        await request.response.close();
      }
    }());

    api.setApiBaseUrl(url: 'http://127.0.0.1:${server.port}');

    await tester.pumpWidget(const ProviderScope(child: ApiBrowserApp()));
    await tester.pumpAndSettle(const Duration(seconds: 5));

    expect(find.text('Leanne Graham'), findsOneWidget);
    expect(find.text('hello world'), findsOneWidget);
    expect(find.text('comment one'), findsOneWidget);

    await tester.tap(find.text('Ervin Howell'));
    await tester.pumpAndSettle(const Duration(seconds: 5));

    expect(find.text('user two post'), findsOneWidget);
    expect(find.text('user two comment'), findsOneWidget);

    await server.close(force: true);
  });
}
