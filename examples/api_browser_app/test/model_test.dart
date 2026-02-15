import 'package:flutter_test/flutter_test.dart';
import 'dart:convert';

void main() {
  test('Sample API fixtures are valid JSON and stable shape', () {
    const usersJson =
        '[{"id":1,"name":"Leanne Graham","username":"Bret"},{"id":2,"name":"Ervin Howell","username":"Antonette"}]';
    const postsJson =
        '[{"id":10,"userId":1,"title":"hello world"},{"id":11,"userId":1,"title":"second post"}]';
    const commentsJson =
        '[{"id":100,"postId":10,"name":"comment one"},{"id":101,"postId":10,"name":"comment two"}]';

    final users = jsonDecode(usersJson) as List<dynamic>;
    expect(users, hasLength(2));
    expect(users.first, containsPair('id', 1));
    expect(users.first, containsPair('username', 'Bret'));

    final posts = jsonDecode(postsJson) as List<dynamic>;
    expect(posts, hasLength(2));
    expect(posts.first, containsPair('userId', 1));
    expect(posts.first, containsPair('title', 'hello world'));

    final comments = jsonDecode(commentsJson) as List<dynamic>;
    expect(comments, hasLength(2));
    expect(comments.first, containsPair('postId', 10));
    expect(comments.first, containsPair('name', 'comment one'));
  });
}
