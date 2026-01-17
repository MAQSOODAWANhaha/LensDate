import "package:flutter/material.dart";

import "../../services/api_client.dart";
import "portfolio_items_page.dart";

class PortfolioListPage extends StatefulWidget {
  final int photographerId;
  final bool readOnly;
  final String? title;
  const PortfolioListPage({
    super.key,
    required this.photographerId,
    this.readOnly = false,
    this.title,
  });

  @override
  State<PortfolioListPage> createState() => _PortfolioListPageState();
}

class _PortfolioListPageState extends State<PortfolioListPage> {
  bool _loading = false;
  List<Map<String, dynamic>> _items = [];
  final TextEditingController _titleController = TextEditingController();
  bool _creating = false;

  @override
  void initState() {
    super.initState();
    _load();
  }

  @override
  void dispose() {
    _titleController.dispose();
    super.dispose();
  }

  Future<void> _load() async {
    setState(() => _loading = true);
    try {
      final data = await ApiClient.get("/portfolios?photographer_id=${widget.photographerId}");
      if (data is List) {
        _items = data.cast<Map<String, dynamic>>();
      } else {
        _items = [];
      }
    } catch (error) {
      _items = [];
      _showMessage("加载失败：$error");
    } finally {
      if (mounted) {
        setState(() => _loading = false);
      }
    }
  }

  Future<void> _createPortfolio() async {
    final title = _titleController.text.trim();
    if (title.isEmpty) {
      _showMessage("请填写作品集标题");
      return;
    }
    setState(() => _creating = true);
    try {
      await ApiClient.post("/portfolios", {
        "photographer_id": widget.photographerId,
        "title": title,
      });
      _titleController.clear();
      await _load();
    } catch (error) {
      _showMessage("创建失败：$error");
    } finally {
      if (mounted) {
        setState(() => _creating = false);
      }
    }
  }

  void _openItems(Map<String, dynamic> item) {
    final id = item["id"] as int? ?? 0;
    final title = item["title"]?.toString() ?? "作品集";
    Navigator.of(context).push(
      MaterialPageRoute(
        builder: (_) => PortfolioItemsPage(
          portfolioId: id,
          title: title,
          readOnly: widget.readOnly,
        ),
      ),
    );
  }

  void _showMessage(String message) {
    ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text(message)));
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text(widget.title ?? (widget.readOnly ? "作品集" : "作品集管理")),
        actions: [IconButton(onPressed: _load, icon: const Icon(Icons.refresh))],
      ),
      body: Column(
        children: [
          if (!widget.readOnly) ...[
            Padding(
              padding: const EdgeInsets.all(16),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  const Text("新增作品集", style: TextStyle(fontWeight: FontWeight.bold)),
                  const SizedBox(height: 8),
                  TextField(
                    controller: _titleController,
                    decoration: const InputDecoration(labelText: "标题"),
                  ),
                  const SizedBox(height: 8),
                  FilledButton(
                    onPressed: _creating ? null : _createPortfolio,
                    child: Text(_creating ? "创建中..." : "创建作品集"),
                  ),
                ],
              ),
            ),
            const Divider(height: 1),
          ],
          Expanded(
            child: _loading
                ? const Center(child: CircularProgressIndicator())
                : RefreshIndicator(
                    onRefresh: _load,
                    child: _items.isEmpty
                        ? ListView(
                            children: const [
                              SizedBox(height: 120),
                              Center(child: Text("暂无作品集")),
                            ],
                          )
                        : ListView.separated(
                            padding: const EdgeInsets.all(16),
                            itemBuilder: (context, index) {
                              final item = _items[index];
                              return ListTile(
                                tileColor: Colors.white,
                                title: Text(item["title"]?.toString() ?? "作品集"),
                                subtitle: Text("状态：${item["status"] ?? "-"}"),
                                trailing: const Icon(Icons.chevron_right),
                                onTap: () => _openItems(item),
                              );
                            },
                            separatorBuilder: (_, __) => const SizedBox(height: 12),
                            itemCount: _items.length,
                          ),
                  ),
          ),
        ],
      ),
    );
  }
}
