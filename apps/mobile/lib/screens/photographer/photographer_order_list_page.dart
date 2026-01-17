import "package:flutter/material.dart";

import "../../services/api_client.dart";
import "photographer_order_detail_page.dart";

class PhotographerOrderListPage extends StatefulWidget {
  const PhotographerOrderListPage({super.key});

  @override
  State<PhotographerOrderListPage> createState() => _PhotographerOrderListPageState();
}

class _PhotographerOrderListPageState extends State<PhotographerOrderListPage> {
  bool _loading = false;
  bool _loadingMore = false;
  List<Map<String, dynamic>> _orders = [];
  String _status = "all";
  int _page = 1;
  bool _hasMore = true;
  static const int _pageSize = 20;
  final ScrollController _scrollController = ScrollController();

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
      _orders = [];
      setState(() => _loading = true);
    } else {
      setState(() => _loadingMore = true);
    }
    try {
      final statusQuery = _status == "all" ? "" : "&status=$_status";
      final data = await ApiClient.get(
        "/photographers/me/orders?page=$_page&page_size=$_pageSize$statusQuery",
      );
      if (data is Map<String, dynamic>) {
        final rawItems = data["items"];
        final list = rawItems is List
            ? rawItems.cast<Map<String, dynamic>>()
            : <Map<String, dynamic>>[];
        final total = (data["total"] as num?)?.toInt() ?? list.length;
        if (reset) {
          _orders = list;
        } else {
          _orders.addAll(list);
        }
        if (list.isEmpty) {
          _hasMore = false;
        } else {
          _hasMore = _orders.length < total;
          if (_hasMore) {
            _page += 1;
          }
        }
      } else if (data is List) {
        final list = data.cast<Map<String, dynamic>>();
        if (reset) {
          _orders = list;
        } else {
          _orders.addAll(list);
        }
        _hasMore = list.length == _pageSize;
        if (_hasMore) {
          _page += 1;
        }
      } else if (reset) {
        _orders = [];
        _hasMore = false;
      }
    } catch (error) {
      if (reset) {
        _orders = [];
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

  void _openDetail(int orderId) {
    Navigator.of(context).push(
      MaterialPageRoute(builder: (_) => PhotographerOrderDetailPage(orderId: orderId)),
    );
  }

  void _showMessage(String message) {
    ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text(message)));
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text("摄影师订单"),
        actions: [
          IconButton(onPressed: () => _load(reset: true), icon: const Icon(Icons.refresh))
        ],
      ),
      body: Column(
        children: [
          Padding(
            padding: const EdgeInsets.all(12),
            child: DropdownButtonFormField<String>(
              key: ValueKey(_status),
              initialValue: _status,
              decoration: const InputDecoration(labelText: "状态筛选"),
              items: const [
                DropdownMenuItem(value: "all", child: Text("全部")),
                DropdownMenuItem(value: "confirmed", child: Text("已确认")),
                DropdownMenuItem(value: "paid", child: Text("已支付")),
                DropdownMenuItem(value: "ongoing", child: Text("进行中")),
                DropdownMenuItem(value: "completed", child: Text("已完成")),
                DropdownMenuItem(value: "frozen", child: Text("已冻结")),
              ],
              onChanged: (value) {
                setState(() => _status = value ?? "all");
                _load(reset: true);
              },
            ),
          ),
          Expanded(
            child: _loading
                ? const Center(child: CircularProgressIndicator())
                : RefreshIndicator(
                    onRefresh: () => _load(reset: true),
                    child: _orders.isEmpty
                        ? ListView(
                            children: const [
                              SizedBox(height: 120),
                              Center(child: Text("暂无订单")),
                            ],
                          )
                        : ListView.separated(
                            controller: _scrollController,
                            padding: const EdgeInsets.all(16),
                            itemBuilder: (context, index) {
                              if (index >= _orders.length) {
                                return const Padding(
                                  padding: EdgeInsets.symmetric(vertical: 12),
                                  child: Center(child: CircularProgressIndicator()),
                                );
                              }
                              final item = _orders[index];
                              final id = item["id"] as int? ?? 0;
                              return ListTile(
                                tileColor: Colors.white,
                                title: Text("订单 #$id"),
                                subtitle: Text(
                                  "状态：${item["status"] ?? "-"}\n金额：${item["total_amount"] ?? "-"}\n时间：${item["created_at"] ?? "-"}",
                                ),
                                trailing: const Icon(Icons.chevron_right),
                                onTap: () => _openDetail(id),
                              );
                            },
                            separatorBuilder: (_, __) => const SizedBox(height: 12),
                            itemCount: _orders.length + (_loadingMore ? 1 : 0),
                          ),
                  ),
          ),
        ],
      ),
    );
  }
}
