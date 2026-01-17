import "package:flutter/material.dart";

import "../services/api_client.dart";
import "../services/notification_store.dart";

class NotificationDetailPage extends StatefulWidget {
  final int notificationId;
  const NotificationDetailPage({super.key, required this.notificationId});

  @override
  State<NotificationDetailPage> createState() => _NotificationDetailPageState();
}

class _NotificationDetailPageState extends State<NotificationDetailPage> {
  Map<String, dynamic>? _detail;
  bool _loading = false;

  @override
  void initState() {
    super.initState();
    _load();
  }

  Future<void> _load() async {
    setState(() => _loading = true);
    try {
      final data = await ApiClient.get("/notifications/${widget.notificationId}");
      if (data is Map<String, dynamic>) {
        _detail = data;
        if (_detail?["read_at"] == null) {
          await _markRead();
        }
      }
    } catch (error) {
      _showMessage("加载失败：$error");
    } finally {
      if (mounted) {
        setState(() => _loading = false);
      }
    }
  }

  Future<void> _markRead() async {
    try {
      final data = await ApiClient.post("/notifications/${widget.notificationId}/read", {});
      if (data is Map<String, dynamic>) {
        _detail = data;
      }
      await NotificationStore.refresh();
    } catch (_) {
      // 标记失败不打断阅读
    }
  }

  void _showMessage(String message) {
    ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text(message)));
  }

  @override
  Widget build(BuildContext context) {
    final detail = _detail ?? {};
    return Scaffold(
      appBar: AppBar(
        title: const Text("通知详情"),
        actions: [IconButton(onPressed: _load, icon: const Icon(Icons.refresh))],
      ),
      body: _loading
          ? const Center(child: CircularProgressIndicator())
          : ListView(
              padding: const EdgeInsets.all(16),
              children: [
                Text(
                  detail["title"]?.toString() ?? "通知",
                  style: const TextStyle(fontSize: 18, fontWeight: FontWeight.bold),
                ),
                const SizedBox(height: 8),
                Text("类型：${detail["type"] ?? "-"}"),
                const SizedBox(height: 4),
                Text("时间：${detail["created_at"] ?? "-"}"),
                const Divider(height: 24),
                Text(detail["content"]?.toString() ?? "暂无内容"),
              ],
            ),
    );
  }
}
