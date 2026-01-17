import "package:flutter/material.dart";

import "../../services/api_client.dart";

class MerchantMemberListPage extends StatefulWidget {
  final int merchantId;
  final String merchantName;
  const MerchantMemberListPage({
    super.key,
    required this.merchantId,
    required this.merchantName,
  });

  @override
  State<MerchantMemberListPage> createState() => _MerchantMemberListPageState();
}

class _MerchantMemberListPageState extends State<MerchantMemberListPage> {
  bool _loading = false;
  List<Map<String, dynamic>> _members = [];
  final TextEditingController _userIdController = TextEditingController();
  String _role = "requester";

  @override
  void initState() {
    super.initState();
    _load();
  }

  @override
  void dispose() {
    _userIdController.dispose();
    super.dispose();
  }

  Future<void> _load() async {
    setState(() => _loading = true);
    try {
      final data = await ApiClient.get("/merchants/${widget.merchantId}/members");
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

  Future<void> _addMember() async {
    final userId = int.tryParse(_userIdController.text.trim());
    if (userId == null) {
      _showMessage("请输入用户 ID");
      return;
    }
    try {
      await ApiClient.post("/merchants/${widget.merchantId}/members", {
        "user_id": userId,
        "role": _role
      });
      _userIdController.clear();
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
          FilledButton(onPressed: () => Navigator.of(context).pop(true), child: const Text("移除")),
        ],
      ),
    );
    if (confirm != true) {
      return;
    }
    try {
      await ApiClient.delete("/merchants/${widget.merchantId}/members/$userId");
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
        title: Text("${widget.merchantName} · 成员"),
        actions: [IconButton(onPressed: _load, icon: const Icon(Icons.refresh))],
      ),
      body: Column(
        children: [
          Padding(
            padding: const EdgeInsets.all(16),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                const Text("添加成员", style: TextStyle(fontWeight: FontWeight.bold)),
                const SizedBox(height: 8),
                TextField(
                  controller: _userIdController,
                  decoration: const InputDecoration(labelText: "用户 ID"),
                  keyboardType: TextInputType.number,
                ),
                const SizedBox(height: 8),
                DropdownButtonFormField<String>(
                  key: ValueKey(_role),
                  initialValue: _role,
                  decoration: const InputDecoration(labelText: "角色"),
                  items: const [
                    DropdownMenuItem(value: "requester", child: Text("发起人")),
                    DropdownMenuItem(value: "approver", child: Text("负责人")),
                    DropdownMenuItem(value: "finance", child: Text("财务")),
                  ],
                  onChanged: (value) => setState(() => _role = value ?? "requester"),
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
                              final item = _members[index];
                              final userId = item["user_id"] ?? "-";
                              final role = item["role"] ?? "-";
                              return ListTile(
                                tileColor: Colors.white,
                                title: Text("成员 #$userId"),
                                subtitle: Text("角色：$role"),
                                trailing: IconButton(
                                  icon: const Icon(Icons.delete_outline, color: Colors.red),
                                  onPressed: () => _removeMember(item["user_id"] as int),
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
