import "package:flutter/material.dart";

import "../../services/api_client.dart";

class PhotographerApplyPage extends StatefulWidget {
  const PhotographerApplyPage({super.key});

  @override
  State<PhotographerApplyPage> createState() => _PhotographerApplyPageState();
}

class _PhotographerApplyPageState extends State<PhotographerApplyPage> {
  String _type = "individual";
  final TextEditingController _cityController = TextEditingController();
  final TextEditingController _serviceAreaController = TextEditingController();
  bool _loading = false;

  @override
  void dispose() {
    _cityController.dispose();
    _serviceAreaController.dispose();
    super.dispose();
  }

  Future<void> _submit() async {
    final cityId = int.tryParse(_cityController.text.trim());
    if (cityId == null) {
      _showMessage("请填写城市 ID");
      return;
    }
    setState(() => _loading = true);
    try {
      await ApiClient.post("/photographers", {
        "type": _type,
        "city_id": cityId,
        "service_area": _serviceAreaController.text.trim().isEmpty
            ? null
            : _serviceAreaController.text.trim()
      });
      if (mounted) {
        _showMessage("提交成功");
        Navigator.of(context).pop();
      }
    } catch (error) {
      _showMessage("提交失败：$error");
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
      appBar: AppBar(title: const Text("申请摄影师/团队")),
      body: ListView(
        padding: const EdgeInsets.all(16),
        children: [
          DropdownButtonFormField<String>(
            key: ValueKey(_type),
            initialValue: _type,
            decoration: const InputDecoration(labelText: "类型"),
            items: const [
              DropdownMenuItem(value: "individual", child: Text("个人摄影师")),
              DropdownMenuItem(value: "team", child: Text("摄影团队")),
            ],
            onChanged: (value) => setState(() => _type = value ?? "individual"),
          ),
          TextField(
            controller: _cityController,
            decoration: const InputDecoration(labelText: "城市 ID"),
            keyboardType: TextInputType.number,
          ),
          TextField(
            controller: _serviceAreaController,
            decoration: const InputDecoration(labelText: "服务范围"),
          ),
          const SizedBox(height: 16),
          FilledButton(
            onPressed: _loading ? null : _submit,
            child: Text(_loading ? "提交中..." : "提交申请"),
          ),
        ],
      ),
    );
  }
}
