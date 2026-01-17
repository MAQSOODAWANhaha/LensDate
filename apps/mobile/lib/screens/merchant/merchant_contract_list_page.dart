import "dart:convert";

import "package:flutter/material.dart";

import "../../services/api_client.dart";

class MerchantContractListPage extends StatefulWidget {
  final int merchantId;
  final String merchantName;
  const MerchantContractListPage({super.key, required this.merchantId, required this.merchantName});

  @override
  State<MerchantContractListPage> createState() => _MerchantContractListPageState();
}

class _MerchantContractListPageState extends State<MerchantContractListPage> {
  bool _loading = false;
  bool _loadingMore = false;
  bool _creating = false;
  List<Map<String, dynamic>> _items = [];
  int _page = 1;
  bool _hasMore = true;
  static const int _pageSize = 20;
  final TextEditingController _orderController = TextEditingController();
  final TextEditingController _termsController = TextEditingController();
  final TextEditingController _versionController = TextEditingController(text: "1");
  final ScrollController _scrollController = ScrollController();

  @override
  void initState() {
    super.initState();
    _scrollController.addListener(_handleScroll);
    _load(reset: true);
  }

  @override
  void dispose() {
    _orderController.dispose();
    _termsController.dispose();
    _versionController.dispose();
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
      _items = [];
      setState(() => _loading = true);
    } else {
      setState(() => _loadingMore = true);
    }
    try {
      final data = await ApiClient.get(
        "/merchant-contracts?merchant_id=${widget.merchantId}&page=$_page&page_size=$_pageSize",
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
    await _load();
  }

  Future<void> _createContract() async {
    final orderId = int.tryParse(_orderController.text.trim());
    if (orderId == null) {
      _showMessage("请填写订单 ID");
      return;
    }
    Object? terms;
    final termsText = _termsController.text.trim();
    if (termsText.isEmpty) {
      _showMessage("请填写合同条款 JSON");
      return;
    }
    try {
      terms = jsonDecode(termsText);
    } catch (_) {
      _showMessage("合同条款必须是合法 JSON");
      return;
    }

    final version = int.tryParse(_versionController.text.trim()) ?? 1;
    setState(() => _creating = true);
    try {
      await ApiClient.post("/merchant-contracts", {
        "order_id": orderId,
        "terms": terms,
        "version": version,
      });
      _orderController.clear();
      _termsController.clear();
      _versionController.text = "1";
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

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text("${widget.merchantName} 合同"),
        actions: [
          IconButton(onPressed: () => _load(reset: true), icon: const Icon(Icons.refresh))
        ],
      ),
      body: _loading
          ? const Center(child: CircularProgressIndicator())
          : RefreshIndicator(
              onRefresh: () => _load(reset: true),
              child: ListView(
                controller: _scrollController,
                padding: const EdgeInsets.all(16),
                children: [
                  const Text("新增合同", style: TextStyle(fontWeight: FontWeight.bold)),
                  const SizedBox(height: 8),
                  TextField(
                    controller: _orderController,
                    decoration: const InputDecoration(labelText: "订单 ID"),
                    keyboardType: TextInputType.number,
                  ),
                  TextField(
                    controller: _termsController,
                    decoration: const InputDecoration(labelText: "合同条款 JSON"),
                    maxLines: 3,
                  ),
                  TextField(
                    controller: _versionController,
                    decoration: const InputDecoration(labelText: "版本号"),
                    keyboardType: TextInputType.number,
                  ),
                  const SizedBox(height: 8),
                  FilledButton(
                    onPressed: _creating ? null : _createContract,
                    child: Text(_creating ? "提交中..." : "创建合同"),
                  ),
                  const Divider(height: 32),
                  const Text("合同列表", style: TextStyle(fontWeight: FontWeight.bold)),
                  const SizedBox(height: 8),
                  if (_items.isEmpty)
                    const Text("暂无合同")
                  else
                    for (final item in _items)
                      Card(
                        margin: const EdgeInsets.symmetric(vertical: 8),
                        child: Padding(
                          padding: const EdgeInsets.all(12),
                          child: Column(
                            crossAxisAlignment: CrossAxisAlignment.start,
                            children: [
                              Text("合同 #${item["id"]}"),
                              Text("订单 ID：${item["order_id"]}"),
                              Text("版本：${item["version"]}"),
                              Text("创建时间：${item["created_at"]}"),
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
    );
  }
}
