import "package:flutter/foundation.dart";
import "package:shared_preferences/shared_preferences.dart";

class SessionStore {
  static final ValueNotifier<bool> authed = ValueNotifier(false);
  static String? token;
  static SharedPreferences? _prefs;

  static Future<void> init() async {
    _prefs = await SharedPreferences.getInstance();
    token = _prefs?.getString("token");
    authed.value = token != null && token!.isNotEmpty;
  }

  static Future<void> setToken(String? value) async {
    token = value;
    authed.value = value != null && value.isNotEmpty;
    final prefs = _prefs ?? await SharedPreferences.getInstance();
    _prefs = prefs;
    if (authed.value) {
      await prefs.setString("token", value ?? "");
    } else {
      await prefs.remove("token");
    }
  }

  static Future<void> clear() async {
    await setToken(null);
  }
}
