import "package:flutter/material.dart";

import "../../services/api_client.dart";

class QuoteVersionsPage extends StatefulWidget {
  final int quoteId;
  const QuoteVersionsPage({super.key, required this.quoteId});

  @override
  State<QuoteVersionsPage> createState() => _QuoteVersionsPageState();
}

class _QuoteVersionsPageState extends State<QuoteVersionsPage> {
  bool _loading = false;
  List<Map<String, dynamic>> _items = [];
  String? _lastError;

  @override
  void initState() {
    super.initState();
    _load();
  }

  Future<void> _load() async {
    setState(() => _loading = true);
    try {
      final data = await ApiClient.get("/quotes/${widget.quoteId}/versions");
      if (data is List) {
        _items = data.cast<Map<String, dynamic>>();
      } else {
        _items = [];
      }
      _lastError = null;
    } catch (error) {
      _items = [];
      _lastError = error.toString();
      _showMessage("加载失败：$error");
    } finally {
      if (mounted) {
        setState(() => _loading = false);
      }
    }
  }

  void _showMessage(String message) {
    ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text(message)));
  }

  Widget _buildVersion(Map<String, dynamic> item) {
    final items = item["items"] is List ? item["items"] as List : [];
    return Card(
      child: Padding(
        padding: const EdgeInsets.all(12),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text('版本：v${item["version"]}'),
            Text('总价：${item["total_price"] ?? "-"}'),
            if (item["note"] != null) Text('说明：${item["note"]}'),
            const SizedBox(height: 6),
            const Text("条目："),
            for (final entry in items)
              Text('- ${entry["name"]} x${entry["quantity"]} ¥${entry["price"]}'),
            const SizedBox(height: 6),
            Text('时间：${item["created_at"] ?? "-"}'),
          ],
        ),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text("报价版本 #${widget.quoteId}"),
        actions: [IconButton(onPressed: _load, icon: const Icon(Icons.refresh))],
      ),
      body: _loading
          ? const Center(child: CircularProgressIndicator())
          : ListView(
              padding: const EdgeInsets.all(16),
              children: [
                if (_items.isEmpty)
                  Padding(
                    padding: const EdgeInsets.only(top: 80),
                    child: Column(
                      children: [
                        Text(_lastError != null ? "加载失败" : "暂无版本记录"),
                        const SizedBox(height: 8),
                        if (_lastError != null)
                          FilledButton.tonal(
                            onPressed: _load,
                            child: const Text("点击重试"),
                          ),
                      ],
                    ),
                  )
                else
                  for (final item in _items) _buildVersion(item),
              ],
            ),
    );
  }
}
