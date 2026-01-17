import "package:flutter/material.dart";

import "../../services/api_client.dart";
import "merchant_create_page.dart";
import "merchant_dashboard_page.dart";

class MerchantCenterPage extends StatefulWidget {
  const MerchantCenterPage({super.key});

  @override
  State<MerchantCenterPage> createState() => _MerchantCenterPageState();
}

class _MerchantCenterPageState extends State<MerchantCenterPage> {
  bool _loading = false;
  List<Map<String, dynamic>> _merchants = [];
  int? _userId;

  @override
  void initState() {
    super.initState();
    _load();
  }

  Future<void> _load() async {
    setState(() => _loading = true);
    try {
      final me = await ApiClient.get("/users/me");
      if (me is Map<String, dynamic>) {
        _userId = me["id"] as int?;
      }
      final data = await ApiClient.get("/merchants/mine");
      if (data is List) {
        _merchants = data.cast<Map<String, dynamic>>();
      } else {
        _merchants = [];
      }
    } catch (error) {
      _merchants = [];
      _showMessage("加载失败：$error");
    } finally {
      if (mounted) {
        setState(() => _loading = false);
      }
    }
  }

  Future<void> _openCreate() async {
    final userId = _userId;
    if (userId == null) {
      _showMessage("无法获取用户信息");
      return;
    }
    await Navigator.of(context).push(
      MaterialPageRoute(builder: (_) => MerchantCreatePage(userId: userId)),
    );
    await _load();
  }

  void _openDashboard(Map<String, dynamic> merchant) {
    Navigator.of(context).push(
      MaterialPageRoute(builder: (_) => MerchantDashboardPage(merchant: merchant)),
    );
  }

  void _showMessage(String message) {
    ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text(message)));
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text("商户中心"),
        actions: [IconButton(onPressed: _load, icon: const Icon(Icons.refresh))],
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: _openCreate,
        child: const Icon(Icons.add_business),
      ),
      body: _loading
          ? const Center(child: CircularProgressIndicator())
          : RefreshIndicator(
              onRefresh: _load,
              child: _merchants.isEmpty
                  ? ListView(
                      children: const [
                        SizedBox(height: 120),
                        Center(child: Text("暂无商户")),
                      ],
                    )
                  : ListView.separated(
                      padding: const EdgeInsets.all(16),
                      itemBuilder: (context, index) {
                        final item = _merchants[index];
                        final name = item["name"]?.toString() ?? "商户";
                        return ListTile(
                          tileColor: Colors.white,
                          title: Text(name),
                          subtitle: Text(
                            "状态：${item["status"] ?? "-"} / 角色：${item["role"] ?? "-"}",
                          ),
                          trailing: const Icon(Icons.chevron_right),
                          onTap: () => _openDashboard(item),
                        );
                      },
                      separatorBuilder: (_, __) => const SizedBox(height: 12),
                      itemCount: _merchants.length,
                    ),
            ),
    );
  }
}