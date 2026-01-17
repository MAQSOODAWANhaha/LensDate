import "package:flutter/material.dart";

import "../../services/api_client.dart";
import "chat_messages_page.dart";

class ConversationsTab extends StatefulWidget {
  const ConversationsTab({super.key});

  @override
  State<ConversationsTab> createState() => _ConversationsTabState();
}

class _ConversationsTabState extends State<ConversationsTab> {
  bool _loading = false;
  bool _loadingMore = false;
  List<Map<String, dynamic>> _items = [];
  int _page = 1;
  bool _hasMore = true;
  String? _lastError;
  final TextEditingController _orderIdController = TextEditingController();
  final ScrollController _scrollController = ScrollController();

  static const int _pageSize = 20;

  @override
  void initState() {
    super.initState();
    _scrollController.addListener(_handleScroll);
    _load(reset: true);
  }

  @override
  void dispose() {
    _scrollController.removeListener(_handleScroll);
    _scrollController.dispose();
    _orderIdController.dispose();
    super.dispose();
  }

  void _handleScroll() {
    if (_scrollController.position.pixels >=
        _scrollController.position.maxScrollExtent - 200) {
      _loadMore();
    }
  }

  Future<void> _load({bool reset = false}) async {
    if (reset) {
      _page = 1;
      _hasMore = true;
      _items = [];
      setState(() => _loading = true);
    }
    try {
      final data = await ApiClient.get(
        "/conversations?page=$_page&page_size=$_pageSize",
      );
      if (data is Map<String, dynamic>) {
        final rawItems = data["items"];
        final list = rawItems is List
            ? rawItems.cast<Map<String, dynamic>>()
            : <Map<String, dynamic>>[];
        final total = (data["total"] as num?)?.toInt() ?? list.length;
        if (reset) {
          _items = list;
        } else {
          _items.addAll(list);
        }
        if (list.isEmpty) {
          _hasMore = false;
        } else {
          _hasMore = _items.length < total;
          if (_hasMore) {
            _page += 1;
          }
        }
      } else if (data is List) {
        final list = data.cast<Map<String, dynamic>>();
        if (reset) {
          _items = list;
        } else {
          _items.addAll(list);
        }
        _hasMore = list.length == _pageSize;
        if (_hasMore) {
          _page += 1;
        }
      } else {
        _items = [];
        _hasMore = false;
      }
      _lastError = null;
    } catch (error) {
      if (reset) {
        _items = [];
        _hasMore = false;
      }
      _lastError = error.toString();
      _showMessage("加载失败：$error");
    } finally {
      if (mounted) {
        setState(() => _loading = false);
      }
    }
  }

  Future<void> _loadMore() async {
    if (_loadingMore || !_hasMore) {
      return;
    }
    setState(() => _loadingMore = true);
    await _load();
    if (mounted) {
      setState(() => _loadingMore = false);
    }
  }

  Future<void> _openByOrder() async {
    final raw = _orderIdController.text.trim();
    final orderId = int.tryParse(raw);
    if (orderId == null) {
      _showMessage("请输入订单 ID");
      return;
    }
    try {
      final data = await ApiClient.post("/conversations", {
        "type": "order",
        "order_id": orderId,
      });
      if (data is Map<String, dynamic>) {
        final conversationId = data["id"] as int?;
        if (conversationId != null && mounted) {
          Navigator.of(context).push(
            MaterialPageRoute(
              builder: (_) => ChatMessagesPage(
                conversationId: conversationId,
                orderId: orderId,
              ),
            ),
          );
        }
      }
      await _load(reset: true);
    } catch (error) {
      _showMessage("进入失败：$error");
    }
  }

  void _openConversation(Map<String, dynamic> item) {
    final conversationId = item["id"] as int?;
    final orderId = item["order_id"] as int?;
    if (conversationId == null) {
      return;
    }
    Navigator.of(context).push(
      MaterialPageRoute(
        builder: (_) => ChatMessagesPage(
          conversationId: conversationId,
          orderId: orderId,
        ),
      ),
    );
  }

  void _showMessage(String message) {
    ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text(message)));
  }

  @override
  Widget build(BuildContext context) {
    return _loading
        ? const Center(child: CircularProgressIndicator())
        : RefreshIndicator(
            onRefresh: () => _load(reset: true),
            child: ListView(
              controller: _scrollController,
              padding: const EdgeInsets.all(16),
              children: [
                Card(
                  child: Padding(
                    padding: const EdgeInsets.all(12),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        const Text("按订单进入会话", style: TextStyle(fontWeight: FontWeight.bold)),
                        const SizedBox(height: 8),
                        Row(
                          children: [
                            Expanded(
                              child: TextField(
                                controller: _orderIdController,
                                keyboardType: TextInputType.number,
                                decoration: const InputDecoration(
                                  labelText: "订单 ID",
                                ),
                              ),
                            ),
                            const SizedBox(width: 8),
                            FilledButton(onPressed: _openByOrder, child: const Text("进入")),
                          ],
                        ),
                      ],
                    ),
                  ),
                ),
                const SizedBox(height: 12),
                if (_items.isEmpty)
                  Padding(
                    padding: const EdgeInsets.only(top: 80),
                    child: Column(
                      children: [
                        Text(_lastError != null ? "加载失败" : "暂无会话"),
                        const SizedBox(height: 8),
                        if (_lastError != null)
                          FilledButton.tonal(
                            onPressed: () => _load(reset: true),
                            child: const Text("点击重试"),
                          ),
                      ],
                    ),
                  )
                else ...[
                  if (_lastError != null)
                    Padding(
                      padding: const EdgeInsets.only(bottom: 8),
                      child: FilledButton.tonal(
                        onPressed: () => _load(reset: true),
                        child: const Text("加载失败，点击重试"),
                      ),
                    ),
                  for (final item in _items)
                    Card(
                      child: ListTile(
                        onTap: () => _openConversation(item),
                        title: Text('会话 #${item["id"]}'),
                        subtitle: Text('订单 ID：${item["order_id"] ?? "-"}'),
                        trailing: const Icon(Icons.chevron_right),
                      ),
                    ),
                  if (_loadingMore)
                    const Padding(
                      padding: EdgeInsets.symmetric(vertical: 12),
                      child: Center(child: CircularProgressIndicator()),
                    ),
                ],
              ],
            ),
          );
  }
}
