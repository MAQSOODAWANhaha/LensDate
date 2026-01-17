import "dart:convert";

import "package:flutter/material.dart";

import "../../services/api_client.dart";
import "merchant_asset_versions_page.dart";

class MerchantAssetListPage extends StatefulWidget {
  final int merchantId;
  final String merchantName;
  const MerchantAssetListPage({super.key, required this.merchantId, required this.merchantName});

  @override
  State<MerchantAssetListPage> createState() => _MerchantAssetListPageState();
}

class _MerchantAssetListPageState extends State<MerchantAssetListPage> {
  bool _loading = false;
  bool _loadingMore = false;
  bool _creating = false;
  List<Map<String, dynamic>> _assets = [];
  int _page = 1;
  bool _hasMore = true;
  static const int _pageSize = 20;
  final ScrollController _scrollController = ScrollController();

  final TextEditingController _nameController = TextEditingController();
  final TextEditingController _payloadController = TextEditingController();

  String _createType = "logo";
  String _createStatus = "active";
  String _filterType = "";
  String _filterStatus = "";

  @override
  void initState() {
    super.initState();
    _scrollController.addListener(_handleScroll);
    _load(reset: true);
  }

  @override
  void dispose() {
    _nameController.dispose();
    _payloadController.dispose();
    _scrollController.removeListener(_handleScroll);
    _scrollController.dispose();
    super.dispose();
  }

  void _handleScroll() {
    if (_scrollController.position.pixels >=
        _scrollController.position.maxScrollExtent - 200) {
      _loadMore();
    }
  }

  Future<void> _load({bool reset = false}) async {
    if (reset) {
      _page = 1;
      _hasMore = true;
      _assets = [];
      setState(() => _loading = true);
    } else {
      setState(() => _loadingMore = true);
    }
    try {
      final query = _buildQuery();
      final data = await ApiClient.get(
        "/merchants/${widget.merchantId}/assets?$query",
      );
      if (data is Map<String, dynamic>) {
        final rawItems = data["items"];
        final list = rawItems is List
            ? rawItems.cast<Map<String, dynamic>>()
            : <Map<String, dynamic>>[];
        final total = (data["total"] as num?)?.toInt() ?? list.length;
        if (reset) {
          _assets = list;
        } else {
          _assets.addAll(list);
        }
        if (list.isEmpty) {
          _hasMore = false;
        } else {
          _hasMore = _assets.length < total;
          if (_hasMore) {
            _page += 1;
          }
        }
      } else if (data is List) {
        final list = data.cast<Map<String, dynamic>>();
        if (reset) {
          _assets = list;
        } else {
          _assets.addAll(list);
        }
        _hasMore = list.length == _pageSize;
        if (_hasMore) {
          _page += 1;
        }
      } else if (reset) {
        _assets = [];
        _hasMore = false;
      }
    } catch (error) {
      if (reset) {
        _assets = [];
        _hasMore = false;
      }
      _showMessage("加载失败：$error");
    } finally {
      if (mounted) {
        setState(() {
          _loading = false;
          _loadingMore = false;
        });
      }
    }
  }

  String _buildQuery() {
    final buffer = StringBuffer();
    buffer.write("page=$_page&page_size=$_pageSize");
    if (_filterType.isNotEmpty) {
      buffer.write("&asset_type=$_filterType");
    }
    if (_filterStatus.isNotEmpty) {
      buffer.write("&status=$_filterStatus");
    }
    return buffer.toString();
  }

  Future<void> _loadMore() async {
    if (_loadingMore || !_hasMore || _loading) {
      return;
    }
    await _load();
  }

  Future<void> _createAsset() async {
    final name = _nameController.text.trim();
    if (name.isEmpty) {
      _showMessage("请填写素材名称");
      return;
    }

    Object? payload;
    final payloadText = _payloadController.text.trim();
    if (payloadText.isNotEmpty) {
      try {
        payload = jsonDecode(payloadText);
      } catch (_) {
        _showMessage("内容必须是合法 JSON");
        return;
      }
    }

    setState(() => _creating = true);
    try {
      final body = {
        "name": name,
        "asset_type": _createType,
        "status": _createStatus,
        if (payload != null) "payload": payload,
      };
      await ApiClient.post("/merchants/${widget.merchantId}/assets", body);
      _nameController.clear();
      _payloadController.clear();
      await _load(reset: true);
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

  String _formatPayload(Object? payload) {
    if (payload == null) {
      return "-";
    }
    String text;
    try {
      text = jsonEncode(payload);
    } catch (_) {
      text = payload.toString();
    }
    if (text.length > 200) {
      return "${text.substring(0, 200)}...";
    }
    return text;
  }

  Widget _buildFilterSection() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text("筛选", style: TextStyle(fontWeight: FontWeight.bold)),
        const SizedBox(height: 8),
        Row(
          children: [
            Expanded(
              child: DropdownButtonFormField<String>(
                initialValue: _filterType,
                decoration: const InputDecoration(labelText: "类型"),
                items: const [
                  DropdownMenuItem(value: "", child: Text("全部")),
                  DropdownMenuItem(value: "logo", child: Text("logo")),
                  DropdownMenuItem(value: "brand", child: Text("brand")),
                  DropdownMenuItem(value: "style", child: Text("style")),
                  DropdownMenuItem(value: "reference", child: Text("reference")),
                ],
                onChanged: (value) => setState(() => _filterType = value ?? ""),
              ),
            ),
            const SizedBox(width: 12),
            Expanded(
              child: DropdownButtonFormField<String>(
                initialValue: _filterStatus,
                decoration: const InputDecoration(labelText: "状态"),
                items: const [
                  DropdownMenuItem(value: "", child: Text("全部")),
                  DropdownMenuItem(value: "active", child: Text("active")),
                  DropdownMenuItem(value: "archived", child: Text("archived")),
                ],
                onChanged: (value) => setState(() => _filterStatus = value ?? ""),
              ),
            ),
          ],
        ),
        const SizedBox(height: 8),
        FilledButton.icon(
          onPressed: () => _load(reset: true),
          icon: const Icon(Icons.search),
          label: const Text("应用筛选"),
        ),
      ],
    );
  }

  Widget _buildCreateSection() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text("新增素材", style: TextStyle(fontWeight: FontWeight.bold)),
        const SizedBox(height: 8),
        TextField(
          controller: _nameController,
          decoration: const InputDecoration(labelText: "素材名称"),
        ),
        const SizedBox(height: 8),
        Row(
          children: [
            Expanded(
              child: DropdownButtonFormField<String>(
                initialValue: _createType,
                decoration: const InputDecoration(labelText: "类型"),
                items: const [
                  DropdownMenuItem(value: "logo", child: Text("logo")),
                  DropdownMenuItem(value: "brand", child: Text("brand")),
                  DropdownMenuItem(value: "style", child: Text("style")),
                  DropdownMenuItem(value: "reference", child: Text("reference")),
                ],
                onChanged: (value) => setState(() => _createType = value ?? "logo"),
              ),
            ),
            const SizedBox(width: 12),
            Expanded(
              child: DropdownButtonFormField<String>(
                initialValue: _createStatus,
                decoration: const InputDecoration(labelText: "状态"),
                items: const [
                  DropdownMenuItem(value: "active", child: Text("active")),
                  DropdownMenuItem(value: "archived", child: Text("archived")),
                ],
                onChanged: (value) => setState(() => _createStatus = value ?? "active"),
              ),
            ),
          ],
        ),
        const SizedBox(height: 8),
        TextField(
          controller: _payloadController,
          decoration: const InputDecoration(labelText: "内容 JSON（可选）"),
          maxLines: 3,
        ),
        const SizedBox(height: 8),
        FilledButton(
          onPressed: _creating ? null : _createAsset,
          child: Text(_creating ? "提交中..." : "创建素材"),
        ),
      ],
    );
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text("${widget.merchantName} 素材库"),
        actions: [
          IconButton(onPressed: () => _load(reset: true), icon: const Icon(Icons.refresh))
        ],
      ),
      body: Column(
        children: [
          Expanded(
            child: _loading
                ? const Center(child: CircularProgressIndicator())
                : RefreshIndicator(
                    onRefresh: () => _load(reset: true),
                    child: ListView(
                      controller: _scrollController,
                      padding: const EdgeInsets.all(16),
                      children: [
                        _buildFilterSection(),
                        const SizedBox(height: 16),
                        const Divider(),
                        _buildCreateSection(),
                        const SizedBox(height: 16),
                        const Divider(),
                        const Text("素材列表", style: TextStyle(fontWeight: FontWeight.bold)),
                        const SizedBox(height: 8),
                        if (_assets.isEmpty)
                          const Text("暂无素材")
                        else
                          for (final asset in _assets)
                            Card(
                              margin: const EdgeInsets.symmetric(vertical: 8),
                              child: Padding(
                                padding: const EdgeInsets.all(12),
                                child: Column(
                                  crossAxisAlignment: CrossAxisAlignment.start,
                                  children: [
                                    Text(
                                      asset["name"]?.toString() ?? "素材",
                                      style: const TextStyle(fontWeight: FontWeight.bold),
                                    ),
                                    const SizedBox(height: 4),
                                    Text("类型：${asset["asset_type"] ?? "-"}"),
                                    Text("状态：${asset["status"] ?? "-"}"),
                                    Text("最新版本：${asset["latest_version"] ?? "-"}"),
                                    Text("更新时间：${asset["updated_at"] ?? "-"}"),
                                    const SizedBox(height: 4),
                                    Text("内容：${_formatPayload(asset["latest_payload"])}"),
                                    const SizedBox(height: 8),
                                    Align(
                                      alignment: Alignment.centerRight,
                                      child: TextButton(
                                        onPressed: () {
                                          final assetId = asset["id"] as int? ?? 0;
                                          if (assetId == 0) {
                                            _showMessage("素材ID异常");
                                            return;
                                          }
                                          Navigator.of(context).push(
                                            MaterialPageRoute(
                                              builder: (_) => MerchantAssetVersionsPage(
                                                assetId: assetId,
                                                assetName: asset["name"]?.toString() ?? "素材",
                                              ),
                                            ),
                                          );
                                        },
                                        child: const Text("查看版本"),
                                      ),
                                    ),
                                  ],
                                ),
                              ),
                            ),
                        if (_loadingMore)
                          const Padding(
                            padding: EdgeInsets.symmetric(vertical: 12),
                            child: Center(child: CircularProgressIndicator()),
                          ),
                      ],
                    ),
                  ),
          ),
        ],
      ),
    );
  }
}
