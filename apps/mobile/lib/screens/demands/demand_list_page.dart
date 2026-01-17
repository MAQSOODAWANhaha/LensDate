import "package:flutter/material.dart";

import "../../services/api_client.dart";
import "demand_create_page.dart";
import "demand_detail_page.dart";
import "../photographer/photographer_list_page.dart";

class DemandListPage extends StatefulWidget {
  const DemandListPage({super.key});

  @override
  State<DemandListPage> createState() => _DemandListPageState();
}

class _DemandListPageState extends State<DemandListPage> {
  bool _checkingRole = true;
  bool _isPhotographer = false;

  @override
  void initState() {
    super.initState();
    _checkRole();
  }

  Future<void> _checkRole() async {
    try {
      final data = await ApiClient.get("/photographers/me");
      if (data is Map<String, dynamic>) {
        _isPhotographer = true;
      }
    } catch (_) {
      _isPhotographer = false;
    } finally {
      if (mounted) {
        setState(() => _checkingRole = false);
      }
    }
  }

  Future<void> _openCreate() async {
    await Navigator.of(context).push(MaterialPageRoute(builder: (_) => const DemandCreatePage()));
  }

  @override
  Widget build(BuildContext context) {
    if (_checkingRole) {
      return Scaffold(
        appBar: AppBar(title: const Text("需求")),
        body: const Center(child: CircularProgressIndicator()),
      );
    }

    if (_isPhotographer) {
      return DefaultTabController(
        length: 2,
        child: Scaffold(
          appBar: AppBar(
            title: const Text("需求"),
            bottom: const TabBar(
              tabs: [
                Tab(text: "我的需求"),
                Tab(text: "需求广场"),
              ],
            ),
          ),
          floatingActionButton: FloatingActionButton(
            onPressed: _openCreate,
            child: const Icon(Icons.add),
          ),
          body: const TabBarView(
            children: [
              DemandListTab(mine: true),
              DemandListTab(mine: false),
            ],
          ),
        ),
      );
    }

    return DefaultTabController(
      length: 2,
      child: Scaffold(
        appBar: AppBar(
          title: const Text("需求"),
          bottom: const TabBar(
            tabs: [
              Tab(text: "我的需求"),
              Tab(text: "摄影师"),
            ],
          ),
        ),
        floatingActionButton: FloatingActionButton(
          onPressed: _openCreate,
          child: const Icon(Icons.add),
        ),
        body: const TabBarView(
          children: [
            DemandListTab(mine: true),
            PhotographerListPage(embedded: true),
          ],
        ),
      ),
    );
  }
}

class DemandListTab extends StatefulWidget {
  final bool mine;
  const DemandListTab({super.key, required this.mine});

  @override
  State<DemandListTab> createState() => _DemandListTabState();
}

class _DemandListTabState extends State<DemandListTab> {
  bool _loading = false;
  bool _loadingMore = false;
  List<Map<String, dynamic>> _items = [];
  int _page = 1;
  bool _hasMore = true;
  static const int _pageSize = 20;
  final ScrollController _scrollController = ScrollController();

  final TextEditingController _typeController = TextEditingController();
  final TextEditingController _cityController = TextEditingController();
  final TextEditingController _startController = TextEditingController();
  final TextEditingController _endController = TextEditingController();
  final TextEditingController _minBudgetController = TextEditingController();
  final TextEditingController _maxBudgetController = TextEditingController();
  final TextEditingController _styleTagController = TextEditingController();
  DateTime? _startDate;
  DateTime? _endDate;
  String _status = "all";
  String _merchantFilter = "all";
  String _sort = "time_desc";
  bool _filtersExpanded = false;

  @override
  void initState() {
    super.initState();
    if (!widget.mine) {
      _status = "open";
    }
    _scrollController.addListener(_onScroll);
    _load(reset: true);
  }

  @override
  void dispose() {
    _scrollController.removeListener(_onScroll);
    _scrollController.dispose();
    _typeController.dispose();
    _cityController.dispose();
    _startController.dispose();
    _endController.dispose();
    _minBudgetController.dispose();
    _maxBudgetController.dispose();
    _styleTagController.dispose();
    super.dispose();
  }

  Future<void> _pickStartDate() async {
    final now = DateTime.now();
    final picked = await showDatePicker(
      context: context,
      firstDate: DateTime(now.year - 1, 1, 1),
      lastDate: DateTime(now.year + 2, 12, 31),
      initialDate: _startDate ?? now,
    );
    if (picked != null) {
      setState(() {
        _startDate = DateTime(picked.year, picked.month, picked.day);
        _startController.text = _formatDate(_startDate!);
      });
    }
  }

  Future<void> _pickEndDate() async {
    final now = DateTime.now();
    final picked = await showDatePicker(
      context: context,
      firstDate: DateTime(now.year - 1, 1, 1),
      lastDate: DateTime(now.year + 2, 12, 31),
      initialDate: _endDate ?? now,
    );
    if (picked != null) {
      setState(() {
        _endDate = DateTime(picked.year, picked.month, picked.day);
        _endController.text = _formatDate(_endDate!);
      });
    }
  }

  void _onScroll() {
    if (!_hasMore || _loadingMore || _loading) {
      return;
    }
    if (_scrollController.position.pixels >=
        _scrollController.position.maxScrollExtent - 160) {
      _loadMore();
    }
  }

  String _formatDate(DateTime date) {
    final month = date.month.toString().padLeft(2, "0");
    final day = date.day.toString().padLeft(2, "0");
    return "${date.year}-$month-$day";
  }

  Map<String, String> _buildQueryParams() {
    final params = <String, String>{
      "page": _page.toString(),
      "page_size": _pageSize.toString(),
      "sort": _sort,
    };
    final type = _typeController.text.trim();
    final cityId = int.tryParse(_cityController.text.trim());
    final minBudget = double.tryParse(_minBudgetController.text.trim());
    final maxBudget = double.tryParse(_maxBudgetController.text.trim());
    final styleTag = _styleTagController.text
        .split(",")
        .map((e) => e.trim())
        .where((e) => e.isNotEmpty)
        .toList();
    if (type.isNotEmpty) {
      params["type"] = type;
    }
    if (cityId != null) {
      params["city_id"] = cityId.toString();
    }
    if (_startDate != null) {
      params["schedule_start"] = _startDate!.toIso8601String();
    }
    if (_endDate != null) {
      params["schedule_end"] = _endDate!.toIso8601String();
    }
    if (minBudget != null) {
      params["min_budget"] = minBudget.toString();
    }
    if (maxBudget != null) {
      params["max_budget"] = maxBudget.toString();
    }
    if (styleTag.isNotEmpty) {
      params["style_tag"] = styleTag.first;
    }
    if (_merchantFilter != "all") {
      params["is_merchant"] = (_merchantFilter == "merchant").toString();
    }
    if (widget.mine) {
      params["mine"] = "true";
    }
    if (_status != "all") {
      params["status"] = _status;
    }
    return params;
  }

  String _buildQueryString(Map<String, String> params) {
    return params.entries
        .map((e) => "${e.key}=${Uri.encodeQueryComponent(e.value)}")
        .join("&");
  }

  Future<void> _load({required bool reset}) async {
    if (reset) {
      _page = 1;
      _hasMore = true;
      _items = [];
      setState(() => _loading = true);
    } else {
      setState(() => _loadingMore = true);
    }
    try {
      final query = _buildQueryString(_buildQueryParams());
      final data = await ApiClient.get("/demands?$query");
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
    if (_loadingMore || !_hasMore) {
      return;
    }
    await _load(reset: false);
  }

  void _resetFilters() {
    _typeController.clear();
    _cityController.clear();
    _startController.clear();
    _startDate = null;
    _endController.clear();
    _endDate = null;
    _minBudgetController.clear();
    _maxBudgetController.clear();
    _styleTagController.clear();
    _merchantFilter = "all";
    _sort = "time_desc";
    _status = widget.mine ? "all" : "open";
    setState(() {});
    _load(reset: true);
  }

  void _openDetail(int id) {
    Navigator.of(context).push(MaterialPageRoute(builder: (_) => DemandDetailPage(demandId: id)));
  }

  void _showMessage(String message) {
    ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text(message)));
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        ExpansionPanelList(
          expansionCallback: (_, __) => setState(() => _filtersExpanded = !_filtersExpanded),
          children: [
            ExpansionPanel(
              isExpanded: _filtersExpanded,
              headerBuilder: (_, __) => const ListTile(title: Text("筛选条件")),
              body: Padding(
                padding: const EdgeInsets.all(16),
                child: Column(
                  children: [
                    TextField(
                      controller: _typeController,
                      decoration: const InputDecoration(labelText: "类型"),
                    ),
                    TextField(
                      controller: _cityController,
                      decoration: const InputDecoration(labelText: "城市 ID"),
                      keyboardType: TextInputType.number,
                    ),
                    TextField(
                      controller: _minBudgetController,
                      decoration: const InputDecoration(labelText: "预算下限（可选）"),
                      keyboardType: TextInputType.number,
                    ),
                    TextField(
                      controller: _maxBudgetController,
                      decoration: const InputDecoration(labelText: "预算上限（可选）"),
                      keyboardType: TextInputType.number,
                    ),
                    TextField(
                      controller: _styleTagController,
                      decoration: const InputDecoration(labelText: "风格标签（可选，逗号分隔）"),
                    ),
                    TextField(
                      controller: _startController,
                      decoration: const InputDecoration(labelText: "开始日期（可选）"),
                      readOnly: true,
                      onTap: _pickStartDate,
                    ),
                    TextField(
                      controller: _endController,
                      decoration: const InputDecoration(labelText: "结束日期（可选）"),
                      readOnly: true,
                      onTap: _pickEndDate,
                    ),
                    DropdownButtonFormField<String>(
                      key: ValueKey(_merchantFilter),
                      initialValue: _merchantFilter,
                      decoration: const InputDecoration(labelText: "需求类型"),
                      items: const [
                        DropdownMenuItem(value: "all", child: Text("全部")),
                        DropdownMenuItem(value: "merchant", child: Text("商户需求")),
                        DropdownMenuItem(value: "personal", child: Text("个人需求")),
                      ],
                      onChanged: (value) => setState(() => _merchantFilter = value ?? "all"),
                    ),
                    DropdownButtonFormField<String>(
                      key: ValueKey(_sort),
                      initialValue: _sort,
                      decoration: const InputDecoration(labelText: "排序"),
                      items: const [
                        DropdownMenuItem(value: "time_desc", child: Text("时间最新")),
                        DropdownMenuItem(value: "time_asc", child: Text("时间最早")),
                        DropdownMenuItem(value: "budget_desc", child: Text("预算从高到低")),
                        DropdownMenuItem(value: "budget_asc", child: Text("预算从低到高")),
                      ],
                      onChanged: (value) => setState(() => _sort = value ?? "time_desc"),
                    ),
                    if (widget.mine)
                      DropdownButtonFormField<String>(
                        key: ValueKey(_status),
                        initialValue: _status,
                        decoration: const InputDecoration(labelText: "状态"),
                        items: const [
                          DropdownMenuItem(value: "all", child: Text("全部")),
                          DropdownMenuItem(value: "draft", child: Text("草稿")),
                          DropdownMenuItem(value: "open", child: Text("开放")),
                          DropdownMenuItem(value: "closed", child: Text("已关闭")),
                        ],
                        onChanged: (value) => setState(() => _status = value ?? "all"),
                      )
                    else
                      const Padding(
                        padding: EdgeInsets.only(top: 12),
                        child: Align(
                          alignment: Alignment.centerLeft,
                          child: Text("状态：开放"),
                        ),
                      ),
                    const SizedBox(height: 12),
                    Row(
                      children: [
                        Expanded(
                          child: OutlinedButton(
                            onPressed: _resetFilters,
                            child: const Text("重置"),
                          ),
                        ),
                        const SizedBox(width: 12),
                        Expanded(
                          child: FilledButton(
                            onPressed: () => _load(reset: true),
                            child: const Text("筛选"),
                          ),
                        ),
                      ],
                    )
                  ],
                ),
              ),
            ),
          ],
        ),
        Expanded(
          child: _loading
              ? const Center(child: CircularProgressIndicator())
              : RefreshIndicator(
                  onRefresh: () => _load(reset: true),
                  child: _items.isEmpty
                      ? ListView(
                          controller: _scrollController,
                          children: const [
                            SizedBox(height: 120),
                            Center(child: Text("暂无需求")),
                          ],
                        )
                      : ListView.builder(
                          controller: _scrollController,
                          padding: const EdgeInsets.all(16),
                          itemCount: _items.length + (_loadingMore ? 1 : 0),
                          itemBuilder: (context, index) {
                            if (index >= _items.length) {
                              return const Padding(
                                padding: EdgeInsets.symmetric(vertical: 16),
                                child: Center(child: CircularProgressIndicator()),
                              );
                            }
                            final item = _items[index];
                            final id = item["id"] as int? ?? 0;
                            return Padding(
                              padding: const EdgeInsets.only(bottom: 12),
                              child: ListTile(
                                tileColor: Colors.white,
                                title: Text(item["type"]?.toString() ?? "需求"),
                                subtitle: Text(
                                  "状态：${item["status"] ?? "-"}\n城市：${item["city_id"] ?? "-"}\n时间：${item["schedule_start"] ?? "-"}",
                                ),
                                trailing: const Icon(Icons.chevron_right),
                                onTap: () => _openDetail(id),
                              ),
                            );
                          },
                        ),
                ),
        ),
      ],
    );
  }
}
