import "package:flutter/material.dart";

import "../../services/api_client.dart";

class MerchantInvoiceListPage extends StatefulWidget {
  final int merchantId;
  final String merchantName;
  const MerchantInvoiceListPage({super.key, required this.merchantId, required this.merchantName});

  @override
  State<MerchantInvoiceListPage> createState() => _MerchantInvoiceListPageState();
}

class _MerchantInvoiceListPageState extends State<MerchantInvoiceListPage> {
  bool _loading = false;
  bool _loadingMore = false;
  bool _creating = false;
  List<Map<String, dynamic>> _items = [];
  int _page = 1;
  bool _hasMore = true;
  static const int _pageSize = 20;
  final TextEditingController _orderController = TextEditingController();
  final TextEditingController _titleController = TextEditingController();
  final TextEditingController _taxController = TextEditingController();
  final TextEditingController _amountController = TextEditingController();
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
    _titleController.dispose();
    _taxController.dispose();
    _amountController.dispose();
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
        "/merchant-invoices?merchant_id=${widget.merchantId}&page=$_page&page_size=$_pageSize",
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

  Future<void> _createInvoice() async {
    final title = _titleController.text.trim();
    final amount = double.tryParse(_amountController.text.trim());
    if (title.isEmpty || amount == null) {
      _showMessage("请填写抬头和金额");
      return;
    }
    final orderId = int.tryParse(_orderController.text.trim());
    setState(() => _creating = true);
    try {
      await ApiClient.post("/merchant-invoices", {
        "merchant_id": widget.merchantId,
        "order_id": orderId,
        "title": title,
        "tax_no": _taxController.text.trim().isEmpty ? null : _taxController.text.trim(),
        "amount": amount
      });
      _orderController.clear();
      _titleController.clear();
      _taxController.clear();
      _amountController.clear();
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
        title: Text("${widget.merchantName} 发票"),
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
                  const Text("新增发票", style: TextStyle(fontWeight: FontWeight.bold)),
                  const SizedBox(height: 8),
                  TextField(
                    controller: _orderController,
                    decoration: const InputDecoration(labelText: "订单 ID（可选）"),
                    keyboardType: TextInputType.number,
                  ),
                  TextField(
                    controller: _titleController,
                    decoration: const InputDecoration(labelText: "发票抬头"),
                  ),
                  TextField(
                    controller: _taxController,
                    decoration: const InputDecoration(labelText: "税号（可选）"),
                  ),
                  TextField(
                    controller: _amountController,
                    decoration: const InputDecoration(labelText: "开票金额"),
                    keyboardType: TextInputType.number,
                  ),
                  const SizedBox(height: 8),
                  FilledButton(
                    onPressed: _creating ? null : _createInvoice,
                    child: Text(_creating ? "提交中..." : "创建发票"),
                  ),
                  const Divider(height: 32),
                  const Text("发票列表", style: TextStyle(fontWeight: FontWeight.bold)),
                  const SizedBox(height: 8),
                  if (_items.isEmpty)
                    const Text("暂无发票")
                  else
                    for (final item in _items)
                      Card(
                        margin: const EdgeInsets.symmetric(vertical: 8),
                        child: Padding(
                          padding: const EdgeInsets.all(12),
                          child: Column(
                            crossAxisAlignment: CrossAxisAlignment.start,
                            children: [
                              Text("发票 #${item["id"]}"),
                              Text("金额：${item["amount"]}"),
                              Text("状态：${item["status"]}"),
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
