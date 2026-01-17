import "package:flutter/material.dart";
import "package:flutter/services.dart";
import "package:shared_preferences/shared_preferences.dart";

import "../../services/api_client.dart";
import "../demands/demand_create_page.dart";
import "order_detail_page.dart";

class OrderListPage extends StatefulWidget {
  const OrderListPage({super.key});

  @override
  State<OrderListPage> createState() => _OrderListPageState();
}

class _OrderListPageState extends State<OrderListPage> {
  bool _loading = false;
  bool _loadingMore = false;
  List<Map<String, dynamic>> _orders = [];
  String _statusFilter = "all";
  String _sort = "time_desc";
  bool _filtersExpanded = true;
  final TextEditingController _keywordController = TextEditingController();
  final TextEditingController _minAmountController = TextEditingController();
  final TextEditingController _maxAmountController = TextEditingController();
  final TextEditingController _startTimeController = TextEditingController();
  final TextEditingController _endTimeController = TextEditingController();
  DateTime? _startDate;
  DateTime? _endDate;
  int _page = 1;
  bool _hasMore = true;
  static const int _pageSize = 20;
  static const String _prefStatusKey = "order_status_filter";
  static const String _prefSortKey = "order_sort";
  static const String _prefKeywordKey = "order_keyword";
  static const String _prefMinAmountKey = "order_min_amount";
  static const String _prefMaxAmountKey = "order_max_amount";
  static const String _prefStartTimeKey = "order_start_time";
  static const String _prefEndTimeKey = "order_end_time";

  @override
  void initState() {
    super.initState();
    _initPrefs();
  }

  @override
  void dispose() {
    _keywordController.dispose();
    _minAmountController.dispose();
    _maxAmountController.dispose();
    _startTimeController.dispose();
    _endTimeController.dispose();
    super.dispose();
  }

  Future<void> _initPrefs() async {
    final prefs = await SharedPreferences.getInstance();
    _statusFilter = prefs.getString(_prefStatusKey) ?? "all";
    _sort = prefs.getString(_prefSortKey) ?? "time_desc";
    _keywordController.text = prefs.getString(_prefKeywordKey) ?? "";
    _minAmountController.text = prefs.getString(_prefMinAmountKey) ?? "";
    _maxAmountController.text = prefs.getString(_prefMaxAmountKey) ?? "";
    final startRaw = prefs.getString(_prefStartTimeKey) ?? "";
    final endRaw = prefs.getString(_prefEndTimeKey) ?? "";
    _startDate = startRaw.isEmpty ? null : DateTime.tryParse(startRaw);
    _endDate = endRaw.isEmpty ? null : DateTime.tryParse(endRaw);
    _startTimeController.text = _startDate == null ? "" : _formatDate(_startDate!);
    _endTimeController.text = _endDate == null ? "" : _formatDate(_endDate!);
    if (mounted) {
      setState(() {});
    }
    await _load(reset: true);
  }

  Future<void> _persistFilters() async {
    final prefs = await SharedPreferences.getInstance();
    await prefs.setString(_prefStatusKey, _statusFilter);
    await prefs.setString(_prefSortKey, _sort);
    await prefs.setString(_prefKeywordKey, _keywordController.text.trim());
    await prefs.setString(_prefMinAmountKey, _minAmountController.text.trim());
    await prefs.setString(_prefMaxAmountKey, _maxAmountController.text.trim());
    if (_startDate == null) {
      await prefs.remove(_prefStartTimeKey);
    } else {
      await prefs.setString(_prefStartTimeKey, _startDate!.toIso8601String());
    }
    if (_endDate == null) {
      await prefs.remove(_prefEndTimeKey);
    } else {
      await prefs.setString(_prefEndTimeKey, _endDate!.toIso8601String());
    }
  }

  Map<String, String> _buildQueryParams() {
    final params = <String, String>{
      "page": _page.toString(),
      "page_size": _pageSize.toString(),
    };
    if (_statusFilter != "all") {
      params["status"] = _statusFilter;
    }
    final keyword = _keywordController.text.trim();
    if (keyword.isNotEmpty) {
      params["keyword"] = keyword;
    }
    params["sort"] = _sort;
    final minAmount = double.tryParse(_minAmountController.text.trim());
    final maxAmount = double.tryParse(_maxAmountController.text.trim());
    if (minAmount != null) {
      params["min_amount"] = minAmount.toString();
    }
    if (maxAmount != null) {
      params["max_amount"] = maxAmount.toString();
    }
    if (_startDate != null) {
      params["start_time"] = _startDate!.toIso8601String();
    }
    if (_endDate != null) {
      params["end_time"] = _endDate!.toIso8601String();
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
      _orders = [];
    }
    if (reset) {
      setState(() => _loading = true);
    }
    try {
      final params = _buildQueryParams();
      final query = _buildQueryString(params);
      final data = await ApiClient.get("/orders?$query");
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
      } else {
        if (reset) {
          _orders = [];
          _hasMore = false;
        }
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
    if (_loadingMore || !_hasMore) {
      return;
    }
    setState(() => _loadingMore = true);
    await _load();
  }

  void _openDetail(int id) {
    Navigator.of(context).push(MaterialPageRoute(builder: (_) => OrderDetailPage(orderId: id)));
  }

  void _showMessage(String message) {
    ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text(message)));
  }

  String _statusLabel(String status) {
    switch (status) {
      case "confirmed":
        return "已确认";
      case "paid":
        return "已支付";
      case "frozen":
        return "已冻结";
      case "completed":
        return "已完成";
      default:
        return status;
    }
  }

  Color _statusColor(String status) {
    switch (status) {
      case "confirmed":
        return Colors.orange;
      case "paid":
        return Colors.green;
      case "frozen":
        return Colors.red;
      case "completed":
        return Colors.blue;
      default:
        return Colors.grey;
    }
  }

  Widget _buildFilterChips() {
    final options = {
      "all": "全部",
      "confirmed": "已确认",
      "paid": "已支付",
      "frozen": "已冻结",
      "completed": "已完成",
    };
    return Padding(
      padding: const EdgeInsets.fromLTRB(16, 12, 16, 4),
      child: Wrap(
        spacing: 8,
        children: options.entries.map((entry) {
          return ChoiceChip(
            label: Text(entry.value),
            selected: _statusFilter == entry.key,
            onSelected: (_) {
              setState(() => _statusFilter = entry.key);
              _persistFilters();
              _load(reset: true);
            },
          );
        }).toList(),
      ),
    );
  }

  Widget _buildSortChips() {
    final options = {
      "time_desc": "时间新",
      "time_asc": "时间旧",
      "amount_desc": "金额高",
      "amount_asc": "金额低",
    };
    return Padding(
      padding: const EdgeInsets.fromLTRB(16, 0, 16, 8),
      child: Wrap(
        spacing: 8,
        children: options.entries.map((entry) {
          return ChoiceChip(
            label: Text(entry.value),
            selected: _sort == entry.key,
            onSelected: (_) {
              setState(() => _sort = entry.key);
              _persistFilters();
              _load(reset: true);
            },
          );
        }).toList(),
      ),
    );
  }

  Widget _buildSearchBar() {
    return Padding(
      padding: const EdgeInsets.fromLTRB(16, 12, 16, 4),
      child: Row(
        children: [
          Expanded(
            child: TextField(
              controller: _keywordController,
              keyboardType: TextInputType.number,
              decoration: const InputDecoration(
                labelText: "订单 ID",
                prefixIcon: Icon(Icons.search),
              ),
              onSubmitted: (_) {
                _persistFilters();
                _load(reset: true);
              },
            ),
          ),
          const SizedBox(width: 8),
          FilledButton(
            onPressed: () {
              _persistFilters();
              _load(reset: true);
            },
            child: const Text("搜索"),
          ),
        ],
      ),
    );
  }

  Widget _buildAmountFilter() {
    return Padding(
      padding: const EdgeInsets.fromLTRB(16, 0, 16, 8),
      child: Row(
        children: [
          Expanded(
            child: TextField(
              controller: _minAmountController,
              keyboardType: TextInputType.number,
              decoration: const InputDecoration(labelText: "最小金额"),
              onSubmitted: (_) {
                _persistFilters();
                _load(reset: true);
              },
            ),
          ),
          const SizedBox(width: 8),
          Expanded(
            child: TextField(
              controller: _maxAmountController,
              keyboardType: TextInputType.number,
              decoration: const InputDecoration(labelText: "最大金额"),
              onSubmitted: (_) {
                _persistFilters();
                _load(reset: true);
              },
            ),
          ),
        ],
      ),
    );
  }

  Future<void> _pickDate({required bool isStart}) async {
    final now = DateTime.now();
    final initial = now;
    final picked = await showDatePicker(
      context: context,
      firstDate: DateTime(2020),
      lastDate: DateTime(now.year + 1),
      initialDate: initial,
    );
    if (picked == null) {
      return;
    }
    final value = isStart
        ? DateTime(picked.year, picked.month, picked.day, 0, 0, 0)
        : DateTime(picked.year, picked.month, picked.day, 23, 59, 59);
    if (isStart) {
      _startDate = value;
      _startTimeController.text = _formatDate(value);
    } else {
      _endDate = value;
      _endTimeController.text = _formatDate(value);
    }
    await _persistFilters();
    await _load(reset: true);
  }

  Widget _buildTimeFilter() {
    return Padding(
      padding: const EdgeInsets.fromLTRB(16, 0, 16, 8),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          TextField(
            controller: _startTimeController,
            readOnly: true,
            decoration: const InputDecoration(
              labelText: "开始日期",
              suffixIcon: Icon(Icons.date_range),
            ),
            onTap: () => _pickDate(isStart: true),
          ),
          const SizedBox(height: 8),
          TextField(
            controller: _endTimeController,
            readOnly: true,
            decoration: const InputDecoration(
              labelText: "结束日期",
              suffixIcon: Icon(Icons.event),
            ),
            onTap: () => _pickDate(isStart: false),
          ),
          const SizedBox(height: 8),
          Align(
            alignment: Alignment.centerRight,
            child: TextButton(
              onPressed: () {
                _startDate = null;
                _endDate = null;
                _startTimeController.clear();
                _endTimeController.clear();
                _persistFilters();
                _load(reset: true);
              },
              child: const Text("清除日期"),
            ),
          ),
        ],
      ),
    );
  }

  void _resetFilters() {
    _keywordController.clear();
    _minAmountController.clear();
    _maxAmountController.clear();
    _startTimeController.clear();
    _endTimeController.clear();
    _startDate = null;
    _endDate = null;
    _statusFilter = "all";
    _sort = "time_desc";
    SharedPreferences.getInstance().then((prefs) {
      prefs.remove(_prefKeywordKey);
      prefs.remove(_prefMinAmountKey);
      prefs.remove(_prefMaxAmountKey);
      prefs.remove(_prefStartTimeKey);
      prefs.remove(_prefEndTimeKey);
      prefs.setString(_prefStatusKey, _statusFilter);
      prefs.setString(_prefSortKey, _sort);
    });
    _load(reset: true);
  }

  Widget _buildFilterHeader() {
    final summary = _buildSummaryText();
    return Padding(
      padding: const EdgeInsets.fromLTRB(16, 12, 16, 4),
      child: InkWell(
        onTap: () => setState(() => _filtersExpanded = !_filtersExpanded),
        onLongPress: summary.isNotEmpty ? () => _copySummary(summary) : null,
        child: Row(
          children: [
            const Text(
              "筛选条件",
              style: TextStyle(fontWeight: FontWeight.bold),
            ),
            const Spacer(),
            if (!_filtersExpanded && summary.isNotEmpty)
              Expanded(
                child: InkWell(
                  onTap: () => _copySummary(summary),
                  child: Text(
                    summary,
                    textAlign: TextAlign.right,
                    overflow: TextOverflow.ellipsis,
                    style: const TextStyle(color: Colors.grey),
                  ),
                ),
              ),
            Icon(_filtersExpanded ? Icons.expand_less : Icons.expand_more),
          ],
        ),
      ),
    );
  }

  String _buildSummaryText() {
    final parts = <String>[];
    if (_keywordController.text.trim().isNotEmpty) {
      parts.add("订单:${_keywordController.text.trim()}");
    }
    if (_statusFilter != "all") {
      parts.add("状态:${_statusLabel(_statusFilter)}");
    }
    if (_sort != "time_desc") {
      parts.add("排序:${_sortLabel(_sort)}");
    }
    if (_minAmountController.text.trim().isNotEmpty ||
        _maxAmountController.text.trim().isNotEmpty) {
      parts.add("金额:${_minAmountController.text.trim()}-${_maxAmountController.text.trim()}");
    }
    if (_startDate != null || _endDate != null) {
      final start = _startDate == null ? "" : _formatDate(_startDate!);
      final end = _endDate == null ? "" : _formatDate(_endDate!);
      parts.add("日期:$start~$end");
    }
    return parts.join(" | ");
  }

  String _sortLabel(String sort) {
    switch (sort) {
      case "time_desc":
        return "时间新";
      case "time_asc":
        return "时间旧";
      case "amount_desc":
        return "金额高";
      case "amount_asc":
        return "金额低";
      default:
        return sort;
    }
  }

  String _formatDate(DateTime date) {
    final year = date.year.toString().padLeft(4, "0");
    final month = date.month.toString().padLeft(2, "0");
    final day = date.day.toString().padLeft(2, "0");
    return "$year-$month-$day";
  }

  Future<void> _copyDebugUrl() async {
    final params = _buildQueryParams();
    final query = _buildQueryString(params);
    final url = "${ApiClient.baseUrl}/orders?$query";
    await Clipboard.setData(ClipboardData(text: url));
    if (mounted) {
      ScaffoldMessenger.of(context).showSnackBar(const SnackBar(content: Text("已复制筛选链接")));
    }
  }

  Future<void> _copySummary(String summary) async {
    await Clipboard.setData(ClipboardData(text: summary));
    if (mounted) {
      ScaffoldMessenger.of(context)
          .showSnackBar(const SnackBar(content: Text("已复制筛选摘要")));
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text("我的订单"),
        actions: [
          IconButton(onPressed: _copyDebugUrl, icon: const Icon(Icons.link)),
          IconButton(onPressed: _resetFilters, icon: const Icon(Icons.filter_alt_off)),
          IconButton(onPressed: () => _load(reset: true), icon: const Icon(Icons.refresh)),
        ],
      ),
      body: _loading
          ? const Center(child: CircularProgressIndicator())
          : RefreshIndicator(
              onRefresh: () => _load(reset: true),
              child: ListView(
                children: [
                  _buildFilterHeader(),
                  AnimatedCrossFade(
                    firstChild: Column(
                      children: [
                        _buildSearchBar(),
                        _buildFilterChips(),
                        _buildSortChips(),
                        _buildAmountFilter(),
                        _buildTimeFilter(),
                      ],
                    ),
                    secondChild: const SizedBox.shrink(),
                    crossFadeState: _filtersExpanded
                        ? CrossFadeState.showFirst
                        : CrossFadeState.showSecond,
                    duration: const Duration(milliseconds: 200),
                  ),
                  if (_orders.isEmpty)
                    Padding(
                      padding: const EdgeInsets.only(top: 80),
                      child: Column(
                        children: [
                          Icon(
                            Icons.receipt_long,
                            size: 72,
                            color: Colors.grey.shade400,
                          ),
                          const SizedBox(height: 12),
                          const Text("暂无订单", style: TextStyle(fontSize: 16)),
                          const SizedBox(height: 8),
                          Text(
                            "试试调整筛选条件或发布需求",
                            style: TextStyle(color: Colors.grey.shade600),
                          ),
                          const SizedBox(height: 16),
                          Row(
                            mainAxisAlignment: MainAxisAlignment.center,
                            children: [
                              FilledButton(
                                onPressed: () {
                                  _resetFilters();
                                  Navigator.of(context).push(
                                    MaterialPageRoute(
                                      builder: (_) => const DemandCreatePage(),
                                    ),
                                  );
                                },
                                child: const Text("发布需求"),
                              ),
                              const SizedBox(width: 12),
                              OutlinedButton(
                                onPressed: _resetFilters,
                                child: const Text("清空筛选"),
                              ),
                            ],
                          ),
                        ],
                      ),
                    )
                  else
                    ListView.separated(
                      padding: const EdgeInsets.all(16),
                      shrinkWrap: true,
                      physics: const NeverScrollableScrollPhysics(),
                      itemBuilder: (context, index) {
                        final order = _orders[index];
                        final status = order["status"]?.toString() ?? "-";
                        return ListTile(
                          tileColor: Colors.white,
                          title: Text("订单 #${order["id"]}"),
                          subtitle: Text("金额：${order["total_amount"]}"),
                          trailing: Chip(
                            label: Text(_statusLabel(status)),
                            backgroundColor: _statusColor(status).withValues(alpha: 0.12),
                            labelStyle: TextStyle(color: _statusColor(status)),
                          ),
                          onTap: () => _openDetail(order["id"] as int),
                        );
                      },
                      separatorBuilder: (_, __) => const SizedBox(height: 12),
                      itemCount: _orders.length,
                    ),
                  if (_hasMore)
                    Padding(
                      padding: const EdgeInsets.only(bottom: 24),
                      child: Center(
                        child: TextButton(
                          onPressed: _loadingMore ? null : _loadMore,
                          child: Text(_loadingMore ? "加载中..." : "加载更多"),
                        ),
                      ),
                    ),
                ],
              ),
            ),
    );
  }
}
