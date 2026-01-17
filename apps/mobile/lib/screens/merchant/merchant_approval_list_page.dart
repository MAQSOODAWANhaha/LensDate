import "package:flutter/material.dart";

import "../../services/api_client.dart";

class MerchantApprovalListPage extends StatefulWidget {
  final int merchantId;
  final String merchantName;
  const MerchantApprovalListPage({super.key, required this.merchantId, required this.merchantName});

  @override
  State<MerchantApprovalListPage> createState() => _MerchantApprovalListPageState();
}

class _MerchantApprovalListPageState extends State<MerchantApprovalListPage> {
  bool _loading = false;
  bool _loadingMore = false;
  bool _creating = false;
  List<Map<String, dynamic>> _items = [];
  int _page = 1;
  bool _hasMore = true;
  static const int _pageSize = 20;
  String _status = "pending";
  final TextEditingController _demandController = TextEditingController();
  final TextEditingController _commentController = TextEditingController();
  final ScrollController _scrollController = ScrollController();

  @override
  void initState() {
    super.initState();
    _scrollController.addListener(_handleScroll);
    _load(reset: true);
  }

  @override
  void dispose() {
    _demandController.dispose();
    _commentController.dispose();
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
        "/merchant-approvals?merchant_id=${widget.merchantId}&page=$_page&page_size=$_pageSize",
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

  Future<void> _createApproval() async {
    final demandId = int.tryParse(_demandController.text.trim());
    if (demandId == null) {
      _showMessage("请填写需求 ID");
      return;
    }
    setState(() => _creating = true);
    try {
      await ApiClient.post("/merchant-approvals", {
        "demand_id": demandId,
        "merchant_id": widget.merchantId,
        "status": _status,
        "comment": _commentController.text.trim().isEmpty ? null : _commentController.text.trim()
      });
      _demandController.clear();
      _commentController.clear();
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
        title: Text("${widget.merchantName} 审批"),
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
                  const Text("新增审批", style: TextStyle(fontWeight: FontWeight.bold)),
                  const SizedBox(height: 8),
                  TextField(
                    controller: _demandController,
                    decoration: const InputDecoration(labelText: "需求 ID"),
                    keyboardType: TextInputType.number,
                  ),
                  DropdownButtonFormField<String>(
                    key: ValueKey(_status),
                    initialValue: _status,
                    decoration: const InputDecoration(labelText: "状态"),
                    items: const [
                      DropdownMenuItem(value: "draft", child: Text("草稿")),
                      DropdownMenuItem(value: "pending", child: Text("待审批")),
                      DropdownMenuItem(value: "approved", child: Text("已通过")),
                      DropdownMenuItem(value: "rejected", child: Text("已拒绝")),
                    ],
                    onChanged: (value) => setState(() => _status = value ?? "pending"),
                  ),
                  TextField(
                    controller: _commentController,
                    decoration: const InputDecoration(labelText: "备注（可选）"),
                  ),
                  const SizedBox(height: 8),
                  FilledButton(
                    onPressed: _creating ? null : _createApproval,
                    child: Text(_creating ? "提交中..." : "提交审批"),
                  ),
                  const Divider(height: 32),
                  const Text("审批列表", style: TextStyle(fontWeight: FontWeight.bold)),
                  const SizedBox(height: 8),
                  if (_items.isEmpty)
                    const Text("暂无审批记录")
                  else
                    for (final item in _items)
                      Card(
                        margin: const EdgeInsets.symmetric(vertical: 8),
                        child: Padding(
                          padding: const EdgeInsets.all(12),
                          child: Column(
                            crossAxisAlignment: CrossAxisAlignment.start,
                            children: [
                              Text("审批 #${item["id"]}"),
                              Text("需求 ID：${item["demand_id"]}"),
                              Text("状态：${item["status"]}"),
                              Text("时间：${item["created_at"]}"),
                              Text("备注：${item["comment"] ?? "-"}"),
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
