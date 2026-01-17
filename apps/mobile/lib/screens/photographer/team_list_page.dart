import "package:flutter/material.dart";

import "../../services/api_client.dart";
import "team_detail_page.dart";

class TeamListPage extends StatefulWidget {
  const TeamListPage({super.key});

  @override
  State<TeamListPage> createState() => _TeamListPageState();
}

class _TeamListPageState extends State<TeamListPage> {
  bool _loading = false;
  bool _creating = false;
  List<Map<String, dynamic>> _teams = [];
  final TextEditingController _nameController = TextEditingController();

  @override
  void initState() {
    super.initState();
    _load();
  }

  @override
  void dispose() {
    _nameController.dispose();
    super.dispose();
  }

  Future<void> _load() async {
    setState(() => _loading = true);
    try {
      final data = await ApiClient.get("/teams");
      if (data is List) {
        _teams = data.cast<Map<String, dynamic>>();
      } else {
        _teams = [];
      }
    } catch (error) {
      _teams = [];
      _showMessage("加载失败：$error");
    } finally {
      if (mounted) {
        setState(() => _loading = false);
      }
    }
  }

  Future<void> _createTeam() async {
    final name = _nameController.text.trim();
    if (name.isEmpty) {
      _showMessage("请填写团队名称");
      return;
    }
    setState(() => _creating = true);
    try {
      await ApiClient.post("/teams", {"name": name});
      _nameController.clear();
      await _load();
    } catch (error) {
      _showMessage("创建失败：$error");
    } finally {
      if (mounted) {
        setState(() => _creating = false);
      }
    }
  }

  void _openDetail(Map<String, dynamic> team) {
    Navigator.of(context).push(
      MaterialPageRoute(builder: (_) => TeamDetailPage(team: team)),
    );
  }

  void _showMessage(String message) {
    ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text(message)));
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text("团队管理"),
        actions: [IconButton(onPressed: _load, icon: const Icon(Icons.refresh))],
      ),
      body: Column(
        children: [
          Padding(
            padding: const EdgeInsets.all(16),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                const Text("创建团队", style: TextStyle(fontWeight: FontWeight.bold)),
                const SizedBox(height: 8),
                TextField(
                  controller: _nameController,
                  decoration: const InputDecoration(labelText: "团队名称"),
                ),
                const SizedBox(height: 8),
                FilledButton(
                  onPressed: _creating ? null : _createTeam,
                  child: Text(_creating ? "创建中..." : "创建团队"),
                ),
              ],
            ),
          ),
          const Divider(height: 1),
          Expanded(
            child: _loading
                ? const Center(child: CircularProgressIndicator())
                : RefreshIndicator(
                    onRefresh: _load,
                    child: _teams.isEmpty
                        ? ListView(
                            children: const [
                              SizedBox(height: 120),
                              Center(child: Text("暂无团队")),
                            ],
                          )
                        : ListView.separated(
                            padding: const EdgeInsets.all(16),
                            itemBuilder: (context, index) {
                              final item = _teams[index];
                              final name = item["name"]?.toString() ?? "团队";
                              final role = item["role"]?.toString() ?? "-";
                              final status = item["status"]?.toString() ?? "-";
                              return ListTile(
                                tileColor: Colors.white,
                                title: Text(name),
                                subtitle: Text("角色：$role / 状态：$status"),
                                trailing: const Icon(Icons.chevron_right),
                                onTap: () => _openDetail(item),
                              );
                            },
                            separatorBuilder: (_, __) => const SizedBox(height: 12),
                            itemCount: _teams.length,
                          ),
                  ),
          ),
        ],
      ),
    );
  }
}
