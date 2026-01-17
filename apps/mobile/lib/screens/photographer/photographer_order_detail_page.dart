import "package:flutter/material.dart";
import "package:image_picker/image_picker.dart";

import "../../services/api_client.dart";
import "../../services/upload_service.dart";
import "../chat/chat_messages_page.dart";
import "../demands/demand_merchant_assets_page.dart";

class _DeliveryDraft {
  final TextEditingController fileUrl = TextEditingController();
  final TextEditingController version = TextEditingController();
  final TextEditingController note = TextEditingController();

  void dispose() {
    fileUrl.dispose();
    version.dispose();
    note.dispose();
  }
}

class PhotographerOrderDetailPage extends StatefulWidget {
  final int orderId;
  const PhotographerOrderDetailPage({super.key, required this.orderId});

  @override
  State<PhotographerOrderDetailPage> createState() => _PhotographerOrderDetailPageState();
}

class _PhotographerOrderDetailPageState extends State<PhotographerOrderDetailPage> {
  Map<String, dynamic>? _detail;
  List<dynamic> _deliveries = [];
  bool _loading = false;
  bool _submitting = false;
  bool _cancelling = false;
  final List<_DeliveryDraft> _drafts = [_DeliveryDraft()];
  final TextEditingController _cancelReason = TextEditingController();

  @override
  void initState() {
    super.initState();
    _load();
  }

  @override
  void dispose() {
    for (final draft in _drafts) {
      draft.dispose();
    }
    _cancelReason.dispose();
    super.dispose();
  }

  Future<void> _load() async {
    setState(() => _loading = true);
    try {
      final detail = await ApiClient.get("/photographers/me/orders/${widget.orderId}");
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

  void _addDraft() {
    setState(() => _drafts.add(_DeliveryDraft()));
  }

  void _removeDraft(int index) {
    final draft = _drafts.removeAt(index);
    draft.dispose();
    setState(() {});
  }

  Future<void> _pickUpload(int index, ImageSource source) async {
    try {
      final url = await UploadService.pickAndUpload(source);
      if (url != null && url.isNotEmpty) {
        _drafts[index].fileUrl.text = url;
      }
    } catch (error) {
      _showMessage("上传失败：$error");
    }
  }

  Future<void> _submitDelivery() async {
    final items = <Map<String, dynamic>>[];
    for (final draft in _drafts) {
      final fileUrl = draft.fileUrl.text.trim();
      if (fileUrl.isEmpty) {
        continue;
      }
      items.add({
        "file_url": fileUrl,
        "version": draft.version.text.trim().isEmpty ? null : draft.version.text.trim(),
        "note": draft.note.text.trim().isEmpty ? null : draft.note.text.trim(),
      });
    }
    if (items.isEmpty) {
      _showMessage("请至少填写一条交付文件");
      return;
    }

    setState(() => _submitting = true);
    try {
      await ApiClient.post("/deliveries", {
        "order_id": widget.orderId,
        "items": items,
      });
      _showMessage("交付已提交");
      _drafts.clear();
      _drafts.add(_DeliveryDraft());
      await _load();
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

  Future<void> _cancelOrder() async {
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

  Widget _buildDeliveryCard(Map<String, dynamic> delivery) {
    final items = delivery["items"] is List ? delivery["items"] as List : [];
    return Card(
      child: Padding(
        padding: const EdgeInsets.all(12),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text("交付 #${delivery["id"]}"),
            Text("状态：${delivery["status"] ?? "-"}"),
            Text("提交时间：${delivery["submitted_at"] ?? "-"}"),
            if (delivery["accepted_at"] != null) Text("验收时间：${delivery["accepted_at"]}"),
            const SizedBox(height: 8),
            const Text("交付条目："),
            for (final item in items)
              Text("- ${item["file_url"] ?? ""} ${item["version"] ?? ""}"),
          ],
        ),
      ),
    );
  }

  Widget _buildDraftForm(int index) {
    final draft = _drafts[index];
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
                Text("交付文件 ${index + 1}", style: const TextStyle(fontWeight: FontWeight.bold)),
                if (_drafts.length > 1)
                  IconButton(
                    onPressed: () => _removeDraft(index),
                    icon: const Icon(Icons.delete_outline),
                  ),
              ],
            ),
            TextField(
              controller: draft.fileUrl,
              decoration: const InputDecoration(labelText: "文件链接"),
            ),
            Row(
              children: [
                TextButton.icon(
                  onPressed: () => _pickUpload(index, ImageSource.gallery),
                  icon: const Icon(Icons.photo_library_outlined),
                  label: const Text("选择图片"),
                ),
                TextButton.icon(
                  onPressed: () => _pickUpload(index, ImageSource.camera),
                  icon: const Icon(Icons.photo_camera_outlined),
                  label: const Text("拍照上传"),
                ),
              ],
            ),
            TextField(
              controller: draft.version,
              decoration: const InputDecoration(labelText: "版本号（可选）"),
            ),
            TextField(
              controller: draft.note,
              decoration: const InputDecoration(labelText: "备注（可选）"),
            ),
          ],
        ),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    final detail = _detail ?? {};
    final items = detail["items"] is List ? detail["items"] as List : [];
    final status = detail["status"]?.toString() ?? "-";
    final demandId = detail["demand_id"] as int?;
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
                    padding: const EdgeInsets.all(12),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text("状态：${detail["status"] ?? "-"}"),
                        Text("用户 ID：${detail["user_id"] ?? "-"}"),
                        Text("用户电话：${detail["user_phone"] ?? "-"}"),
                        Text("订单金额：${detail["total_amount"] ?? "-"}"),
                        Text("服务费：${detail["service_fee"] ?? "-"}"),
                        Text("开始时间：${detail["schedule_start"] ?? "-"}"),
                        Text("结束时间：${detail["schedule_end"] ?? "-"}"),
                        const SizedBox(height: 8),
                        FilledButton.tonal(
                          onPressed: _openChat,
                          child: const Text("联系用户"),
                        ),
                        if (demandId != null) ...[
                          const SizedBox(height: 8),
                          OutlinedButton(
                            onPressed: () => Navigator.of(context).push(
                              MaterialPageRoute(
                                builder: (_) => DemandMerchantAssetsPage(
                                  demandId: demandId,
                                ),
                              ),
                            ),
                            child: const Text("查看商户素材库"),
                          ),
                        ],
                        const SizedBox(height: 8),
                        const Text("订单条目："),
                        for (final item in items)
                          Text("- ${item["name"]} x${item["quantity"]} ¥${item["price"]}"),
                      ],
                    ),
                  ),
                ),
                if (status != "cancelled" && status != "completed" && status != "reviewed") ...[
                  const SizedBox(height: 12),
                  Card(
                    child: Padding(
                      padding: const EdgeInsets.all(12),
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
                const Text("交付记录", style: TextStyle(fontWeight: FontWeight.bold)),
                const SizedBox(height: 8),
                if (_deliveries.isEmpty)
                  const Text("暂无交付记录")
                else
                  for (final delivery in _deliveries)
                    _buildDeliveryCard(delivery as Map<String, dynamic>),
                const SizedBox(height: 16),
                const Text("提交交付", style: TextStyle(fontWeight: FontWeight.bold)),
                const SizedBox(height: 8),
                for (var i = 0; i < _drafts.length; i++) _buildDraftForm(i),
                TextButton.icon(
                  onPressed: _addDraft,
                  icon: const Icon(Icons.add),
                  label: const Text("添加文件"),
                ),
                const SizedBox(height: 8),
                FilledButton(
                  onPressed: _submitting ? null : _submitDelivery,
                  child: Text(_submitting ? "提交中..." : "提交交付"),
                ),
              ],
            ),
    );
  }
}
