import "package:flutter/material.dart";

import "../services/api_client.dart";
import "../services/session.dart";

class LoginPage extends StatefulWidget {
  const LoginPage({super.key});

  @override
  State<LoginPage> createState() => _LoginPageState();
}

class _LoginPageState extends State<LoginPage> {
  final TextEditingController _phoneController = TextEditingController();
  final TextEditingController _codeController = TextEditingController();
  bool _loading = false;

  @override
  void dispose() {
    _phoneController.dispose();
    _codeController.dispose();
    super.dispose();
  }

  Future<void> _sendCode() async {
    final phone = _phoneController.text.trim();
    if (phone.length != 11) {
      _showMessage("请输入 11 位手机号");
      return;
    }
    setState(() => _loading = true);
    try {
      await ApiClient.post("/auth/code", {"phone": phone});
      _showMessage("验证码已发送");
    } catch (error) {
      _showMessage("发送失败：$error");
    } finally {
      if (mounted) {
        setState(() => _loading = false);
      }
    }
  }

  Future<void> _login() async {
    final phone = _phoneController.text.trim();
    final code = _codeController.text.trim();
    if (phone.length != 11 || code.length != 6) {
      _showMessage("请输入正确的手机号和 6 位验证码");
      return;
    }
    setState(() => _loading = true);
    try {
      final data = await ApiClient.post("/auth/login", {"phone": phone, "code": code});
      final token = data["token"] as String?;
      if (token == null || token.isEmpty) {
        throw ApiException("token_missing");
      }
      await SessionStore.setToken(token);
    } catch (error) {
      _showMessage("登录失败：$error");
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
      body: Center(
        child: Padding(
          padding: const EdgeInsets.all(24),
          child: Column(
            mainAxisSize: MainAxisSize.min,
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              const Text(
                "约拍平台",
                textAlign: TextAlign.center,
                style: TextStyle(fontSize: 28, fontWeight: FontWeight.bold),
              ),
              const SizedBox(height: 24),
              TextField(
                controller: _phoneController,
                keyboardType: TextInputType.phone,
                maxLength: 11,
                decoration: const InputDecoration(
                  labelText: "手机号",
                  prefixIcon: Icon(Icons.phone_android),
                ),
              ),
              const SizedBox(height: 12),
              TextField(
                controller: _codeController,
                keyboardType: TextInputType.number,
                maxLength: 6,
                decoration: const InputDecoration(
                  labelText: "验证码",
                  prefixIcon: Icon(Icons.verified_outlined),
                ),
              ),
              const SizedBox(height: 16),
              ElevatedButton(
                onPressed: _loading ? null : _sendCode,
                child: const Text("发送验证码"),
              ),
              const SizedBox(height: 8),
              FilledButton(
                onPressed: _loading ? null : _login,
                child: const Text("登录"),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
