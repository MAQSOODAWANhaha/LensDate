import "dart:convert";

import "package:flutter/material.dart";

import "../../services/api_client.dart";

class MerchantAssetVersionsPage extends StatefulWidget {
  final int assetId;
  final String assetName;
  const MerchantAssetVersionsPage({super.key, required this.assetId, required this.assetName});

  @override
  State<MerchantAssetVersionsPage> createState() => _MerchantAssetVersionsPageState();
}

class _MerchantAssetVersionsPageState extends State<MerchantAssetVersionsPage> {
  bool _loading = false;
  bool _loadingMore = false;
  bool _creating = false;
  List<Map<String, dynamic>> _versions = [];
  int _page = 1;
  bool _hasMore = true;
  static const int _pageSize = 20;
  final ScrollController _scrollController = ScrollController();

  final TextEditingController _payloadController = TextEditingController();

  @override
  void initState() {
    super.initState();
    _scrollController.addListener(_handleScroll);
    _load(reset: true);
  }

  @override
  void dispose() {
    _payloadController.dispose();
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

  Future<void> _load({bool reset = false}) async {
    if (reset) {
      _page = 1;
      _hasMore = true;
      _versions = [];
      setState(() => _loading = true);
    } else {
      setState(() => _loadingMore = true);
    }
    try {
      final data = await ApiClient.get(
        "/merchants/assets/${widget.assetId}/versions?page=$_page&page_size=$_pageSize",
      );
      if (data is Map<String, dynamic>) {
        final rawItems = data["items"];
        final list = rawItems is List
            ? rawItems.cast<Map<String, dynamic>>()
            : <Map<String, dynamic>>[];
        final total = (data["total"] as num?)?.toInt() ?? list.length;
        if (reset) {
          _versions = list;
        } else {
          _versions.addAll(list);
        }
        if (list.isEmpty) {
          _hasMore = false;
        } else {
          _hasMore = _versions.length < total;
          if (_hasMore) {
            _page += 1;
          }
        }
      } else if (data is List) {
        final list = data.cast<Map<String, dynamic>>();
        if (reset) {
          _versions = list;
        } else {
          _versions.addAll(list);
        }
        _hasMore = list.length == _pageSize;
        if (_hasMore) {
          _page += 1;
        }
      } else if (reset) {
        _versions = [];
        _hasMore = false;
      }
    } catch (error) {
      if (reset) {
        _versions = [];
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
    await _load();
  }

  Future<void> _createVersion() async {
    final payloadText = _payloadController.text.trim();
    if (payloadText.isEmpty) {
      _showMessage("请填写版本内容 JSON");
      return;
    }
    Object payload;
    try {
      payload = jsonDecode(payloadText);
    } catch (_) {
      _showMessage("内容必须是合法 JSON");
      return;
    }

    setState(() => _creating = true);
    try {
      await ApiClient.post(
        "/merchants/assets/${widget.assetId}/versions",
        {"payload": payload},
      );
      _payloadController.clear();
      await _load(reset: true);
    } catch (error) {
      _showMessage("创建失败：$error");
    } finally {
      if (mounted) {
        setState(() => _creating = false);
      }
    }
  }

  void _showMessage(String message) {
    ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text(message)));
  }

  String _formatPayload(Object? payload) {
    if (payload == null) {
      return "-";
    }
    String text;
    try {
      text = jsonEncode(payload);
    } catch (_) {
      text = payload.toString();
    }
    if (text.length > 200) {
      return "${text.substring(0, 200)}...";
    }
    return text;
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text("${widget.assetName} 版本"),
        actions: [
          IconButton(onPressed: () => _load(reset: true), icon: const Icon(Icons.refresh))
        ],
      ),
      body: Column(
        children: [
          Expanded(
            child: _loading
                ? const Center(child: CircularProgressIndicator())
                : RefreshIndicator(
                    onRefresh: () => _load(reset: true),
                    child: ListView(
                      controller: _scrollController,
                      padding: const EdgeInsets.all(16),
                      children: [
                        const Text("新增版本", style: TextStyle(fontWeight: FontWeight.bold)),
                        const SizedBox(height: 8),
                        TextField(
                          controller: _payloadController,
                          decoration: const InputDecoration(labelText: "版本内容 JSON"),
                          maxLines: 3,
                        ),
                        const SizedBox(height: 8),
                        FilledButton(
                          onPressed: _creating ? null : _createVersion,
                          child: Text(_creating ? "提交中..." : "新增版本"),
                        ),
                        const SizedBox(height: 16),
                        const Divider(),
                        const Text("版本列表", style: TextStyle(fontWeight: FontWeight.bold)),
                        const SizedBox(height: 8),
                        if (_versions.isEmpty)
                          const Text("暂无版本")
                        else
                          for (final version in _versions)
                            Card(
                              margin: const EdgeInsets.symmetric(vertical: 8),
                              child: Padding(
                                padding: const EdgeInsets.all(12),
                                child: Column(
                                  crossAxisAlignment: CrossAxisAlignment.start,
                                  children: [
                                    Text(
                                      "v${version["version"] ?? "-"}",
                                      style: const TextStyle(fontWeight: FontWeight.bold),
                                    ),
                                    const SizedBox(height: 4),
                                    Text("创建人：${version["created_by"] ?? "-"}"),
                                    Text("创建时间：${version["created_at"] ?? "-"}"),
                                    const SizedBox(height: 4),
                                    Text("内容：${_formatPayload(version["payload"])}"),
                                  ],
                                ),
                              ),
                            ),
                        if (_loadingMore)
                          const Padding(
                            padding: EdgeInsets.symmetric(vertical: 12),
                            child: Center(child: CircularProgressIndicator()),
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
