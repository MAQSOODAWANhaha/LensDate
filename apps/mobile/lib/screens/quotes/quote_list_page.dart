import "package:flutter/material.dart";

import "../../services/api_client.dart";
import "../orders/order_detail_page.dart";

class QuoteListPage extends StatefulWidget {
  final int demandId;
  const QuoteListPage({super.key, required this.demandId});

  @override
  State<QuoteListPage> createState() => _QuoteListPageState();
}

class _QuoteListPageState extends State<QuoteListPage> {
  bool _loading = false;
  bool _loadingMore = false;
  List<Map<String, dynamic>> _quotes = [];
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
      final data = await ApiClient.get(
        "/quotes?demand_id=${widget.demandId}&page=$_page&page_size=$_pageSize",
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

  Future<void> _acceptQuote(int quoteId) async {
    try {
      final data = await ApiClient.post("/quotes/$quoteId/accept", {});
      final orderId = data["order_id"] as int?;
      if (orderId != null && mounted) {
        Navigator.of(context).push(
          MaterialPageRoute(builder: (_) => OrderDetailPage(orderId: orderId)),
        );
      }
    } catch (error) {
      _showMessage("接受失败：$error");
    }
  }

  void _showMessage(String message) {
    ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text(message)));
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text("报价列表"),
        actions: [
          IconButton(onPressed: () => _load(reset: true), icon: const Icon(Icons.refresh))
        ],
      ),
      body: _loading
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
                  : ListView.builder(
                      controller: _scrollController,
                      itemCount: _quotes.length + (_loadingMore ? 1 : 0),
                      itemBuilder: (context, index) {
                        if (index >= _quotes.length) {
                          return const Padding(
                            padding: EdgeInsets.symmetric(vertical: 12),
                            child: Center(child: CircularProgressIndicator()),
                          );
                        }
                        final quote = _quotes[index];
                        final items = quote["items"] is List ? quote["items"] as List : [];
                        return Card(
                          margin: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
                          child: Padding(
                            padding: const EdgeInsets.all(12),
                            child: Column(
                              crossAxisAlignment: CrossAxisAlignment.start,
                              children: [
                                Text("报价 #${quote["id"]}"),
                                Text("状态：${quote["status"]}"),
                                Text("总价：${quote["total_price"]}"),
                                const SizedBox(height: 8),
                                const Text("报价条目："),
                                for (final item in items)
                                  Text("- ${item["name"]} x${item["quantity"]} ¥${item["price"]}"),
                                const SizedBox(height: 8),
                                Align(
                                  alignment: Alignment.centerRight,
                                  child: FilledButton(
                                    onPressed: () => _acceptQuote(quote["id"] as int),
                                    child: const Text("接受报价"),
                                  ),
                                )
                              ],
                            ),
                          ),
                        );
                      },
                    ),
            ),
    );
  }
}
