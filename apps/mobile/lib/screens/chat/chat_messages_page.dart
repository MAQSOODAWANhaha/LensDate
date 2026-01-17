import "package:flutter/material.dart";

import "../../services/api_client.dart";

class ChatMessagesPage extends StatefulWidget {
  final int conversationId;
  final int? orderId;
  const ChatMessagesPage({super.key, required this.conversationId, this.orderId});

  @override
  State<ChatMessagesPage> createState() => _ChatMessagesPageState();
}

class _ChatMessagesPageState extends State<ChatMessagesPage> {
  final TextEditingController _controller = TextEditingController();
  final ScrollController _scrollController = ScrollController();

  bool _loading = false;
  bool _loadingMore = false;
  bool _sending = false;
  List<Map<String, dynamic>> _items = [];
  int _page = 1;
  bool _hasMore = true;
  int? _currentUserId;

  static const int _pageSize = 20;

  @override
  void initState() {
    super.initState();
    _scrollController.addListener(_handleScroll);
    _loadCurrentUser();
    _load(reset: true);
  }

  @override
  void dispose() {
    _controller.dispose();
    _scrollController.removeListener(_handleScroll);
    _scrollController.dispose();
    super.dispose();
  }

  void _handleScroll() {
    if (_scrollController.position.pixels >=
        _scrollController.position.maxScrollExtent - 120) {
      _loadMore();
    }
  }

  Future<void> _loadCurrentUser() async {
    try {
      final me = await ApiClient.get("/users/me");
      if (me is Map<String, dynamic>) {
        _currentUserId = me["id"] as int?;
      }
    } catch (_) {
      _currentUserId = null;
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
        "/messages?conversation_id=${widget.conversationId}&page=$_page&page_size=$_pageSize",
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
    } catch (error) {
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

  Future<void> _send() async {
    final content = _controller.text.trim();
    if (content.isEmpty) {
      return;
    }
    setState(() => _sending = true);
    try {
      await ApiClient.post("/messages", {
        "conversation_id": widget.conversationId,
        "content": content,
        "msg_type": "text",
      });
      _controller.clear();
      await _load(reset: true);
    } catch (error) {
      _showMessage("发送失败：$error");
    } finally {
      if (mounted) {
        setState(() => _sending = false);
      }
    }
  }

  void _showMessage(String message) {
    ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text(message)));
  }

  Widget _buildMessage(Map<String, dynamic> item) {
    final senderId = item["sender_id"] as int?;
    final isMine = _currentUserId != null && senderId == _currentUserId;
    final content = item["content"]?.toString() ?? "";
    final sentAt = item["sent_at"]?.toString() ?? "";

    return Align(
      alignment: isMine ? Alignment.centerRight : Alignment.centerLeft,
      child: Container(
        margin: const EdgeInsets.symmetric(vertical: 6),
        padding: const EdgeInsets.all(12),
        constraints: const BoxConstraints(maxWidth: 280),
        decoration: BoxDecoration(
          color: isMine ? Colors.blue.shade50 : Colors.grey.shade200,
          borderRadius: BorderRadius.circular(12),
        ),
        child: Column(
          crossAxisAlignment: isMine ? CrossAxisAlignment.end : CrossAxisAlignment.start,
          children: [
            Text(content),
            const SizedBox(height: 4),
            Text(sentAt, style: const TextStyle(color: Colors.grey, fontSize: 11)),
          ],
        ),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    final title = widget.orderId == null
        ? "会话 #${widget.conversationId}"
        : "订单 #${widget.orderId}";
    return Scaffold(
      appBar: AppBar(
        title: Text(title),
        actions: [
          IconButton(onPressed: () => _load(reset: true), icon: const Icon(Icons.refresh)),
        ],
      ),
      body: Column(
        children: [
          Expanded(
            child: _loading
                ? const Center(child: CircularProgressIndicator())
                : ListView.builder(
                    controller: _scrollController,
                    reverse: true,
                    padding: const EdgeInsets.all(16),
                    itemCount: _items.length + (_loadingMore ? 1 : 0),
                    itemBuilder: (context, index) {
                      if (_loadingMore && index == _items.length) {
                        return const Padding(
                          padding: EdgeInsets.symmetric(vertical: 12),
                          child: Center(child: CircularProgressIndicator()),
                        );
                      }
                      final item = _items[index];
                      return _buildMessage(item);
                    },
                  ),
          ),
          SafeArea(
            top: false,
            child: Padding(
              padding: const EdgeInsets.fromLTRB(12, 8, 12, 12),
              child: Row(
                children: [
                  Expanded(
                    child: TextField(
                      controller: _controller,
                      minLines: 1,
                      maxLines: 4,
                      decoration: const InputDecoration(
                        hintText: "输入消息...",
                        border: OutlineInputBorder(),
                      ),
                    ),
                  ),
                  const SizedBox(width: 8),
                  FilledButton(
                    onPressed: _sending ? null : _send,
                    child: Text(_sending ? "发送中" : "发送"),
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
