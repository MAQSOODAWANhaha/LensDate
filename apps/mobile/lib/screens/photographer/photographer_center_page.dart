import "package:flutter/material.dart";

import "../../services/api_client.dart";
import "photographer_apply_page.dart";
import "photographer_order_list_page.dart";
import "photographer_quote_list_page.dart";
import "portfolio_list_page.dart";
import "team_list_page.dart";

class PhotographerCenterPage extends StatefulWidget {
  const PhotographerCenterPage({super.key});

  @override
  State<PhotographerCenterPage> createState() => _PhotographerCenterPageState();
}

class _PhotographerCenterPageState extends State<PhotographerCenterPage> {
  bool _loading = false;
  Map<String, dynamic>? _profile;
  String? _error;

  @override
  void initState() {
    super.initState();
    _load();
  }

  Future<void> _load() async {
    setState(() {
      _loading = true;
      _error = null;
    });
    try {
      final data = await ApiClient.get("/photographers/me");
      if (data is Map<String, dynamic>) {
        _profile = data;
      }
    } catch (error) {
      _profile = null;
      _error = error.toString();
    } finally {
      if (mounted) {
        setState(() => _loading = false);
      }
    }
  }

  bool get _notFound {
    final err = _error ?? "";
    return err.contains("not_found");
  }

  Future<void> _openApply() async {
    await Navigator.of(context).push(
      MaterialPageRoute(builder: (_) => const PhotographerApplyPage()),
    );
    await _load();
  }

  void _openOrders() {
    Navigator.of(context).push(
      MaterialPageRoute(builder: (_) => const PhotographerOrderListPage()),
    );
  }

  void _openQuotes() {
    Navigator.of(context).push(
      MaterialPageRoute(builder: (_) => const PhotographerQuoteListPage()),
    );
  }

  void _openPortfolios(int photographerId) {
    Navigator.of(context).push(
      MaterialPageRoute(
        builder: (_) => PortfolioListPage(photographerId: photographerId),
      ),
    );
  }

  void _openTeams() {
    Navigator.of(context).push(
      MaterialPageRoute(builder: (_) => const TeamListPage()),
    );
  }

  Widget _buildProfileCard() {
    final profile = _profile ?? {};
    final id = profile["id"] as int? ?? 0;
    return Card(
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text("摄影师 ID：$id", style: const TextStyle(fontWeight: FontWeight.bold)),
            const SizedBox(height: 8),
            Text("类型：${profile["type"] ?? "-"}"),
            Text("状态：${profile["status"] ?? "-"}"),
            Text("城市：${profile["city_id"] ?? "-"}"),
            Text("服务范围：${profile["service_area"] ?? "-"}"),
            const SizedBox(height: 12),
            FilledButton.icon(
              onPressed: _openOrders,
              icon: const Icon(Icons.receipt_long),
              label: const Text("我的订单"),
            ),
            const SizedBox(height: 8),
            OutlinedButton.icon(
              onPressed: _openQuotes,
              icon: const Icon(Icons.request_quote_outlined),
              label: const Text("我的报价"),
            ),
            const SizedBox(height: 8),
            OutlinedButton.icon(
              onPressed: () => _openPortfolios(id),
              icon: const Icon(Icons.photo_library_outlined),
              label: const Text("作品集管理"),
            ),
            const SizedBox(height: 8),
            OutlinedButton.icon(
              onPressed: _openTeams,
              icon: const Icon(Icons.group_outlined),
              label: const Text("团队管理"),
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildEmptyCard() {
    return Card(
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Text("你还没有摄影师档案"),
            const SizedBox(height: 8),
            FilledButton(
              onPressed: _openApply,
              child: const Text("申请成为摄影师/团队"),
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
        title: const Text("摄影师中心"),
        actions: [IconButton(onPressed: _load, icon: const Icon(Icons.refresh))],
      ),
      body: _loading
          ? const Center(child: CircularProgressIndicator())
          : ListView(
              padding: const EdgeInsets.all(16),
              children: [
                if (_profile != null) _buildProfileCard() else _buildEmptyCard(),
                if (_error != null && !_notFound)
                  Padding(
                    padding: const EdgeInsets.only(top: 12),
                    child: Text("加载失败：$_error", style: const TextStyle(color: Colors.red)),
                  ),
              ],
            ),
    );
  }
}
