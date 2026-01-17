import "package:flutter/material.dart";

import "../../services/api_client.dart";
import "../demands/demand_detail_page.dart";
import "photographer_order_detail_page.dart";
import "photographer_quote_edit_page.dart";
import "quote_versions_page.dart";

class PhotographerQuoteDetailPage extends StatefulWidget {
  final int quoteId;
  const PhotographerQuoteDetailPage({super.key, required this.quoteId});

  @override
  State<PhotographerQuoteDetailPage> createState() => _PhotographerQuoteDetailPageState();
}

class _PhotographerQuoteDetailPageState extends State<PhotographerQuoteDetailPage> {
  Map<String, dynamic>? _detail;
  bool _loading = false;
  bool _withdrawing = false;

  @override
  void initState() {
    super.initState();
    _load();
  }

  Future<void> _load() async {
    setState(() => _loading = true);
    try {
      final detail = await ApiClient.get("/quotes/${widget.quoteId}");
      if (detail is Map<String, dynamic>) {
        _detail = detail;
      }
    } catch (error) {
      _showMessage("加载失败：$error");
    } finally {
      if (mounted) {
        setState(() => _loading = false);
      }
    }
  }

  void _openDemand(int demandId) {
    Navigator.of(context).push(
      MaterialPageRoute(builder: (_) => DemandDetailPage(demandId: demandId)),
    );
  }

  void _openOrder(int orderId) {
    Navigator.of(context).push(
      MaterialPageRoute(builder: (_) => PhotographerOrderDetailPage(orderId: orderId)),
    );
  }

  Future<void> _withdraw() async {
    setState(() => _withdrawing = true);
    try {
      await ApiClient.post("/quotes/${widget.quoteId}/withdraw", {});
      _showMessage("报价已撤回");
      await _load();
    } catch (error) {
      _showMessage("撤回失败：$error");
    } finally {
      if (mounted) {
        setState(() => _withdrawing = false);
      }
    }
  }

  Future<void> _editQuote() async {
    final detail = _detail ?? {};
    final items = detail["items"] is List ? detail["items"] as List : [];
    final total = (detail["total_price"] as num?)?.toDouble() ?? 0;
    final note = detail["note"]?.toString();
    final updated = await Navigator.of(context).push<bool>(
      MaterialPageRoute(
        builder: (_) => PhotographerQuoteEditPage(
          quoteId: widget.quoteId,
          totalPrice: total,
          items: items,
          note: note,
        ),
      ),
    );
    if (updated == true) {
      await _load();
    }
  }

  void _openVersions() {
    Navigator.of(context).push(
      MaterialPageRoute(
        builder: (_) => QuoteVersionsPage(quoteId: widget.quoteId),
      ),
    );
  }

  void _showMessage(String message) {
    ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text(message)));
  }

  String _quoteStatusLabel(String status) {
    switch (status) {
      case "pending":
        return "待确认";
      case "accepted":
        return "已接受";
      case "expired":
        return "已过期/撤回";
      default:
        return "未知";
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
    final detail = _detail ?? {};
    final items = detail["items"] is List ? detail["items"] as List : [];
    final demandId = detail["demand_id"] as int? ?? 0;
    final orderId = detail["order_id"] as int? ?? 0;
    final orderStatus = detail["order_status"]?.toString() ?? "-";
    final status = detail["status"]?.toString() ?? "-";
    final version = detail["version"]?.toString() ?? "-";
    final expiresAt = detail["expires_at"]?.toString() ?? "-";
    final canWithdraw = status == "pending" && orderId == 0;
    final canEdit = status == "pending" && orderId == 0;
    return Scaffold(
      appBar: AppBar(
        title: Text("报价 #${widget.quoteId}"),
        actions: [IconButton(onPressed: _load, icon: const Icon(Icons.refresh))],
      ),
      body: _loading
          ? const Center(child: CircularProgressIndicator())
          : ListView(
              padding: const EdgeInsets.all(16),
              children: [
                Card(
                  child: Padding(
                    padding: const EdgeInsets.all(12),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text("需求 ID：$demandId"),
                        Text("状态：$status（${_quoteStatusLabel(status)}）"),
                        Text("总价：${detail["total_price"] ?? "-"}"),
                        Text("版本：v$version"),
                        Text("有效期：$expiresAt"),
                        Text("订单 ID：${detail["order_id"] ?? "-"}"),
                        Text(
                          "订单状态：${orderStatus == "-" ? "-" : _orderStatusLabel(orderStatus)}",
                        ),
                        const SizedBox(height: 8),
                        const Text("报价条目："),
                        for (final item in items)
                          Text("- ${item["name"]} x${item["quantity"]} ¥${item["price"]}"),
                      ],
                    ),
                  ),
                ),
                const SizedBox(height: 12),
                if (demandId > 0)
                  OutlinedButton(
                    onPressed: () => _openDemand(demandId),
                    child: const Text("查看需求详情"),
                  ),
                if (orderId > 0)
                  OutlinedButton(
                    onPressed: () => _openOrder(orderId),
                    child: const Text("查看订单详情"),
                  ),
                OutlinedButton(
                  onPressed: _openVersions,
                  child: const Text("查看版本记录"),
                ),
                if (canEdit)
                  FilledButton(
                    onPressed: _editQuote,
                    child: const Text("修改报价"),
                  ),
                if (canWithdraw)
                  FilledButton.tonal(
                    onPressed: _withdrawing ? null : _withdraw,
                    child: Text(_withdrawing ? "撤回中..." : "撤回报价"),
                  ),
              ],
            ),
    );
  }
}
