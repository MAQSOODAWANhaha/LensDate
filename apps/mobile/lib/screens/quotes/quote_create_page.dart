import "package:flutter/material.dart";

import "../../services/api_client.dart";

class _QuoteItemDraft {
  final TextEditingController name = TextEditingController();
  final TextEditingController price = TextEditingController();
  final TextEditingController quantity = TextEditingController(text: "1");

  void dispose() {
    name.dispose();
    price.dispose();
    quantity.dispose();
  }
}

class QuoteCreatePage extends StatefulWidget {
  final int demandId;
  const QuoteCreatePage({super.key, required this.demandId});

  @override
  State<QuoteCreatePage> createState() => _QuoteCreatePageState();
}

class _QuoteCreatePageState extends State<QuoteCreatePage> {
  bool _loading = false;
  bool _submitting = false;
  int? _photographerId;
  String? _error;

  final TextEditingController _totalController = TextEditingController();
  final TextEditingController _noteController = TextEditingController();
  final List<_QuoteItemDraft> _items = [_QuoteItemDraft()];

  @override
  void initState() {
    super.initState();
    _loadPhotographer();
  }

  @override
  void dispose() {
    _totalController.dispose();
    _noteController.dispose();
    for (final item in _items) {
      item.dispose();
    }
    super.dispose();
  }

  Future<void> _loadPhotographer() async {
    setState(() {
      _loading = true;
      _error = null;
    });
    try {
      final data = await ApiClient.get("/photographers/me");
      if (data is Map<String, dynamic>) {
        _photographerId = data["id"] as int?;
      }
    } catch (error) {
      _photographerId = null;
      _error = error.toString();
    } finally {
      if (mounted) {
        setState(() => _loading = false);
      }
    }
  }

  void _addItem() {
    setState(() => _items.add(_QuoteItemDraft()));
  }

  void _removeItem(int index) {
    if (_items.length <= 1) {
      return;
    }
    final item = _items.removeAt(index);
    item.dispose();
    setState(() {});
  }

  Future<void> _submit() async {
    final photographerId = _photographerId;
    if (photographerId == null) {
      _showMessage("仅摄影师可提交报价");
      return;
    }
    final total = double.tryParse(_totalController.text.trim());
    if (total == null || total <= 0) {
      _showMessage("请填写总价");
      return;
    }

    final items = <Map<String, dynamic>>[];
    for (final item in _items) {
      final name = item.name.text.trim();
      final price = double.tryParse(item.price.text.trim());
      final quantity = int.tryParse(item.quantity.text.trim());
      if (name.isEmpty || price == null || quantity == null) {
        continue;
      }
      items.add({"name": name, "price": price, "quantity": quantity});
    }
    if (items.isEmpty) {
      _showMessage("请至少填写一个报价条目");
      return;
    }

    setState(() => _submitting = true);
    try {
      await ApiClient.post("/quotes", {
        "demand_id": widget.demandId,
        "photographer_id": photographerId,
        "team_id": null,
        "total_price": total,
        "items": items,
        "note": _noteController.text.trim().isEmpty ? null : _noteController.text.trim(),
      });
      if (mounted) {
        _showMessage("报价已提交");
        Navigator.of(context).pop();
      }
    } catch (error) {
      _showMessage("提交失败：$error");
    } finally {
      if (mounted) {
        setState(() => _submitting = false);
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
              controller: item.price,
              decoration: const InputDecoration(labelText: "单价"),
              keyboardType: TextInputType.number,
            ),
            TextField(
              controller: item.quantity,
              decoration: const InputDecoration(labelText: "数量"),
              keyboardType: TextInputType.number,
            ),
          ],
        ),
      ),
    );
  }

  bool get _isNotFound {
    final err = _error ?? "";
    return err.contains("not_found");
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text("提交报价"),
        actions: [IconButton(onPressed: _loadPhotographer, icon: const Icon(Icons.refresh))],
      ),
      body: _loading
          ? const Center(child: CircularProgressIndicator())
          : ListView(
              padding: const EdgeInsets.all(16),
              children: [
                if (_photographerId == null)
                  Card(
                    child: Padding(
                      padding: const EdgeInsets.all(12),
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          const Text("未找到摄影师档案"),
                          const SizedBox(height: 8),
                          const Text("请先在摄影师中心完成申请。"),
                          if (_error != null && !_isNotFound)
                            Padding(
                              padding: const EdgeInsets.only(top: 8),
                              child: Text("错误：$_error", style: const TextStyle(color: Colors.red)),
                            ),
                        ],
                      ),
                    ),
                  ),
                TextField(
                  controller: _totalController,
                  decoration: const InputDecoration(labelText: "报价总价"),
                  keyboardType: TextInputType.number,
                ),
                const SizedBox(height: 12),
                TextField(
                  controller: _noteController,
                  decoration: const InputDecoration(labelText: "版本说明（可选）"),
                  maxLines: 2,
                ),
                const SizedBox(height: 8),
                const Text("报价条目", style: TextStyle(fontWeight: FontWeight.bold)),
                for (var i = 0; i < _items.length; i++) _buildItemForm(i),
                TextButton.icon(
                  onPressed: _addItem,
                  icon: const Icon(Icons.add),
                  label: const Text("添加条目"),
                ),
                const SizedBox(height: 8),
                FilledButton(
                  onPressed: _submitting ? null : _submit,
                  child: Text(_submitting ? "提交中..." : "提交报价"),
                ),
              ],
            ),
    );
  }
}
