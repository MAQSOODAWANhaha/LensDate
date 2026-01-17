import "package:flutter/material.dart";

import "../../services/api_client.dart";
import "portfolio_list_page.dart";

class PhotographerDetailPage extends StatefulWidget {
  final int photographerId;
  final Map<String, dynamic>? initial;
  const PhotographerDetailPage({
    super.key,
    required this.photographerId,
    this.initial,
  });

  @override
  State<PhotographerDetailPage> createState() => _PhotographerDetailPageState();
}

class _PhotographerDetailPageState extends State<PhotographerDetailPage> {
  Map<String, dynamic>? _detail;
  bool _loading = false;

  @override
  void initState() {
    super.initState();
    _detail = widget.initial;
    _load();
  }

  Future<void> _load() async {
    setState(() => _loading = true);
    try {
      final data = await ApiClient.get("/photographers/${widget.photographerId}");
      if (data is Map<String, dynamic>) {
        _detail = {...?_detail, ...data};
      }
    } catch (error) {
      _showMessage("加载失败：$error");
    } finally {
      if (mounted) {
        setState(() => _loading = false);
      }
    }
  }

  void _openPortfolios() {
    Navigator.of(context).push(
      MaterialPageRoute(
        builder: (_) => PortfolioListPage(
          photographerId: widget.photographerId,
          readOnly: true,
          title: "作品集",
        ),
      ),
    );
  }

  void _showMessage(String message) {
    ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text(message)));
  }

  String _typeLabel(String value) {
    switch (value) {
      case "individual":
        return "个人";
      case "team":
        return "团队";
      default:
        return value;
    }
  }

  @override
  Widget build(BuildContext context) {
    final detail = _detail ?? {};
    final nickname = detail["nickname"]?.toString() ?? "摄影师 #${widget.photographerId}";
    final rating = (detail["rating_avg"] as num?)?.toDouble() ?? 0.0;
    final completed = detail["completed_orders"] ?? 0;
    return Scaffold(
      appBar: AppBar(
        title: Text(nickname),
        actions: [IconButton(onPressed: _load, icon: const Icon(Icons.refresh))],
      ),
      body: _loading
          ? const Center(child: CircularProgressIndicator())
          : ListView(
              padding: const EdgeInsets.all(16),
              children: [
                Card(
                  child: Padding(
                    padding: const EdgeInsets.all(16),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text(nickname, style: const TextStyle(fontWeight: FontWeight.bold)),
                        const SizedBox(height: 8),
                        Text("类型：${_typeLabel(detail["type"]?.toString() ?? "-")}"),
                        Text("状态：${detail["status"] ?? "-"}"),
                        Text("城市：${detail["city_id"] ?? "-"}"),
                        Text("服务范围：${detail["service_area"] ?? "-"}"),
                        Text("评分：${rating.toStringAsFixed(1)} / 完成单数：$completed"),
                      ],
                    ),
                  ),
                ),
                const SizedBox(height: 12),
                FilledButton.icon(
                  onPressed: _openPortfolios,
                  icon: const Icon(Icons.photo_library_outlined),
                  label: const Text("查看作品集"),
                ),
              ],
            ),
    );
  }
}
