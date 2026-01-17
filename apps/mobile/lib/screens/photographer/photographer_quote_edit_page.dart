import "package:flutter/material.dart";

import "../../services/api_client.dart";

class _QuoteItemDraft {
  final TextEditingController name;
  final TextEditingController price;
  final TextEditingController quantity;

  _QuoteItemDraft({String? nameValue, String? priceValue, String? quantityValue})
      : name = TextEditingController(text: nameValue ?? ""),
        price = TextEditingController(text: priceValue ?? ""),
        quantity = TextEditingController(text: quantityValue ?? "1");

  void dispose() {
    name.dispose();
    price.dispose();
    quantity.dispose();
  }
}

class PhotographerQuoteEditPage extends StatefulWidget {
  final int quoteId;
  final double totalPrice;
  final String? note;
  final List<dynamic> items;
  const PhotographerQuoteEditPage({
    super.key,
    required this.quoteId,
    required this.totalPrice,
    required this.items,
    this.note,
  });

  @override
  State<PhotographerQuoteEditPage> createState() => _PhotographerQuoteEditPageState();
}

class _PhotographerQuoteEditPageState extends State<PhotographerQuoteEditPage> {
  final TextEditingController _totalController = TextEditingController();
  final TextEditingController _noteController = TextEditingController();
  final List<_QuoteItemDraft> _items = [];
  bool _submitting = false;

  @override
  void initState() {
    super.initState();
    _totalController.text = widget.totalPrice.toString();
    _noteController.text = widget.note ?? "";
    for (final raw in widget.items) {
      final item = raw is Map<String, dynamic> ? raw : <String, dynamic>{};
      _items.add(
        _QuoteItemDraft(
          nameValue: item["name"]?.toString(),
          priceValue: item["price"]?.toString(),
          quantityValue: item["quantity"]?.toString(),
        ),
      );
    }
    if (_items.isEmpty) {
      _items.add(_QuoteItemDraft());
    }
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
      await ApiClient.put("/quotes/${widget.quoteId}", {
        "total_price": total,
        "items": items,
        "note": _noteController.text.trim().isEmpty ? null : _noteController.text.trim(),
      });
      if (mounted) {
        _showMessage("报价已更新");
        Navigator.of(context).pop(true);
      }
    } catch (error) {
      _showMessage("更新失败：$error");
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

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text("修改报价 #${widget.quoteId}"),
      ),
      body: ListView(
        padding: const EdgeInsets.all(16),
        children: [
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
          const SizedBox(height: 12),
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
            child: Text(_submitting ? "提交中..." : "保存修改"),
          ),
        ],
      ),
    );
  }
}
