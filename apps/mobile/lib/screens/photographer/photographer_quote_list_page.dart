import "package:flutter/material.dart";

import "../../services/api_client.dart";
import "photographer_quote_detail_page.dart";

class PhotographerQuoteListPage extends StatefulWidget {
  const PhotographerQuoteListPage({super.key});

  @override
  State<PhotographerQuoteListPage> createState() => _PhotographerQuoteListPageState();
}

class _PhotographerQuoteListPageState extends State<PhotographerQuoteListPage> {
  bool _loading = false;
  bool _loadingMore = false;
  List<Map<String, dynamic>> _quotes = [];
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
      _quotes = [];
      setState(() => _loading = true);
    } else {
      setState(() => _loadingMore = true);
    }
    try {
      final statusQuery = _status == "all" ? "" : "&status=$_status";
      final data = await ApiClient.get(
        "/quotes/mine?page=$_page&page_size=$_pageSize$statusQuery",
      );
      if (data is Map<String, dynamic>) {
        final rawItems = data["items"];
        final list = rawItems is List
            ? rawItems.cast<Map<String, dynamic>>()
            : <Map<String, dynamic>>[];
        final total = (data["total"] as num?)?.toInt() ?? list.length;
        if (reset) {
          _quotes = list;
        } else {
          _quotes.addAll(list);
        }
        if (list.isEmpty) {
          _hasMore = false;
        } else {
          _hasMore = _quotes.length < total;
          if (_hasMore) {
            _page += 1;
          }
        }
      } else if (data is List) {
        final list = data.cast<Map<String, dynamic>>();
        if (reset) {
          _quotes = list;
        } else {
          _quotes.addAll(list);
        }
        _hasMore = list.length == _pageSize;
        if (_hasMore) {
          _page += 1;
        }
      } else if (reset) {
        _quotes = [];
        _hasMore = false;
      }
    } catch (error) {
      if (reset) {
        _quotes = [];
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

  void _openDetail(int quoteId) {
    Navigator.of(context).push(
      MaterialPageRoute(builder: (_) => PhotographerQuoteDetailPage(quoteId: quoteId)),
    );
  }

  void _showMessage(String message) {
    ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text(message)));
  }

  String _statusHint(String status) {
    switch (status) {
      case "pending":
        return "待客户确认";
      case "accepted":
        return "已转订单";
      case "expired":
        return "已过期/撤回";
      default:
        return "状态更新中";
    }
  }

  String _orderStatusLabel(String status) {
    switch (status) {
      case "confirmed":
        return "已确认";
      case "paid":
        return "已支付";
      case "ongoing":
        return "进行中";
      case "completed":
        return "已完成";
      case "reviewed":
        return "已评价";
      case "cancelled":
        return "已取消";
      case "frozen":
        return "已冻结";
      default:
        return "未知";
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text("我的报价"),
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
                DropdownMenuItem(value: "pending", child: Text("待处理")),
                DropdownMenuItem(value: "accepted", child: Text("已接受")),
                DropdownMenuItem(value: "expired", child: Text("已过期")),
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
                    child: _quotes.isEmpty
                        ? ListView(
                            children: const [
                              SizedBox(height: 120),
                              Center(child: Text("暂无报价")),
                            ],
                          )
                        : ListView.separated(
                            controller: _scrollController,
                            padding: const EdgeInsets.all(16),
                            itemBuilder: (context, index) {
                              if (index >= _quotes.length) {
                                return const Padding(
                                  padding: EdgeInsets.symmetric(vertical: 12),
                                  child: Center(child: CircularProgressIndicator()),
                                );
                              }
                              final quote = _quotes[index];
                              final id = quote["id"] as int? ?? 0;
                              return ListTile(
                                tileColor: Colors.white,
                                title: Text("报价 #$id"),
                                subtitle: Text(
                                  "需求 ID：${quote["demand_id"] ?? "-"}\n"
                                  "状态：${quote["status"] ?? "-"}（${_statusHint(quote["status"]?.toString() ?? "")}）\n"
                                  "总价：${quote["total_price"] ?? "-"}\n"
                                  "订单 ID：${quote["order_id"] ?? "-"}\n"
                                  "订单状态：${quote["order_status"] == null ? "-" : _orderStatusLabel(quote["order_status"].toString())}\n"
                                  "时间：${quote["created_at"] ?? "-"}",
                                ),
                                trailing: const Icon(Icons.chevron_right),
                                onTap: () => _openDetail(id),
                              );
                            },
                            separatorBuilder: (_, __) => const SizedBox(height: 12),
                            itemCount: _quotes.length + (_loadingMore ? 1 : 0),
                          ),
                  ),
          ),
        ],
      ),
    );
  }
}
