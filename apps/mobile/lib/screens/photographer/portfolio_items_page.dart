import "package:flutter/material.dart";
import "package:image_picker/image_picker.dart";

import "../../services/api_client.dart";
import "../../services/upload_service.dart";

class PortfolioItemsPage extends StatefulWidget {
  final int portfolioId;
  final String title;
  final bool readOnly;
  const PortfolioItemsPage({
    super.key,
    required this.portfolioId,
    required this.title,
    this.readOnly = false,
  });

  @override
  State<PortfolioItemsPage> createState() => _PortfolioItemsPageState();
}

class _PortfolioItemsPageState extends State<PortfolioItemsPage> {
  bool _loading = false;
  bool _creating = false;
  List<Map<String, dynamic>> _items = [];
  final TextEditingController _urlController = TextEditingController();
  final TextEditingController _tagsController = TextEditingController();
  bool _cover = false;

  @override
  void initState() {
    super.initState();
    _load();
  }

  @override
  void dispose() {
    _urlController.dispose();
    _tagsController.dispose();
    super.dispose();
  }

  Future<void> _load() async {
    setState(() => _loading = true);
    try {
      final data = await ApiClient.get("/portfolios/${widget.portfolioId}/items");
      if (data is List) {
        _items = data.cast<Map<String, dynamic>>();
      } else {
        _items = [];
      }
    } catch (error) {
      _items = [];
      _showMessage("加载失败：$error");
    } finally {
      if (mounted) {
        setState(() => _loading = false);
      }
    }
  }

  Future<void> _pickUpload(ImageSource source) async {
    try {
      final url = await UploadService.pickAndUpload(source);
      if (url != null && url.isNotEmpty) {
        _urlController.text = url;
      }
    } catch (error) {
      _showMessage("上传失败：$error");
    }
  }

  Future<void> _createItem() async {
    final url = _urlController.text.trim();
    if (url.isEmpty) {
      _showMessage("请填写作品链接");
      return;
    }
    final tags = _tagsController.text
        .split(",")
        .map((e) => e.trim())
        .where((e) => e.isNotEmpty)
        .toList();

    setState(() => _creating = true);
    try {
      await ApiClient.post("/portfolios/${widget.portfolioId}/items", {
        "url": url,
        "tags": tags.isEmpty ? null : tags,
        "cover_flag": _cover,
      });
      _urlController.clear();
      _tagsController.clear();
      setState(() => _cover = false);
      await _load();
    } catch (error) {
      _showMessage("创建失败：$error");
    } finally {
      if (mounted) {
        setState(() => _creating = false);
      }
    }
  }

  void _showMessage(String message) {
    ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text(message)));
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text(widget.title),
        actions: [IconButton(onPressed: _load, icon: const Icon(Icons.refresh))],
      ),
      body: Column(
        children: [
          if (!widget.readOnly) ...[
            Padding(
              padding: const EdgeInsets.all(16),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  const Text("新增作品", style: TextStyle(fontWeight: FontWeight.bold)),
                  const SizedBox(height: 8),
                  TextField(
                    controller: _urlController,
                    decoration: const InputDecoration(labelText: "作品链接"),
                  ),
                  Row(
                    children: [
                      TextButton.icon(
                        onPressed: () => _pickUpload(ImageSource.gallery),
                        icon: const Icon(Icons.photo_library_outlined),
                        label: const Text("选择图片"),
                      ),
                      TextButton.icon(
                        onPressed: () => _pickUpload(ImageSource.camera),
                        icon: const Icon(Icons.photo_camera_outlined),
                        label: const Text("拍照上传"),
                      ),
                    ],
                  ),
                  TextField(
                    controller: _tagsController,
                    decoration: const InputDecoration(labelText: "标签（逗号分隔）"),
                  ),
                  SwitchListTile(
                    value: _cover,
                    onChanged: (value) => setState(() => _cover = value),
                    title: const Text("设为封面"),
                  ),
                  FilledButton(
                    onPressed: _creating ? null : _createItem,
                    child: Text(_creating ? "提交中..." : "新增作品"),
                  ),
                ],
              ),
            ),
            const Divider(height: 1),
          ],
          Expanded(
            child: _loading
                ? const Center(child: CircularProgressIndicator())
                : RefreshIndicator(
                    onRefresh: _load,
                    child: _items.isEmpty
                        ? ListView(
                            children: const [
                              SizedBox(height: 120),
                              Center(child: Text("暂无作品")),
                            ],
                          )
                        : ListView.separated(
                            padding: const EdgeInsets.all(16),
                            itemBuilder: (context, index) {
                              final item = _items[index];
                              return ListTile(
                                tileColor: Colors.white,
                                title: Text(item["url"]?.toString() ?? ""),
                                subtitle: Text("ID：${item["id"] ?? "-"}"),
                              );
                            },
                            separatorBuilder: (_, __) => const SizedBox(height: 12),
                            itemCount: _items.length,
                          ),
                  ),
          ),
        ],
      ),
    );
  }
}
