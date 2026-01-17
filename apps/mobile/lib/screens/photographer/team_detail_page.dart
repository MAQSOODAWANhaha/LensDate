import "package:flutter/material.dart";

import "../../services/api_client.dart";

class TeamDetailPage extends StatefulWidget {
  final Map<String, dynamic> team;
  const TeamDetailPage({super.key, required this.team});

  @override
  State<TeamDetailPage> createState() => _TeamDetailPageState();
}

class _TeamDetailPageState extends State<TeamDetailPage> {
  bool _loading = false;
  bool _saving = false;
  List<Map<String, dynamic>> _members = [];
  late TextEditingController _nameController;
  final TextEditingController _memberIdController = TextEditingController();
  String _memberRole = "member";

  int get _teamId => widget.team["id"] as int? ?? 0;

  @override
  void initState() {
    super.initState();
    _nameController = TextEditingController(text: widget.team["name"]?.toString() ?? "");
    _load();
  }

  @override
  void dispose() {
    _nameController.dispose();
    _memberIdController.dispose();
    super.dispose();
  }

  Future<void> _load() async {
    setState(() => _loading = true);
    try {
      final data = await ApiClient.get("/teams/$_teamId/members");
      if (data is List) {
        _members = data.cast<Map<String, dynamic>>();
      } else {
        _members = [];
      }
    } catch (error) {
      _members = [];
      _showMessage("加载失败：$error");
    } finally {
      if (mounted) {
        setState(() => _loading = false);
      }
    }
  }

  Future<void> _saveName() async {
    final name = _nameController.text.trim();
    if (name.isEmpty) {
      _showMessage("请填写团队名称");
      return;
    }
    setState(() => _saving = true);
    try {
      await ApiClient.put("/teams/$_teamId", {"name": name});
      _showMessage("已更新团队名称");
    } catch (error) {
      _showMessage("保存失败：$error");
    } finally {
      if (mounted) {
        setState(() => _saving = false);
      }
    }
  }

  Future<void> _addMember() async {
    final userId = int.tryParse(_memberIdController.text.trim());
    if (userId == null) {
      _showMessage("请输入成员用户 ID");
      return;
    }
    try {
      await ApiClient.post("/teams/$_teamId/members", {
        "user_id": userId,
        "role": _memberRole
      });
      _memberIdController.clear();
      await _load();
    } catch (error) {
      _showMessage("添加失败：$error");
    }
  }

  Future<void> _removeMember(int userId) async {
    final confirm = await showDialog<bool>(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text("移除成员"),
        content: Text("确认移除成员 $userId 吗？"),
        actions: [
          TextButton(onPressed: () => Navigator.of(context).pop(false), child: const Text("取消")),
          FilledButton(onPressed: () => Navigator.of(context).pop(true), child: const Text("确认")),
        ],
      ),
    );
    if (confirm != true) {
      return;
    }
    try {
      await ApiClient.delete("/teams/$_teamId/members/$userId");
      await _load();
    } catch (error) {
      _showMessage("移除失败：$error");
    }
  }

  void _showMessage(String message) {
    ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text(message)));
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text("团队 #$_teamId"),
        actions: [IconButton(onPressed: _load, icon: const Icon(Icons.refresh))],
      ),
      body: Column(
        children: [
          Padding(
            padding: const EdgeInsets.all(16),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                const Text("团队信息", style: TextStyle(fontWeight: FontWeight.bold)),
                const SizedBox(height: 8),
                TextField(
                  controller: _nameController,
                  decoration: const InputDecoration(labelText: "团队名称"),
                ),
                const SizedBox(height: 8),
                FilledButton(
                  onPressed: _saving ? null : _saveName,
                  child: Text(_saving ? "保存中..." : "保存名称"),
                ),
              ],
            ),
          ),
          const Divider(height: 1),
          Padding(
            padding: const EdgeInsets.all(16),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                const Text("添加成员", style: TextStyle(fontWeight: FontWeight.bold)),
                const SizedBox(height: 8),
                TextField(
                  controller: _memberIdController,
                  decoration: const InputDecoration(labelText: "成员用户 ID"),
                  keyboardType: TextInputType.number,
                ),
                const SizedBox(height: 8),
                DropdownButtonFormField<String>(
                  key: ValueKey(_memberRole),
                  initialValue: _memberRole,
                  decoration: const InputDecoration(labelText: "角色"),
                  items: const [
                    DropdownMenuItem(value: "admin", child: Text("管理员")),
                    DropdownMenuItem(value: "member", child: Text("成员")),
                  ],
                  onChanged: (value) => setState(() => _memberRole = value ?? "member"),
                ),
                const SizedBox(height: 8),
                FilledButton(onPressed: _addMember, child: const Text("添加成员")),
              ],
            ),
          ),
          const Divider(height: 1),
          Expanded(
            child: _loading
                ? const Center(child: CircularProgressIndicator())
                : RefreshIndicator(
                    onRefresh: _load,
                    child: _members.isEmpty
                        ? ListView(
                            children: const [
                              SizedBox(height: 120),
                              Center(child: Text("暂无成员")),
                            ],
                          )
                        : ListView.separated(
                            padding: const EdgeInsets.all(16),
                            itemBuilder: (context, index) {
                              final member = _members[index];
                              final userId = member["user_id"] ?? "-";
                              final role = member["role"] ?? "-";
                              return ListTile(
                                tileColor: Colors.white,
                                title: Text("成员 #$userId"),
                                subtitle: Text("角色：$role"),
                                trailing: IconButton(
                                  icon: const Icon(Icons.delete_outline, color: Colors.red),
                                  onPressed: () => _removeMember(member["user_id"] as int),
                                ),
                              );
                            },
                            separatorBuilder: (_, __) => const SizedBox(height: 12),
                            itemCount: _members.length,
                          ),
                  ),
          ),
        ],
      ),
    );
  }
}
