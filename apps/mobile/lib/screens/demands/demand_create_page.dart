import "package:flutter/material.dart";
import "package:image_picker/image_picker.dart";

import "../../services/api_client.dart";
import "../../services/upload_service.dart";

class DemandCreatePage extends StatefulWidget {
  const DemandCreatePage({super.key});

  @override
  State<DemandCreatePage> createState() => _DemandCreatePageState();
}

class _DemandCreatePageState extends State<DemandCreatePage> {
  final TextEditingController _typeController = TextEditingController();
  final TextEditingController _cityController = TextEditingController();
  final TextEditingController _locationController = TextEditingController();
  final TextEditingController _startController = TextEditingController();
  final TextEditingController _endController = TextEditingController();
  final TextEditingController _budgetMinController = TextEditingController();
  final TextEditingController _budgetMaxController = TextEditingController();
  final TextEditingController _peopleController = TextEditingController();
  final TextEditingController _styleTagsController = TextEditingController();
  final TextEditingController _attachmentsController = TextEditingController();
  final TextEditingController _attachmentTypesController = TextEditingController();
  final TextEditingController _merchantIdController = TextEditingController();
  bool _isMerchant = false;
  bool _loading = false;
  bool _uploading = false;

  @override
  void initState() {
    super.initState();
    _attachmentsController.addListener(_refreshAttachments);
  }

  @override
  void dispose() {
    _attachmentsController.removeListener(_refreshAttachments);
    _typeController.dispose();
    _cityController.dispose();
    _locationController.dispose();
    _startController.dispose();
    _endController.dispose();
    _budgetMinController.dispose();
    _budgetMaxController.dispose();
    _peopleController.dispose();
    _styleTagsController.dispose();
    _attachmentsController.dispose();
    _attachmentTypesController.dispose();
    _merchantIdController.dispose();
    super.dispose();
  }

  void _refreshAttachments() {
    if (mounted) {
      setState(() {});
    }
  }

  List<String> _parseAttachmentUrls() {
    return _attachmentsController.text
        .split(",")
        .map((e) => e.trim())
        .where((e) => e.isNotEmpty)
        .toList();
  }

  bool _isImageUrl(String url) {
    final lower = url.toLowerCase();
    return lower.endsWith(".jpg") ||
        lower.endsWith(".jpeg") ||
        lower.endsWith(".png") ||
        lower.endsWith(".gif") ||
        lower.endsWith(".webp");
  }

  Future<void> _pickAndUpload(ImageSource source) async {
    setState(() => _uploading = true);
    try {
      final url = await UploadService.pickAndUpload(source);
      if (url != null && url.isNotEmpty) {
        _appendAttachmentUrl(url);
        _showMessage("上传成功");
      }
    } catch (error) {
      _showMessage("上传失败：$error");
    } finally {
      if (mounted) {
        setState(() => _uploading = false);
      }
    }
  }

  void _appendAttachmentUrl(String url) {
    final current = _attachmentsController.text.trim();
    if (current.isEmpty) {
      _attachmentsController.text = url;
    } else {
      _attachmentsController.text = "$current,$url";
    }
  }

  Future<void> _submit() async {
    final type = _typeController.text.trim();
    final cityId = int.tryParse(_cityController.text.trim());
    final start = _startController.text.trim();
    final end = _endController.text.trim();
    if (type.isEmpty || cityId == null || start.isEmpty || end.isEmpty) {
      _showMessage("请填写必填项");
      return;
    }
    final peopleCount = int.tryParse(_peopleController.text.trim());
    final styleTags = _styleTagsController.text
        .split(",")
        .map((e) => e.trim())
        .where((e) => e.isNotEmpty)
        .toList();
    final attachmentUrls = _parseAttachmentUrls();
    final attachmentTypes = _attachmentTypesController.text
        .split(",")
        .map((e) => e.trim())
        .where((e) => e.isNotEmpty)
        .toList();
    final attachments = <Map<String, dynamic>>[];
    for (var i = 0; i < attachmentUrls.length; i++) {
      final fileType = attachmentTypes.length > i ? attachmentTypes[i] : null;
      attachments.add({"file_url": attachmentUrls[i], "file_type": fileType});
    }
    final merchantId = int.tryParse(_merchantIdController.text.trim());
    if (_isMerchant && merchantId == null) {
      _showMessage("请选择商户需求并填写商户 ID");
      return;
    }

    setState(() => _loading = true);
    try {
      await ApiClient.post("/demands", {
        "type": type,
        "city_id": cityId,
        "location": _locationController.text.trim().isEmpty
            ? null
            : _locationController.text.trim(),
        "schedule_start": start,
        "schedule_end": end,
        "budget_min": double.tryParse(_budgetMinController.text.trim()),
        "budget_max": double.tryParse(_budgetMaxController.text.trim()),
        "people_count": peopleCount,
        "style_tags": styleTags.isEmpty ? null : styleTags,
        "attachments": attachments.isEmpty ? null : attachments,
        "is_merchant": _isMerchant,
        "merchant_id": _isMerchant ? merchantId : null
      });
      if (mounted) {
        _showMessage("需求已发布");
        Navigator.of(context).pop();
      }
    } catch (error) {
      _showMessage("发布失败：$error");
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
      appBar: AppBar(title: const Text("发布需求")),
      body: ListView(
        padding: const EdgeInsets.all(16),
        children: [
          TextField(
            controller: _typeController,
            decoration: const InputDecoration(labelText: "拍摄类型"),
          ),
          TextField(
            controller: _cityController,
            decoration: const InputDecoration(labelText: "城市 ID"),
            keyboardType: TextInputType.number,
          ),
          TextField(
            controller: _locationController,
            decoration: const InputDecoration(labelText: "拍摄地点"),
          ),
          TextField(
            controller: _startController,
            decoration: const InputDecoration(
              labelText: "开始时间（RFC3339）",
              helperText: "例如：2026-01-16T10:00:00+08:00",
            ),
          ),
          TextField(
            controller: _endController,
            decoration: const InputDecoration(
              labelText: "结束时间（RFC3339）",
              helperText: "例如：2026-01-16T12:00:00+08:00",
            ),
          ),
          TextField(
            controller: _budgetMinController,
            decoration: const InputDecoration(labelText: "预算下限"),
            keyboardType: TextInputType.number,
          ),
          TextField(
            controller: _budgetMaxController,
            decoration: const InputDecoration(labelText: "预算上限"),
            keyboardType: TextInputType.number,
          ),
          TextField(
            controller: _peopleController,
            decoration: const InputDecoration(labelText: "拍摄人数"),
            keyboardType: TextInputType.number,
          ),
          TextField(
            controller: _styleTagsController,
            decoration: const InputDecoration(
              labelText: "风格标签（逗号分隔）",
              helperText: "例如：清新,人像,室内",
            ),
          ),
          TextField(
            controller: _attachmentsController,
            decoration: const InputDecoration(
              labelText: "附件链接（逗号分隔）",
              helperText: "可粘贴多条图片/文件链接",
            ),
          ),
          TextField(
            controller: _attachmentTypesController,
            decoration: const InputDecoration(labelText: "附件类型（可选，逗号分隔）"),
          ),
          const SizedBox(height: 8),
          Row(
            children: [
              Expanded(
                child: OutlinedButton.icon(
                  onPressed: _uploading ? null : () => _pickAndUpload(ImageSource.gallery),
                  icon: const Icon(Icons.photo_library_outlined),
                  label: const Text("相册"),
                ),
              ),
              const SizedBox(width: 12),
              Expanded(
                child: OutlinedButton.icon(
                  onPressed: _uploading ? null : () => _pickAndUpload(ImageSource.camera),
                  icon: const Icon(Icons.camera_alt_outlined),
                  label: const Text("拍照"),
                ),
              ),
            ],
          ),
          if (_uploading)
            const Padding(
              padding: EdgeInsets.only(top: 8),
              child: LinearProgressIndicator(),
            ),
          const SizedBox(height: 8),
          if (_parseAttachmentUrls().isNotEmpty)
            Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                const Text("附件预览", style: TextStyle(fontWeight: FontWeight.bold)),
                const SizedBox(height: 8),
                for (final url in _parseAttachmentUrls())
                  Card(
                    child: Padding(
                      padding: const EdgeInsets.all(8),
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Text(url, style: const TextStyle(fontSize: 12)),
                          const SizedBox(height: 6),
                          if (_isImageUrl(url))
                            ClipRRect(
                              borderRadius: BorderRadius.circular(8),
                              child: Image.network(url, height: 160, fit: BoxFit.cover),
                            ),
                        ],
                      ),
                    ),
                  ),
              ],
            ),
          SwitchListTile(
            value: _isMerchant,
            onChanged: (value) => setState(() => _isMerchant = value),
            title: const Text("商户需求（瑜伽馆）"),
          ),
          if (_isMerchant)
            TextField(
              controller: _merchantIdController,
              decoration: const InputDecoration(labelText: "商户 ID"),
              keyboardType: TextInputType.number,
            ),
          const SizedBox(height: 24),
          FilledButton(
            onPressed: _loading ? null : _submit,
            child: const Text("提交需求"),
          )
        ],
      ),
    );
  }
}
