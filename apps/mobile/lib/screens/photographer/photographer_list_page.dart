import "package:flutter/material.dart";

import "../../services/api_client.dart";
import "photographer_detail_page.dart";

class PhotographerListPage extends StatefulWidget {
  final bool embedded;
  const PhotographerListPage({super.key, this.embedded = false});

  @override
  State<PhotographerListPage> createState() => _PhotographerListPageState();
}

class _PhotographerListPageState extends State<PhotographerListPage> {
  bool _loading = false;
  bool _loadingMore = false;
  List<Map<String, dynamic>> _items = [];
  int _page = 1;
  bool _hasMore = true;
  static const int _pageSize = 20;
  final ScrollController _scrollController = ScrollController();

  final TextEditingController _keywordController = TextEditingController();
  final TextEditingController _cityController = TextEditingController();
  String _type = "all";
  final String _status = "approved";

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
    _keywordController.dispose();
    _cityController.dispose();
    super.dispose();
  }

  void _handleScroll() {
    if (_scrollController.position.pixels >=
        _scrollController.position.maxScrollExtent - 160) {
      _loadMore();
    }
  }

  Map<String, String> _buildQuery() {
    final params = <String, String>{
      "page": _page.toString(),
      "page_size": _pageSize.toString(),
    };
    final keyword = _keywordController.text.trim();
    final cityId = int.tryParse(_cityController.text.trim());
    if (keyword.isNotEmpty) {
      params["keyword"] = keyword;
    }
    if (cityId != null) {
      params["city_id"] = cityId.toString();
    }
    if (_type != "all") {
      params["type"] = _type;
    }
    if (_status.isNotEmpty) {
      params["status"] = _status;
    }
    return params;
  }

  String _buildQueryString(Map<String, String> params) {
    return params.entries
        .map((e) => "${e.key}=${Uri.encodeQueryComponent(e.value)}")
        .join("&");
  }

  Future<void> _load({bool reset = false}) async {
    if (reset) {
      _page = 1;
      _hasMore = true;
      _items = [];
      setState(() => _loading = true);
    }
    try {
      final query = _buildQueryString(_buildQuery());
      final data = await ApiClient.get("/photographers?$query");
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
      } else if (reset) {
        _items = [];
        _hasMore = false;
      }
    } catch (error) {
      if (reset) {
        _items = [];
        _hasMore = false;
      }
      _showMessage("加载失败：$error");
    } finally {
      if (mounted) {
        setState(() {
          _loading = false;
          _loadingMore = false;
        });
      }
    }
  }

  Future<void> _loadMore() async {
    if (_loadingMore || !_hasMore || _loading) {
      return;
    }
    setState(() => _loadingMore = true);
    await _load();
  }

  void _openDetail(Map<String, dynamic> item) {
    final id = item["id"] as int? ?? 0;
    if (id == 0) {
      return;
    }
    Navigator.of(context).push(
      MaterialPageRoute(
        builder: (_) => PhotographerDetailPage(
          photographerId: id,
          initial: item,
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
    final body = Column(
      children: [
          Padding(
            padding: const EdgeInsets.all(16),
            child: Column(
              children: [
                TextField(
                  controller: _keywordController,
                  decoration: const InputDecoration(labelText: "关键词（手机号/昵称）"),
                  onSubmitted: (_) => _load(reset: true),
                ),
                const SizedBox(height: 8),
                TextField(
                  controller: _cityController,
                  decoration: const InputDecoration(labelText: "城市 ID"),
                  keyboardType: TextInputType.number,
                  onSubmitted: (_) => _load(reset: true),
                ),
                const SizedBox(height: 8),
                DropdownButtonFormField<String>(
                  key: ValueKey(_type),
                  initialValue: _type,
                  decoration: const InputDecoration(labelText: "类型"),
                  items: const [
                    DropdownMenuItem(value: "all", child: Text("全部")),
                    DropdownMenuItem(value: "individual", child: Text("个人")),
                    DropdownMenuItem(value: "team", child: Text("团队")),
                  ],
                  onChanged: (value) {
                    setState(() => _type = value ?? "all");
                    _load(reset: true);
                  },
                ),
              ],
            ),
          ),
          const Divider(height: 1),
          Expanded(
            child: _loading
                ? const Center(child: CircularProgressIndicator())
                : RefreshIndicator(
                    onRefresh: () => _load(reset: true),
                    child: _items.isEmpty
                        ? ListView(
                            children: const [
                              SizedBox(height: 120),
                              Center(child: Text("暂无摄影师")),
                            ],
                          )
                        : ListView.separated(
                            controller: _scrollController,
                            padding: const EdgeInsets.all(16),
                            itemBuilder: (context, index) {
                              final item = _items[index];
                              final nickname =
                                  item["nickname"]?.toString() ?? "摄影师 #${item["id"] ?? "-"}";
                              final rating = (item["rating_avg"] as num?)?.toDouble() ?? 0.0;
                              final completed = item["completed_orders"] ?? 0;
                              return ListTile(
                                tileColor: Colors.white,
                                title: Text(nickname),
                                subtitle: Text(
                                  "类型：${_typeLabel(item["type"]?.toString() ?? "-")} "
                                  "· 评分：${rating.toStringAsFixed(1)} "
                                  "· 完成单数：$completed",
                                ),
                                trailing: const Icon(Icons.chevron_right),
                                onTap: () => _openDetail(item),
                              );
                            },
                            separatorBuilder: (_, __) => const SizedBox(height: 12),
                            itemCount: _items.length,
                          ),
                  ),
          ),
          if (_loadingMore)
            const Padding(
              padding: EdgeInsets.all(12),
              child: CircularProgressIndicator(),
            ),
      ],
    );

    if (widget.embedded) {
      return body;
    }

    return Scaffold(
      appBar: AppBar(
        title: const Text("摄影师"),
        actions: [IconButton(onPressed: () => _load(reset: true), icon: const Icon(Icons.refresh))],
      ),
      body: body,
    );
  }
}
