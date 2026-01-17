import "package:flutter/material.dart";

import "../../services/api_client.dart";
import "../chat/chat_messages_page.dart";

class OrderDetailPage extends StatefulWidget {
  final int orderId;
  const OrderDetailPage({super.key, required this.orderId});

  @override
  State<OrderDetailPage> createState() => _OrderDetailPageState();
}

class _OrderDetailPageState extends State<OrderDetailPage> {
  Map<String, dynamic>? _detail;
  List<dynamic> _deliveries = [];
  bool _loading = false;

  final TextEditingController _payAmount = TextEditingController();
  final TextEditingController _payProof = TextEditingController();
  String _payChannel = "wx";
  String _payStage = "deposit";

  final TextEditingController _refundAmount = TextEditingController();
  final TextEditingController _refundReason = TextEditingController();
  final TextEditingController _refundProof = TextEditingController();

  final TextEditingController _reviewScore = TextEditingController();
  final TextEditingController _reviewComment = TextEditingController();
  final TextEditingController _reviewTags = TextEditingController();

  final TextEditingController _disputeReason = TextEditingController();
  final TextEditingController _cancelReason = TextEditingController();
  bool _cancelling = false;

  @override
  void dispose() {
    _payAmount.dispose();
    _payProof.dispose();
    _refundAmount.dispose();
    _refundReason.dispose();
    _refundProof.dispose();
    _reviewScore.dispose();
    _reviewComment.dispose();
    _reviewTags.dispose();
    _disputeReason.dispose();
    _cancelReason.dispose();
    super.dispose();
  }

  @override
  void initState() {
    super.initState();
    _load();
  }

  Future<void> _load() async {
    setState(() => _loading = true);
    try {
      final detail = await ApiClient.get("/orders/${widget.orderId}");
      final deliveries = await ApiClient.get("/deliveries?order_id=${widget.orderId}");
      if (detail is Map<String, dynamic>) {
        _detail = detail;
      }
      if (deliveries is List) {
        _deliveries = deliveries;
      }
    } catch (error) {
      _showMessage("加载失败：$error");
    } finally {
      if (mounted) {
        setState(() => _loading = false);
      }
    }
  }

  Future<void> _createPayment() async {
    final amount = double.tryParse(_payAmount.text.trim());
    final proof = _payProof.text.trim();
    if (amount == null || amount <= 0 || proof.isEmpty) {
      _showMessage("请填写支付金额和凭证链接");
      return;
    }
    try {
      final detail = _detail ?? {};
      final payType = detail["pay_type"]?.toString() ?? "deposit";
      await ApiClient.post("/payments", {
        "order_id": widget.orderId,
        "amount": amount,
        "pay_channel": _payChannel,
        "proof_url": proof,
        if (payType == "phase") "stage": _payStage
      });
      _showMessage("支付提交成功");
      await _load();
    } catch (error) {
      _showMessage("支付失败：$error");
    }
  }

  Future<void> _createRefund() async {
    final amount = double.tryParse(_refundAmount.text.trim());
    if (amount == null || amount <= 0) {
      _showMessage("请填写退款金额");
      return;
    }
    try {
      await ApiClient.post("/refunds", {
        "order_id": widget.orderId,
        "amount": amount,
        "reason": _refundReason.text.trim().isEmpty ? null : _refundReason.text.trim(),
        "proof_url": _refundProof.text.trim().isEmpty ? null : _refundProof.text.trim()
      });
      _showMessage("退款申请已提交");
      await _load();
    } catch (error) {
      _showMessage("退款失败：$error");
    }
  }

  Future<void> _submitReview() async {
    final score = int.tryParse(_reviewScore.text.trim());
    if (score == null || score < 1 || score > 5) {
      _showMessage("请填写 1-5 分评分");
      return;
    }
    final tags = _reviewTags.text
        .split(",")
        .map((e) => e.trim())
        .where((e) => e.isNotEmpty)
        .toList();
    try {
      await ApiClient.post("/reviews", {
        "order_id": widget.orderId,
        "score": score,
        "tags": tags.isEmpty ? null : tags,
        "comment": _reviewComment.text.trim().isEmpty ? null : _reviewComment.text.trim()
      });
      _showMessage("评价已提交");
      await _load();
    } catch (error) {
      _showMessage("评价失败：$error");
    }
  }

  Future<void> _createDispute() async {
    final reason = _disputeReason.text.trim();
    if (reason.isEmpty) {
      _showMessage("请填写纠纷原因");
      return;
    }
    try {
      await ApiClient.post("/disputes", {
        "order_id": widget.orderId,
        "reason": reason
      });
      _showMessage("纠纷已提交");
      await _load();
    } catch (error) {
      _showMessage("提交失败：$error");
    }
  }

  Future<void> _acceptDelivery(int deliveryId) async {
    try {
      await ApiClient.post("/deliveries/$deliveryId/accept", {});
      _showMessage("已验收交付");
      await _load();
    } catch (error) {
      _showMessage("验收失败：$error");
    }
  }

  Future<void> _cancelOrder() async {
    final detail = _detail ?? {};
    final status = detail["status"]?.toString() ?? "";
    if (status == "cancelled" || status == "completed" || status == "reviewed") {
      _showMessage("当前状态无法取消");
      return;
    }
    setState(() => _cancelling = true);
    try {
      final preview = await ApiClient.get("/orders/${widget.orderId}/refund-preview");
      final refundAmount = preview["refund_amount"] ?? 0;
      final paidAmount = preview["paid_amount"] ?? 0;
      final confirm = await showDialog<bool>(
        context: context,
        builder: (context) => AlertDialog(
          title: const Text("确认取消订单"),
          content: Text("已支付：$paidAmount\n预计退款：$refundAmount\n是否继续取消？"),
          actions: [
            TextButton(
              onPressed: () => Navigator.of(context).pop(false),
              child: const Text("先不取消"),
            ),
            FilledButton(
              onPressed: () => Navigator.of(context).pop(true),
              child: const Text("确认取消"),
            ),
          ],
        ),
      );
      if (confirm != true) {
        return;
      }
      await ApiClient.post("/orders/${widget.orderId}/cancel", {
        "reason": _cancelReason.text.trim().isEmpty ? null : _cancelReason.text.trim(),
      });
      _showMessage("订单已取消");
      await _load();
    } catch (error) {
      _showMessage("取消失败：$error");
    } finally {
      if (mounted) {
        setState(() => _cancelling = false);
      }
    }
  }

  Future<void> _openChat() async {
    try {
      final data = await ApiClient.post("/conversations", {
        "type": "order",
        "order_id": widget.orderId,
      });
      if (data is Map<String, dynamic>) {
        final conversationId = data["id"] as int?;
        if (conversationId != null && mounted) {
          Navigator.of(context).push(
            MaterialPageRoute(
              builder: (_) => ChatMessagesPage(
                conversationId: conversationId,
                orderId: widget.orderId,
              ),
            ),
          );
        }
      }
    } catch (error) {
      _showMessage("进入会话失败：$error");
    }
  }

  void _showMessage(String message) {
    ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text(message)));
  }

  Color _statusColor(String status) {
    switch (status) {
      case "paid":
        return Colors.green;
      case "confirmed":
        return Colors.orange;
      case "frozen":
        return Colors.red;
      case "completed":
        return Colors.blue;
      default:
        return Colors.grey;
    }
  }

  String _statusHint(String status) {
    switch (status) {
      case "confirmed":
        return "订单已确认，请尽快完成支付";
      case "paid":
        return "已完成支付，等待交付/验收";
      case "frozen":
        return "订单已冻结，请联系平台";
      case "completed":
        return "订单已完成，感谢使用";
      default:
        return "状态更新中";
    }
  }

  @override
  Widget build(BuildContext context) {
    final detail = _detail ?? {};
    final items = detail["items"] is List ? detail["items"] as List : [];
    final status = detail["status"]?.toString() ?? "-";
    final payType = detail["pay_type"]?.toString() ?? "deposit";
    final canPay = status != "paid" && status != "frozen";
    final canRefund = status == "paid";
    return Scaffold(
      appBar: AppBar(
        title: Text("订单 #${widget.orderId}"),
        actions: [IconButton(onPressed: _load, icon: const Icon(Icons.refresh))],
      ),
      body: _loading
          ? const Center(child: CircularProgressIndicator())
          : ListView(
              padding: const EdgeInsets.all(16),
              children: [
                Card(
                  child: Padding(
                    padding: const EdgeInsets.all(16),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Row(
                          children: [
                            Text(
                              "状态：$status",
                              style: const TextStyle(fontSize: 16, fontWeight: FontWeight.bold),
                            ),
                            const SizedBox(width: 8),
                            Chip(
                              label: Text(status),
                              backgroundColor: _statusColor(status).withValues(alpha: 0.12),
                              labelStyle: TextStyle(color: _statusColor(status)),
                            ),
                          ],
                        ),
                        const SizedBox(height: 8),
                        Text(_statusHint(status)),
                        const SizedBox(height: 8),
                        Text("支付方式：${detail["pay_type"] ?? "-"}"),
                        const SizedBox(height: 4),
                        Text("总金额：${detail["total_amount"] ?? "-"}"),
                        const SizedBox(height: 4),
                        Text("服务费：${detail["service_fee"] ?? "-"}"),
                        const SizedBox(height: 8),
                        FilledButton.tonal(
                          onPressed: _openChat,
                          child: const Text("联系摄影师"),
                        ),
                      ],
                    ),
                  ),
                ),
                const SizedBox(height: 12),
                Card(
                  child: Padding(
                    padding: const EdgeInsets.all(16),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        const Text("订单条目", style: TextStyle(fontWeight: FontWeight.bold)),
                        const SizedBox(height: 8),
                        for (final item in items)
                          ListTile(
                            contentPadding: EdgeInsets.zero,
                            title: Text(item["name"]?.toString() ?? "项目"),
                            subtitle: Text("数量 ${item["quantity"]} / 单价 ${item["price"]}"),
                          ),
                      ],
                    ),
                  ),
                ),
                const SizedBox(height: 12),
                Card(
                  child: Padding(
                    padding: const EdgeInsets.all(16),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        const Text("支付", style: TextStyle(fontWeight: FontWeight.bold)),
                        const SizedBox(height: 8),
                        TextField(
                          controller: _payAmount,
                          keyboardType: TextInputType.number,
                          decoration: const InputDecoration(labelText: "支付金额"),
                        ),
                        const SizedBox(height: 12),
                        DropdownButtonFormField<String>(
                          initialValue: _payChannel,
                          items: const [
                            DropdownMenuItem(value: "wx", child: Text("微信")),
                            DropdownMenuItem(value: "alipay", child: Text("支付宝")),
                            DropdownMenuItem(value: "bank", child: Text("银行转账")),
                          ],
                          onChanged: canPay ? (value) => setState(() => _payChannel = value ?? "wx") : null,
                          decoration: const InputDecoration(labelText: "支付渠道"),
                        ),
                        if (payType == "phase") ...[
                          const SizedBox(height: 12),
                          DropdownButtonFormField<String>(
                            key: ValueKey(_payStage),
                            initialValue: _payStage,
                            items: const [
                              DropdownMenuItem(value: "deposit", child: Text("首款/定金")),
                              DropdownMenuItem(value: "mid", child: Text("中期款")),
                              DropdownMenuItem(value: "final", child: Text("尾款")),
                            ],
                            onChanged: canPay
                                ? (value) => setState(() => _payStage = value ?? "deposit")
                                : null,
                            decoration: const InputDecoration(labelText: "分期阶段"),
                          ),
                        ],
                        const SizedBox(height: 12),
                        TextField(
                          controller: _payProof,
                          decoration: const InputDecoration(labelText: "支付凭证链接"),
                        ),
                        const SizedBox(height: 8),
                        FilledButton(
                          onPressed: canPay ? _createPayment : null,
                          child: const Text("提交支付"),
                        ),
                      ],
                    ),
                  ),
                ),
                if (status != "cancelled" && status != "completed" && status != "reviewed") ...[
                  const SizedBox(height: 12),
                  Card(
                    child: Padding(
                      padding: const EdgeInsets.all(16),
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          const Text("取消订单", style: TextStyle(fontWeight: FontWeight.bold)),
                          const SizedBox(height: 8),
                          TextField(
                            controller: _cancelReason,
                            decoration: const InputDecoration(labelText: "取消原因（可选）"),
                          ),
                          const SizedBox(height: 8),
                          FilledButton.tonal(
                            onPressed: _cancelling ? null : _cancelOrder,
                            child: Text(_cancelling ? "取消中..." : "取消订单"),
                          ),
                        ],
                      ),
                    ),
                  ),
                ],
                const SizedBox(height: 12),
                Card(
                  child: Padding(
                    padding: const EdgeInsets.all(16),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        const Text("退款申请", style: TextStyle(fontWeight: FontWeight.bold)),
                        const SizedBox(height: 8),
                        TextField(
                          controller: _refundAmount,
                          keyboardType: TextInputType.number,
                          decoration: const InputDecoration(labelText: "退款金额"),
                        ),
                        const SizedBox(height: 12),
                        TextField(
                          controller: _refundReason,
                          decoration: const InputDecoration(labelText: "退款原因"),
                        ),
                        const SizedBox(height: 12),
                        TextField(
                          controller: _refundProof,
                          decoration: const InputDecoration(labelText: "凭证链接"),
                        ),
                        const SizedBox(height: 8),
                        FilledButton(
                          onPressed: canRefund ? _createRefund : null,
                          child: const Text("提交退款"),
                        ),
                        if (!canRefund)
                          const Padding(
                            padding: EdgeInsets.only(top: 8),
                            child: Text("仅在订单已支付时可申请退款", style: TextStyle(color: Colors.grey)),
                          ),
                      ],
                    ),
                  ),
                ),
                const SizedBox(height: 12),
                Card(
                  child: Padding(
                    padding: const EdgeInsets.all(16),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        const Text("交付记录", style: TextStyle(fontWeight: FontWeight.bold)),
                        const SizedBox(height: 8),
                        for (final delivery in _deliveries)
                          Card(
                            elevation: 0,
                            child: ListTile(
                              title: Text("交付 #${delivery["id"]}"),
                              subtitle: Text("状态：${delivery["status"]}"),
                              trailing: delivery["status"] == "submitted"
                                  ? TextButton(
                                      onPressed: () => _acceptDelivery(delivery["id"] as int),
                                      child: const Text("验收"),
                                    )
                                  : null,
                            ),
                          ),
                        if (_deliveries.isEmpty) const Text("暂无交付记录"),
                      ],
                    ),
                  ),
                ),
                const SizedBox(height: 12),
                Card(
                  child: Padding(
                    padding: const EdgeInsets.all(16),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        const Text("评价", style: TextStyle(fontWeight: FontWeight.bold)),
                        const SizedBox(height: 8),
                        TextField(
                          controller: _reviewScore,
                          keyboardType: TextInputType.number,
                          decoration: const InputDecoration(labelText: "评分（1-5）"),
                        ),
                        const SizedBox(height: 12),
                        TextField(
                          controller: _reviewTags,
                          decoration: const InputDecoration(labelText: "标签（逗号分隔）"),
                        ),
                        const SizedBox(height: 12),
                        TextField(
                          controller: _reviewComment,
                          decoration: const InputDecoration(labelText: "评价内容"),
                        ),
                        const SizedBox(height: 8),
                        FilledButton(onPressed: _submitReview, child: const Text("提交评价")),
                      ],
                    ),
                  ),
                ),
                const SizedBox(height: 12),
                Card(
                  child: Padding(
                    padding: const EdgeInsets.all(16),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        const Text("纠纷", style: TextStyle(fontWeight: FontWeight.bold)),
                        const SizedBox(height: 8),
                        TextField(
                          controller: _disputeReason,
                          decoration: const InputDecoration(labelText: "纠纷原因"),
                        ),
                        const SizedBox(height: 8),
                        FilledButton(onPressed: _createDispute, child: const Text("提交纠纷")),
                      ],
                    ),
                  ),
                ),
              ],
            ),
    );
  }
}
