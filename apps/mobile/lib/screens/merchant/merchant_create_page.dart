import "package:flutter/material.dart";

import "../../services/api_client.dart";

class MerchantCreatePage extends StatefulWidget {
  final int userId;
  const MerchantCreatePage({super.key, required this.userId});

  @override
  State<MerchantCreatePage> createState() => _MerchantCreatePageState();
}

class _MerchantCreatePageState extends State<MerchantCreatePage> {
  final TextEditingController _nameController = TextEditingController();
  final TextEditingController _logoController = TextEditingController();
  final TextEditingController _brandController = TextEditingController();
  bool _loading = false;

  @override
  void dispose() {
    _nameController.dispose();
    _logoController.dispose();
    _brandController.dispose();
    super.dispose();
  }

  Future<void> _submit() async {
    final name = _nameController.text.trim();
    if (name.isEmpty) {
      _showMessage("请填写商户名称");
      return;
    }
    setState(() => _loading = true);
    try {
      await ApiClient.post("/merchants", {
        "name": name,
        "logo_url": _logoController.text.trim().isEmpty ? null : _logoController.text.trim(),
        "brand_color": _brandController.text.trim().isEmpty
            ? null
            : _brandController.text.trim(),
        "contact_user_id": widget.userId
      });
      if (mounted) {
        _showMessage("创建成功");
        Navigator.of(context).pop();
      }
    } catch (error) {
      _showMessage("创建失败：$error");
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
    return Scaffold(
      appBar: AppBar(title: const Text("创建商户")),
      body: ListView(
        padding: const EdgeInsets.all(16),
        children: [
          TextField(
            controller: _nameController,
            decoration: const InputDecoration(labelText: "商户名称"),
          ),
          TextField(
            controller: _logoController,
            decoration: const InputDecoration(labelText: "Logo 链接（可选）"),
          ),
          TextField(
            controller: _brandController,
            decoration: const InputDecoration(labelText: "品牌色（可选）"),
          ),
          const SizedBox(height: 16),
          FilledButton(
            onPressed: _loading ? null : _submit,
            child: Text(_loading ? "提交中..." : "提交"),
          ),
        ],
      ),
    );
  }
}