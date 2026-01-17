import "package:flutter/material.dart";

import "../../services/api_client.dart";

class MerchantLocationListPage extends StatefulWidget {
  final int merchantId;
  final String merchantName;
  const MerchantLocationListPage({
    super.key,
    required this.merchantId,
    required this.merchantName,
  });

  @override
  State<MerchantLocationListPage> createState() => _MerchantLocationListPageState();
}

class _MerchantLocationListPageState extends State<MerchantLocationListPage> {
  bool _loading = false;
  bool _creating = false;
  List<Map<String, dynamic>> _locations = [];

  final TextEditingController _nameController = TextEditingController();
  final TextEditingController _addressController = TextEditingController();
  final TextEditingController _cityController = TextEditingController();

  @override
  void initState() {
    super.initState();
    _load();
  }

  @override
  void dispose() {
    _nameController.dispose();
    _addressController.dispose();
    _cityController.dispose();
    super.dispose();
  }

  Future<void> _load() async {
    setState(() => _loading = true);
    try {
      final data = await ApiClient.get("/merchants/${widget.merchantId}/locations");
      if (data is List) {
        _locations = data.cast<Map<String, dynamic>>();
      } else {
        _locations = [];
      }
    } catch (error) {
      _locations = [];
      _showMessage("加载失败：$error");
    } finally {
      if (mounted) {
        setState(() => _loading = false);
      }
    }
  }

  Future<void> _createLocation() async {
    final name = _nameController.text.trim();
    if (name.isEmpty) {
      _showMessage("请填写门店名称");
      return;
    }
    final cityId = int.tryParse(_cityController.text.trim());
    setState(() => _creating = true);
    try {
      await ApiClient.post("/merchants/${widget.merchantId}/locations", {
        "name": name,
        "address": _addressController.text.trim().isEmpty ? null : _addressController.text.trim(),
        "city_id": cityId
      });
      _nameController.clear();
      _addressController.clear();
      _cityController.clear();
      await _load();
    } catch (error) {
      _showMessage("创建失败：$error");
    } finally {
      if (mounted) {
        setState(() => _creating = false);
      }
    }
  }

  Future<void> _editLocation(Map<String, dynamic> location) async {
    final nameController = TextEditingController(text: location["name"]?.toString() ?? "");
    final addressController =
        TextEditingController(text: location["address"]?.toString() ?? "");
    final cityController =
        TextEditingController(text: location["city_id"]?.toString() ?? "");
    final ok = await showDialog<bool>(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text("编辑门店"),
        content: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            TextField(
              controller: nameController,
              decoration: const InputDecoration(labelText: "门店名称"),
            ),
            TextField(
              controller: addressController,
              decoration: const InputDecoration(labelText: "地址"),
            ),
            TextField(
              controller: cityController,
              decoration: const InputDecoration(labelText: "城市 ID"),
              keyboardType: TextInputType.number,
            ),
          ],
        ),
        actions: [
          TextButton(onPressed: () => Navigator.of(context).pop(false), child: const Text("取消")),
          FilledButton(onPressed: () => Navigator.of(context).pop(true), child: const Text("保存")),
        ],
      ),
    );
    if (ok != true) {
      return;
    }
    try {
      await ApiClient.put(
        "/merchants/${widget.merchantId}/locations/${location["id"]}",
        {
          "name": nameController.text.trim(),
          "address": addressController.text.trim().isEmpty ? null : addressController.text.trim(),
          "city_id": int.tryParse(cityController.text.trim())
        },
      );
      await _load();
    } catch (error) {
      _showMessage("更新失败：$error");
    }
  }

  Future<void> _deleteLocation(int locationId) async {
    final confirm = await showDialog<bool>(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text("删除门店"),
        content: const Text("确认删除该门店吗？"),
        actions: [
          TextButton(onPressed: () => Navigator.of(context).pop(false), child: const Text("取消")),
          FilledButton(onPressed: () => Navigator.of(context).pop(true), child: const Text("删除")),
        ],
      ),
    );
    if (confirm != true) {
      return;
    }
    try {
      await ApiClient.delete("/merchants/${widget.merchantId}/locations/$locationId");
      await _load();
    } catch (error) {
      _showMessage("删除失败：$error");
    }
  }

  void _showMessage(String message) {
    ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text(message)));
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text("${widget.merchantName} · 门店"),
        actions: [IconButton(onPressed: _load, icon: const Icon(Icons.refresh))],
      ),
      body: Column(
        children: [
          Padding(
            padding: const EdgeInsets.all(16),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                const Text("新增门店", style: TextStyle(fontWeight: FontWeight.bold)),
                const SizedBox(height: 8),
                TextField(
                  controller: _nameController,
                  decoration: const InputDecoration(labelText: "门店名称"),
                ),
                TextField(
                  controller: _addressController,
                  decoration: const InputDecoration(labelText: "地址"),
                ),
                TextField(
                  controller: _cityController,
                  decoration: const InputDecoration(labelText: "城市 ID"),
                  keyboardType: TextInputType.number,
                ),
                const SizedBox(height: 8),
                FilledButton(
                  onPressed: _creating ? null : _createLocation,
                  child: Text(_creating ? "创建中..." : "创建门店"),
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
                    child: _locations.isEmpty
                        ? ListView(
                            children: const [
                              SizedBox(height: 120),
                              Center(child: Text("暂无门店")),
                            ],
                          )
                        : ListView.separated(
                            padding: const EdgeInsets.all(16),
                            itemBuilder: (context, index) {
                              final item = _locations[index];
                              return ListTile(
                                tileColor: Colors.white,
                                title: Text(item["name"]?.toString() ?? "门店"),
                                subtitle: Text(
                                  "地址：${item["address"] ?? "-"} / 城市：${item["city_id"] ?? "-"}",
                                ),
                                trailing: Row(
                                  mainAxisSize: MainAxisSize.min,
                                  children: [
                                    IconButton(
                                      icon: const Icon(Icons.edit_outlined),
                                      onPressed: () => _editLocation(item),
                                    ),
                                    IconButton(
                                      icon: const Icon(Icons.delete_outline, color: Colors.red),
                                      onPressed: () => _deleteLocation(item["id"] as int),
                                    ),
                                  ],
                                ),
                              );
                            },
                            separatorBuilder: (_, __) => const SizedBox(height: 12),
                            itemCount: _locations.length,
                          ),
                  ),
          ),
        ],
      ),
    );
  }
}
