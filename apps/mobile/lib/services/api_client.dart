import "dart:convert";
import "dart:io";

import "session.dart";

class ApiException implements Exception {
  final String message;
  ApiException(this.message);

  @override
  String toString() => message;
}

class ApiClient {
  ApiClient._();

  static String get baseUrl {
    const env = String.fromEnvironment("API_BASE_URL", defaultValue: "");
    if (env.isNotEmpty) {
      return env;
    }
    if (Platform.isAndroid) {
      return "http://10.0.2.2:8080/api/v1";
    }
    return "http://127.0.0.1:8080/api/v1";
  }

  static final HttpClient _client = HttpClient();

  static Future<dynamic> get(String path) {
    return _request("GET", path);
  }

  static Future<dynamic> post(String path, Map<String, dynamic> body) {
    return _request("POST", path, body: body);
  }

  static Future<dynamic> put(String path, Map<String, dynamic> body) {
    return _request("PUT", path, body: body);
  }

  static Future<dynamic> delete(String path) {
    return _request("DELETE", path);
  }

  static Future<dynamic> _request(String method, String path, {Map<String, dynamic>? body}) async {
    final uri = Uri.parse("$baseUrl$path");
    final request = await _client.openUrl(method, uri);
    request.headers.contentType = ContentType.json;
    final token = SessionStore.token;
    if (token != null && token.isNotEmpty) {
      request.headers.set(HttpHeaders.authorizationHeader, "Bearer $token");
    }
    if (body != null) {
      request.write(jsonEncode(body));
    }

    final response = await request.close();
    if (response.statusCode == HttpStatus.unauthorized ||
        response.statusCode == HttpStatus.forbidden) {
      await SessionStore.clear();
    }
    final payload = await response.transform(utf8.decoder).join();
    if (payload.isEmpty) {
      throw ApiException("empty_response");
    }

    final decoded = jsonDecode(payload) as Map<String, dynamic>;
    final code = decoded["code"] as int? ?? 0;
    if (code != 0) {
      throw ApiException(decoded["message"] as String? ?? "request_failed");
    }
    return decoded["data"];
  }
}
