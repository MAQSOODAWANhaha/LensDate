import "package:flutter/material.dart";

import "package:shared_preferences/shared_preferences.dart";

import "../services/api_client.dart";
import "../services/notification_store.dart";
import "chat/conversations_tab.dart";
import "notification_detail_page.dart";

class MessagesPage extends StatelessWidget {
  const MessagesPage({super.key});

  @override
  Widget build(BuildContext context) {
    return DefaultTabController(
      length: 2,
      child: Scaffold(
        appBar: AppBar(
          title: const Text("消息"),
          bottom: const TabBar(
            tabs: [
              Tab(text: "通知"),
              Tab(text: "聊天"),
            ],
          ),
        ),
        body: const TabBarView(
          children: [
            NotificationsTab(),
            ConversationsTab(),
          ],
        ),
      ),
    );
  }
}

class NotificationsTab extends StatefulWidget {
  const NotificationsTab({super.key});

  @override
  State<NotificationsTab> createState() => _NotificationsTabState();
}

class _NotificationsTabState extends State<NotificationsTab> {
  bool _loading = false;
  bool _loadingMore = false;
  List<Map<String, dynamic>> _items = [];
  String _readStatus = "all";
  int _page = 1;
  bool _hasMore = true;
  bool _defaultUnread = false;
  String? _lastError;
  static const int _pageSize = 20;
  static const String _prefStatusKey = "notification_read_status";
  static const String _prefDefaultUnreadKey = "notification_default_unread";
  final ScrollController _scrollController = ScrollController();

  @override
  void initState() {
    super.initState();
    _initPrefs();
    _scrollController.addListener(_handleScroll);
  }

  @override
  void dispose() {
    _scrollController.removeListener(_handleScroll);
    _scrollController.dispose();
    super.dispose();
  }

  void _handleScroll() {
    if (_scrollController.position.pixels >=
        _scrollController.position.maxScrollExtent - 200) {
      _loadMore();
    }
  }

  Future<void> _initPrefs() async {
    final prefs = await SharedPreferences.getInstance();
    _defaultUnread = prefs.getBool(_prefDefaultUnreadKey) ?? false;
    final lastStatus = prefs.getString(_prefStatusKey);
    final defaultStatus = _defaultUnread ? "unread" : "all";
    _readStatus = lastStatus ?? defaultStatus;
    if (mounted) {
      setState(() {});
    }
    await _load(reset: true);
  }

  Future<void> _load({bool reset = false}) async {
    if (reset) {
      _page = 1;
      _hasMore = true;
      _items = [];
    }
    setState(() => _loading = reset ? true : _loading);
    try {
      final filter = _readStatus == "all" ? "" : "&read_status=$_readStatus";
      final data = await ApiClient.get(
        "/notifications?page=$_page&page_size=$_pageSize$filter",
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
      await NotificationStore.refresh();
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

  void _showMessage(String message) {
    ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text(message)));
  }

  Future<void> _markAllRead() async {
    try {
      await ApiClient.post("/notifications/read-all", {});
      NotificationStore.setCount(0);
      if (_readStatus == "unread") {
        setState(() => _readStatus = "read");
        final prefs = await SharedPreferences.getInstance();
        await prefs.setString(_prefStatusKey, _readStatus);
      }
      await _load(reset: true);
    } catch (error) {
      _showMessage("操作失败：$error");
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

  Widget _buildFilterChips() {
    return Padding(
      padding: const EdgeInsets.fromLTRB(16, 12, 16, 4),
      child: Wrap(
        spacing: 8,
        children: [
          FilterChip(
            label: const Text("默认仅未读"),
            selected: _defaultUnread,
            onSelected: (value) async {
              final prefs = await SharedPreferences.getInstance();
              setState(() => _defaultUnread = value);
              await prefs.setBool(_prefDefaultUnreadKey, value);
              if (value) {
                _readStatus = "unread";
                await prefs.setString(_prefStatusKey, _readStatus);
                if (mounted) {
                  setState(() {});
                }
                await _load(reset: true);
              }
            },
          ),
          ActionChip(
            label: const Text("仅未读"),
            onPressed: () async {
              final prefs = await SharedPreferences.getInstance();
              setState(() => _readStatus = "unread");
              await prefs.setString(_prefStatusKey, _readStatus);
              await _load(reset: true);
            },
          ),
          ChoiceChip(
            label: const Text("全部"),
            selected: _readStatus == "all",
            onSelected: (_) {
              setState(() => _readStatus = "all");
              SharedPreferences.getInstance()
                  .then((prefs) => prefs.setString(_prefStatusKey, _readStatus));
              _load(reset: true);
            },
          ),
          ChoiceChip(
            label: const Text("未读"),
            selected: _readStatus == "unread",
            onSelected: (_) {
              setState(() => _readStatus = "unread");
              SharedPreferences.getInstance()
                  .then((prefs) => prefs.setString(_prefStatusKey, _readStatus));
              _load(reset: true);
            },
          ),
          ChoiceChip(
            label: const Text("已读"),
            selected: _readStatus == "read",
            onSelected: (_) {
              setState(() => _readStatus = "read");
              SharedPreferences.getInstance()
                  .then((prefs) => prefs.setString(_prefStatusKey, _readStatus));
              _load(reset: true);
            },
          ),
        ],
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return _loading
        ? const Center(child: CircularProgressIndicator())
        : RefreshIndicator(
            onRefresh: () => _load(reset: true),
            child: ListView(
              controller: _scrollController,
              children: [
                _buildFilterChips(),
                Padding(
                  padding: const EdgeInsets.symmetric(horizontal: 16),
                  child: Row(
                    children: [
                      ValueListenableBuilder<int>(
                        valueListenable: NotificationStore.unreadCount,
                        builder: (context, count, _) {
                          return FilledButton.tonal(
                            onPressed: count == 0 ? null : _markAllRead,
                            child: const Text("全部标记已读"),
                          );
                        },
                      ),
                      const SizedBox(width: 8),
                      OutlinedButton(
                        onPressed: () => _load(reset: true),
                        child: const Text("刷新"),
                      ),
                    ],
                  ),
                ),
                if (_items.isEmpty)
                  Padding(
                    padding: const EdgeInsets.only(top: 120),
                    child: Column(
                      children: [
                        Text(_lastError != null ? "加载失败" : "暂无通知"),
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
                      padding: const EdgeInsets.fromLTRB(16, 8, 16, 0),
                      child: FilledButton.tonal(
                        onPressed: () => _load(reset: true),
                        child: const Text("加载失败，点击重试"),
                      ),
                    ),
                  ListView.separated(
                    padding: const EdgeInsets.all(16),
                    shrinkWrap: true,
                    physics: const NeverScrollableScrollPhysics(),
                    itemBuilder: (context, index) {
                      final item = _items[index];
                      final unread = item["read_at"] == null;
                      return ListTile(
                        tileColor: unread ? Colors.orange.shade50 : null,
                        title: Text(item["title"]?.toString() ?? "通知"),
                        subtitle: Text(item["content"]?.toString() ?? ""),
                        trailing: Text(item["created_at"]?.toString() ?? ""),
                        onTap: () async {
                          final id = item["id"] as int?;
                          if (id == null) {
                            return;
                          }
                          Navigator.of(context).push(
                            MaterialPageRoute(
                              builder: (_) => NotificationDetailPage(notificationId: id),
                            ),
                          );
                        },
                      );
                    },
                    separatorBuilder: (_, __) => const SizedBox(height: 8),
                    itemCount: _items.length,
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
