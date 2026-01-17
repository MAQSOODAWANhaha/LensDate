import "dart:convert";
import "dart:io";

import "package:flutter_image_compress/flutter_image_compress.dart";
import "package:http/http.dart" as http;
import "package:image_picker/image_picker.dart";
import "package:path/path.dart" as p;

import "api_client.dart";
import "session.dart";

class UploadService {
  static final ImagePicker _picker = ImagePicker();

  static Future<String?> pickAndUpload(ImageSource source) async {
    final picked = await _picker.pickImage(source: source, imageQuality: 85);
    if (picked == null) {
      return null;
    }
    return uploadFile(File(picked.path));
  }

  static Future<String> uploadFile(File file) async {
    final compressed = await _maybeCompress(file);
    final uri = Uri.parse("${ApiClient.baseUrl}/uploads");
    final request = http.MultipartRequest("POST", uri);
    final token = SessionStore.token;
    if (token != null && token.isNotEmpty) {
      request.headers[HttpHeaders.authorizationHeader] = "Bearer $token";
    }

    request.files.add(await http.MultipartFile.fromPath(
      "file",
      compressed.path,
      filename: p.basename(compressed.path),
    ));

    final response = await request.send();
    final body = await response.stream.bytesToString();
    if (body.isEmpty) {
      throw Exception("empty_response");
    }
    final decoded = jsonDecode(body) as Map<String, dynamic>;
    final code = decoded["code"] as int? ?? 0;
    if (code != 0) {
      throw Exception(decoded["message"] ?? "upload_failed");
    }
    final data = decoded["data"] as Map<String, dynamic>? ?? {};
    final fileUrl = data["file_url"]?.toString() ?? "";
    if (fileUrl.isEmpty) {
      throw Exception("upload_failed");
    }
    return _resolveUrl(fileUrl);
  }

  static Future<File> _maybeCompress(File file) async {
    final ext = p.extension(file.path).toLowerCase();
    final isImage = [".jpg", ".jpeg", ".png", ".webp"].contains(ext);
    if (!isImage) {
      return file;
    }
    final target = p.join(
      Directory.systemTemp.path,
      "upload_${DateTime.now().millisecondsSinceEpoch}$ext",
    );
    final result = await FlutterImageCompress.compressAndGetFile(
      file.path,
      target,
      quality: 85,
      minWidth: 1600,
      minHeight: 1600,
    );
    if (result != null) {
      return File(result.path);
    }
    return file;
  }

  static String _resolveUrl(String fileUrl) {
    if (fileUrl.startsWith("http://") || fileUrl.startsWith("https://")) {
      return fileUrl;
    }
    if (fileUrl.startsWith("/")) {
      return "${ApiClient.baseUrl}$fileUrl";
    }
    return "${ApiClient.baseUrl}/$fileUrl";
  }
}
