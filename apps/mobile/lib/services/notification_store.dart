import "package:flutter/foundation.dart";

import "api_client.dart";

class NotificationStore {
  static final ValueNotifier<int> unreadCount = ValueNotifier(0);

  static Future<void> refresh() async {
    try {
      final data = await ApiClient.get("/notifications/summary");
      if (data is Map<String, dynamic>) {
        final count = data["unread_count"] as int? ?? 0;
        unreadCount.value = count;
      }
    } catch (_) {
      // 忽略刷新失败
    }
  }

  static void setCount(int count) {
    unreadCount.value = count;
  }

  static void decrement() {
    final next = unreadCount.value - 1;
    unreadCount.value = next < 0 ? 0 : next;
  }
}
