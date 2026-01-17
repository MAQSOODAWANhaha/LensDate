import "package:flutter/material.dart";

import "../../services/api_client.dart";
import "../demands/demand_merchant_assets_page.dart";

class MerchantOrderDetailPage extends StatefulWidget {
  final int orderId;
  const MerchantOrderDetailPage({super.key, required this.orderId});

  @override
  State<MerchantOrderDetailPage> createState() => _MerchantOrderDetailPageState();
}

class _MerchantOrderDetailPageState extends State<MerchantOrderDetailPage> {
  Map<String, dynamic>? _detail;
  bool _loading = false;

  @override
  void initState() {
    super.initState();
    _load();
  }

  Future<void> _load() async {
    setState(() => _loading = true);
    try {
      final detail = await ApiClient.get("/merchants/orders/${widget.orderId}");
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

  void _showMessage(String message) {
    ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text(message)));
  }

  @override
  Widget build(BuildContext context) {
    final detail = _detail ?? {};
    final items = detail["items"] is List ? detail["items"] as List : [];
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
                        Text("支付方式：${detail["pay_type"] ?? "-"}"),
                        Text("订单金额：${detail["total_amount"] ?? "-"}"),
                        Text("服务费：${detail["service_fee"] ?? "-"}"),
                        Text("开始时间：${detail["schedule_start"] ?? "-"}"),
                        Text("结束时间：${detail["schedule_end"] ?? "-"}"),
                        const SizedBox(height: 8),
                        if (demandId != null)
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
                        if (demandId != null) const SizedBox(height: 8),
                        const Text("订单条目："),
                        for (final item in items)
                          Text("- ${item["name"]} x${item["quantity"]} ¥${item["price"]}"),
                      ],
                    ),
                  ),
                ),
              ],
            ),
    );
  }
}
