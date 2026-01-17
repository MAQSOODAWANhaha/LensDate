import "package:flutter/material.dart";

import "../../services/api_client.dart";
import "demand_merchant_assets_page.dart";
import "../quotes/quote_create_page.dart";
import "../quotes/quote_list_page.dart";

class DemandDetailPage extends StatefulWidget {
  final int demandId;
  const DemandDetailPage({super.key, required this.demandId});

  @override
  State<DemandDetailPage> createState() => _DemandDetailPageState();
}

class _DemandDetailPageState extends State<DemandDetailPage> {
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
      final data = await ApiClient.get("/demands/${widget.demandId}");
      if (data is Map<String, dynamic>) {
        _detail = data;
      }
    } catch (error) {
      _showMessage("加载失败：$error");
    } finally {
      if (mounted) {
        setState(() => _loading = false);
      }
    }
  }

  Future<void> _closeDemand() async {
    try {
      await ApiClient.post("/demands/${widget.demandId}/close", {});
      _showMessage("需求已关闭");
      await _load();
    } catch (error) {
      _showMessage("关闭失败：$error");
    }
  }

  void _showMessage(String message) {
    ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text(message)));
  }

  bool _isImageUrl(String url) {
    final lower = url.toLowerCase();
    return lower.endsWith(".jpg") ||
        lower.endsWith(".jpeg") ||
        lower.endsWith(".png") ||
        lower.endsWith(".gif") ||
        lower.endsWith(".webp");
  }

  String _resolveUrl(String url) {
    if (url.startsWith("http://") || url.startsWith("https://")) {
      return url;
    }
    if (url.startsWith("/")) {
      return "${ApiClient.baseUrl}$url";
    }
    return "${ApiClient.baseUrl}/$url";
  }

  @override
  Widget build(BuildContext context) {
    final detail = _detail ?? {};
    final attachments = detail["attachments"] is List ? detail["attachments"] as List : [];
    final isMerchant = detail["is_merchant"] == true;

    return Scaffold(
      appBar: AppBar(
        title: const Text("需求详情"),
        actions: [IconButton(onPressed: _load, icon: const Icon(Icons.refresh))],
      ),
      body: _loading
          ? const Center(child: CircularProgressIndicator())
          : ListView(
              padding: const EdgeInsets.all(16),
              children: [
                Text("类型：${detail["type"] ?? "-"}"),
                const SizedBox(height: 8),
                Text("状态：${detail["status"] ?? "-"}"),
                const SizedBox(height: 8),
                Text("城市：${detail["city_id"] ?? "-"}"),
                const SizedBox(height: 8),
                Text("时间：${detail["schedule_start"] ?? "-"} ~ ${detail["schedule_end"] ?? "-"}"),
                const SizedBox(height: 8),
                Text("预算：${detail["budget_min"] ?? "-"} ~ ${detail["budget_max"] ?? "-"}"),
                const SizedBox(height: 16),
                FilledButton(
                  onPressed: _closeDemand,
                  child: const Text("关闭需求"),
                ),
                const SizedBox(height: 8),
                FilledButton.tonal(
                  onPressed: () => Navigator.of(context).push(
                    MaterialPageRoute(
                      builder: (_) => QuoteCreatePage(demandId: widget.demandId),
                    ),
                  ),
                  child: const Text("提交报价"),
                ),
                const SizedBox(height: 8),
                OutlinedButton(
                  onPressed: () => Navigator.of(context).push(
                    MaterialPageRoute(
                      builder: (_) => QuoteListPage(demandId: widget.demandId),
                    ),
                  ),
                  child: const Text("查看报价"),
                ),
                if (isMerchant) ...[
                  const SizedBox(height: 8),
                  OutlinedButton(
                    onPressed: () => Navigator.of(context).push(
                      MaterialPageRoute(
                        builder: (_) =>
                            DemandMerchantAssetsPage(demandId: widget.demandId),
                      ),
                    ),
                    child: const Text("查看商户素材库"),
                  ),
                ],
                const Divider(height: 32),
                const Text("附件"),
                const SizedBox(height: 8),
                for (final item in attachments)
                  Card(
                    child: Padding(
                      padding: const EdgeInsets.all(8),
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Text(item["file_url"]?.toString() ?? "附件"),
                          if (item["file_type"] != null) Text(item["file_type"].toString()),
                          const SizedBox(height: 6),
                          if (item["file_url"] != null &&
                              _isImageUrl(item["file_url"].toString()))
                            ClipRRect(
                              borderRadius: BorderRadius.circular(8),
                              child: Image.network(
                                _resolveUrl(item["file_url"].toString()),
                                height: 160,
                                fit: BoxFit.cover,
                              ),
                            ),
                        ],
                      ),
                    ),
                  ),
              ],
            ),
    );
  }
}
