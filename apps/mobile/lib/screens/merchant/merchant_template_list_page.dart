import "dart:convert";

import "package:flutter/material.dart";

import "../../services/api_client.dart";

class _TemplateItemDraft {
  final TextEditingController name = TextEditingController();
  final TextEditingController quantity = TextEditingController(text: "1");
  final TextEditingController price = TextEditingController();

  void dispose() {
    name.dispose();
    quantity.dispose();
    price.dispose();
  }
}

class MerchantTemplateListPage extends StatefulWidget {
  final int merchantId;
  final String merchantName;
  const MerchantTemplateListPage({super.key, required this.merchantId, required this.merchantName});

  @override
  State<MerchantTemplateListPage> createState() => _MerchantTemplateListPageState();
}

class _MerchantTemplateListPageState extends State<MerchantTemplateListPage> {
  bool _loading = false;
  bool _loadingMore = false;
  bool _creating = false;
  List<Map<String, dynamic>> _templates = [];
  int _page = 1;
  bool _hasMore = true;
  static const int _pageSize = 20;
  final ScrollController _scrollController = ScrollController();

  final TextEditingController _nameController = TextEditingController();
  final TextEditingController _descController = TextEditingController();
  final TextEditingController _requirementsController = TextEditingController();
  final List<_TemplateItemDraft> _items = [_TemplateItemDraft()];

  @override
  void initState() {
    super.initState();
    _scrollController.addListener(_handleScroll);
    _load(reset: true);
  }

  @override
  void dispose() {
    _nameController.dispose();
    _descController.dispose();
    _requirementsController.dispose();
    for (final item in _items) {
      item.dispose();
    }
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
      _templates = [];
      setState(() => _loading = true);
    } else {
      setState(() => _loadingMore = true);
    }
    try {
      final data = await ApiClient.get(
        "/merchant-templates?merchant_id=${widget.merchantId}&page=$_page&page_size=$_pageSize",
      );
      if (data is Map<String, dynamic>) {
        final rawItems = data["items"];
        final list = rawItems is List
            ? rawItems.cast<Map<String, dynamic>>()
            : <Map<String, dynamic>>[];
        final total = (data["total"] as num?)?.toInt() ?? list.length;
        if (reset) {
          _templates = list;
        } else {
          _templates.addAll(list);
        }
        if (list.isEmpty) {
          _hasMore = false;
        } else {
          _hasMore = _templates.length < total;
          if (_hasMore) {
            _page += 1;
          }
        }
      } else if (data is List) {
        final list = data.cast<Map<String, dynamic>>();
        if (reset) {
          _templates = list;
        } else {
          _templates.addAll(list);
        }
        _hasMore = list.length == _pageSize;
        if (_hasMore) {
          _page += 1;
        }
      } else if (reset) {
        _templates = [];
        _hasMore = false;
      }
    } catch (error) {
      if (reset) {
        _templates = [];
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

  void _addItem() {
    setState(() => _items.add(_TemplateItemDraft()));
  }

  void _removeItem(int index) {
    if (_items.length <= 1) {
      return;
    }
    final item = _items.removeAt(index);
    item.dispose();
    setState(() {});
  }

  Future<void> _createTemplate() async {
    final name = _nameController.text.trim();
    if (name.isEmpty) {
      _showMessage("请填写模板名称");
      return;
    }
    final items = <Map<String, dynamic>>[];
    for (final item in _items) {
      final itemName = item.name.text.trim();
      final quantity = int.tryParse(item.quantity.text.trim());
      final price = double.tryParse(item.price.text.trim());
      if (itemName.isEmpty || quantity == null || price == null) {
        continue;
      }
      items.add({"name": itemName, "quantity": quantity, "price": price});
    }
    if (items.isEmpty) {
      _showMessage("请至少填写一个条目");
      return;
    }

    Object? requirements;
    final requirementsText = _requirementsController.text.trim();
    if (requirementsText.isNotEmpty) {
      try {
        requirements = jsonDecode(requirementsText);
      } catch (_) {
        _showMessage("交付要求必须是合法 JSON");
        return;
      }
    }

    setState(() => _creating = true);
    try {
      await ApiClient.post("/merchant-templates", {
        "merchant_id": widget.merchantId,
        "name": name,
        "description": _descController.text.trim().isEmpty ? null : _descController.text.trim(),
        "delivery_requirements": requirements,
        "items": items,
      });
      _nameController.clear();
      _descController.clear();
      _requirementsController.clear();
      for (final item in _items) {
        item.dispose();
      }
      _items
        ..clear()
        ..add(_TemplateItemDraft());
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

  Widget _buildItemForm(int index) {
    final item = _items[index];
    return Card(
      margin: const EdgeInsets.symmetric(vertical: 8),
      child: Padding(
        padding: const EdgeInsets.all(12),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                Text("条目 ${index + 1}", style: const TextStyle(fontWeight: FontWeight.bold)),
                if (_items.length > 1)
                  IconButton(
                    onPressed: () => _removeItem(index),
                    icon: const Icon(Icons.delete_outline),
                  ),
              ],
            ),
            TextField(
              controller: item.name,
              decoration: const InputDecoration(labelText: "名称"),
            ),
            TextField(
              controller: item.quantity,
              decoration: const InputDecoration(labelText: "数量"),
              keyboardType: TextInputType.number,
            ),
            TextField(
              controller: item.price,
              decoration: const InputDecoration(labelText: "单价"),
              keyboardType: TextInputType.number,
            ),
          ],
        ),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text("${widget.merchantName} 模板"),
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
                        const Text("新增模板", style: TextStyle(fontWeight: FontWeight.bold)),
                        const SizedBox(height: 8),
                        TextField(
                          controller: _nameController,
                          decoration: const InputDecoration(labelText: "模板名称"),
                        ),
                        TextField(
                          controller: _descController,
                          decoration: const InputDecoration(labelText: "描述（可选）"),
                        ),
                        TextField(
                          controller: _requirementsController,
                          decoration: const InputDecoration(labelText: "交付要求 JSON（可选）"),
                          maxLines: 3,
                        ),
                        for (var i = 0; i < _items.length; i++) _buildItemForm(i),
                        TextButton.icon(
                          onPressed: _addItem,
                          icon: const Icon(Icons.add),
                          label: const Text("添加条目"),
                        ),
                        FilledButton(
                          onPressed: _creating ? null : _createTemplate,
                          child: Text(_creating ? "提交中..." : "创建模板"),
                        ),
                        const SizedBox(height: 16),
                        const Divider(),
                        const Text("模板列表", style: TextStyle(fontWeight: FontWeight.bold)),
                        const SizedBox(height: 8),
                        if (_templates.isEmpty)
                          const Text("暂无模板")
                        else
                          for (final template in _templates)
                            Card(
                              margin: const EdgeInsets.symmetric(vertical: 8),
                              child: Padding(
                                padding: const EdgeInsets.all(12),
                                child: Column(
                                  crossAxisAlignment: CrossAxisAlignment.start,
                                  children: [
                                    Text(
                                      template["name"]?.toString() ?? "模板",
                                      style: const TextStyle(fontWeight: FontWeight.bold),
                                    ),
                                    const SizedBox(height: 4),
                                    Text("描述：${template["description"] ?? "-"}"),
                                    Text("创建时间：${template["created_at"] ?? "-"}"),
                                    const SizedBox(height: 4),
                                    const Text("条目："),
                                    if (template["items"] is List)
                                      for (final item in template["items"] as List)
                                        Text(
                                          "- ${item["name"]} x${item["quantity"]} ¥${item["price"]}",
                                        ),
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
