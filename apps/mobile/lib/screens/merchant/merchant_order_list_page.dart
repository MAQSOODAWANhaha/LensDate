import "package:flutter/material.dart";
import "package:flutter/services.dart";

import "../../services/api_client.dart";
import "merchant_order_detail_page.dart";

class MerchantOrderListPage extends StatefulWidget {
  final int merchantId;
  final String merchantName;
  const MerchantOrderListPage({super.key, required this.merchantId, required this.merchantName});

  @override
  State<MerchantOrderListPage> createState() => _MerchantOrderListPageState();
}

class _MerchantOrderListPageState extends State<MerchantOrderListPage> {
  bool _loading = false;
  bool _loadingMore = false;
  List<Map<String, dynamic>> _orders = [];
  String _status = "all";
  DateTimeRange? _reportRange;
  bool _exporting = false;
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
        "/merchants/orders?merchant_id=${widget.merchantId}&page=$_page&page_size=$_pageSize$statusQuery",
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
      MaterialPageRoute(builder: (_) => MerchantOrderDetailPage(orderId: orderId)),
    );
  }

  void _showMessage(String message) {
    ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text(message)));
  }

  String _formatDate(DateTime date) {
    final month = date.month.toString().padLeft(2, "0");
    final day = date.day.toString().padLeft(2, "0");
    return "${date.year}-$month-$day";
  }

  Future<void> _pickReportRange() async {
    final now = DateTime.now();
    final picked = await showDateRangePicker(
      context: context,
      firstDate: DateTime(now.year - 1, 1, 1),
      lastDate: DateTime(now.year + 2, 12, 31),
      initialDateRange: _reportRange ??
          DateTimeRange(
            start: DateTime(now.year, now.month, 1),
            end: now,
          ),
    );
    if (picked != null) {
      setState(() => _reportRange = picked);
    }
  }

  Future<void> _exportReport() async {
    if (_exporting) {
      return;
    }
    setState(() => _exporting = true);
    try {
      final params = <String, String>{
        "merchant_id": widget.merchantId.toString(),
        "format": "csv",
        "limit": "500"
      };
      if (_status != "all") {
        params["status"] = _status;
      }
      if (_reportRange != null) {
        params["start_date"] = _formatDate(_reportRange!.start);
        params["end_date"] = _formatDate(_reportRange!.end);
      }
      final query = params.entries
          .map((e) => "${e.key}=${Uri.encodeQueryComponent(e.value)}")
          .join("&");
      final data = await ApiClient.get("/merchants/reports/orders?$query");
      if (data is Map<String, dynamic>) {
        final csv = data["csv"]?.toString() ?? "";
        if (csv.isEmpty) {
          _showMessage("暂无可导出的对账数据");
        } else {
          await Clipboard.setData(ClipboardData(text: csv));
          _showMessage("对账单已复制到剪贴板");
        }
      }
    } catch (error) {
      _showMessage("导出失败：$error");
    } finally {
      if (mounted) {
        setState(() => _exporting = false);
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text("${widget.merchantName} 订单"),
        actions: [
          IconButton(onPressed: () => _load(reset: true), icon: const Icon(Icons.refresh))
        ],
      ),
      body: Column(
        children: [
          Padding(
            padding: const EdgeInsets.all(12),
            child: Column(
              children: [
                DropdownButtonFormField<String>(
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
                const SizedBox(height: 8),
                Row(
                  children: [
                    Expanded(
                      child: OutlinedButton(
                        onPressed: _pickReportRange,
                        child: Text(
                          _reportRange == null
                              ? "选择对账日期"
                              : "${_formatDate(_reportRange!.start)} ~ ${_formatDate(_reportRange!.end)}",
                          overflow: TextOverflow.ellipsis,
                        ),
                      ),
                    ),
                    const SizedBox(width: 8),
                    FilledButton(
                      onPressed: _exporting ? null : _exportReport,
                      child: Text(_exporting ? "导出中..." : "导出对账单"),
                    ),
                  ],
                ),
              ],
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
