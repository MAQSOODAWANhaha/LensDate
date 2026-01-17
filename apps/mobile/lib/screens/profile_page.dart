import "package:flutter/material.dart";

import "../services/api_client.dart";
import "../services/session.dart";
import "photographer/photographer_center_page.dart";
import "merchant/merchant_center_page.dart";

class ProfilePage extends StatefulWidget {
  const ProfilePage({super.key});

  @override
  State<ProfilePage> createState() => _ProfilePageState();
}

class _ProfilePageState extends State<ProfilePage> {
  final TextEditingController _nicknameController = TextEditingController();
  final TextEditingController _avatarController = TextEditingController();
  final TextEditingController _cityController = TextEditingController();
  final TextEditingController _bioController = TextEditingController();
  String _gender = "unknown";
  bool _loading = false;

  @override
  void initState() {
    super.initState();
    _loadProfile();
  }

  @override
  void dispose() {
    _nicknameController.dispose();
    _avatarController.dispose();
    _cityController.dispose();
    _bioController.dispose();
    super.dispose();
  }

  Future<void> _loadProfile() async {
    setState(() => _loading = true);
    try {
      final data = await ApiClient.get("/users/me");
      if (data is Map<String, dynamic>) {
        final profile = data["profile"] as Map<String, dynamic>? ?? {};
        _nicknameController.text = profile["nickname"]?.toString() ?? "";
        _avatarController.text = profile["avatar_url"]?.toString() ?? "";
        _cityController.text = profile["city_id"]?.toString() ?? "";
        _bioController.text = profile["bio"]?.toString() ?? "";
        final gender = profile["gender"]?.toString();
        if (gender == "male" || gender == "female" || gender == "unknown") {
          _gender = gender ?? "unknown";
        }
      }
    } catch (error) {
      _showMessage("加载失败：$error");
    } finally {
      if (mounted) {
        setState(() => _loading = false);
      }
    }
  }

  Future<void> _saveProfile() async {
    final cityId = int.tryParse(_cityController.text.trim());
    setState(() => _loading = true);
    try {
      await ApiClient.put("/users/me", {
        "nickname": _nicknameController.text.trim().isEmpty
            ? null
            : _nicknameController.text.trim(),
        "avatar_url": _avatarController.text.trim().isEmpty
            ? null
            : _avatarController.text.trim(),
        "gender": _gender,
        "city_id": cityId,
        "bio": _bioController.text.trim().isEmpty ? null : _bioController.text.trim()
      });
      _showMessage("保存成功");
    } catch (error) {
      _showMessage("保存失败：$error");
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
      appBar: AppBar(
        title: const Text("个人中心"),
        actions: [IconButton(onPressed: _loadProfile, icon: const Icon(Icons.refresh))],
      ),
      body: _loading
          ? const Center(child: CircularProgressIndicator())
          : ListView(
              padding: const EdgeInsets.all(16),
              children: [
                TextField(
                  controller: _nicknameController,
                  decoration: const InputDecoration(labelText: "昵称"),
                ),
                TextField(
                  controller: _avatarController,
                  decoration: const InputDecoration(labelText: "头像链接"),
                ),
                DropdownButtonFormField<String>(
                  initialValue: _gender,
                  decoration: const InputDecoration(labelText: "性别"),
                  items: const [
                    DropdownMenuItem(value: "unknown", child: Text("保密")),
                    DropdownMenuItem(value: "male", child: Text("男")),
                    DropdownMenuItem(value: "female", child: Text("女")),
                  ],
                  onChanged: (value) => setState(() => _gender = value ?? "unknown"),
                ),
                TextField(
                  controller: _cityController,
                  decoration: const InputDecoration(labelText: "城市 ID"),
                  keyboardType: TextInputType.number,
                ),
                TextField(
                  controller: _bioController,
                  decoration: const InputDecoration(labelText: "简介"),
                  maxLines: 3,
                ),
                const SizedBox(height: 16),
                FilledButton(onPressed: _saveProfile, child: const Text("保存资料")),
                const SizedBox(height: 16),
                const Divider(),
                ListTile(
                  leading: const Icon(Icons.photo_camera_outlined),
                  title: const Text("摄影师中心"),
                  subtitle: const Text("申请、订单、交付、作品集"),
                  onTap: () {
                    Navigator.of(context).push(
                      MaterialPageRoute(builder: (_) => const PhotographerCenterPage()),
                    );
                  },
                ),
                ListTile(
                  leading: const Icon(Icons.storefront_outlined),
                  title: const Text("商户中心"),
                  subtitle: const Text("模板、审批、订单、合同、发票"),
                  onTap: () {
                    Navigator.of(context).push(
                      MaterialPageRoute(builder: (_) => const MerchantCenterPage()),
                    );
                  },
                ),
                const SizedBox(height: 8),
                OutlinedButton(
                  onPressed: () async {
                    await SessionStore.clear();
                  },
                  child: const Text("退出登录"),
                ),
              ],
            ),
    );
  }
}
