import "package:flutter/material.dart";

import "services/session.dart";
import "screens/login_page.dart";
import "screens/home_page.dart";

class App extends StatelessWidget {
  const App({super.key});

  @override
  Widget build(BuildContext context) {
    return ValueListenableBuilder<bool>(
      valueListenable: SessionStore.authed,
      builder: (context, authed, _) {
        return MaterialApp(
          title: "约拍平台",
          debugShowCheckedModeBanner: false,
          theme: ThemeData(
            colorScheme: ColorScheme.fromSeed(seedColor: const Color(0xFF2B6CB0)),
            useMaterial3: true,
            scaffoldBackgroundColor: const Color(0xFFF7F8FA),
            appBarTheme: const AppBarTheme(centerTitle: true),
            inputDecorationTheme: const InputDecorationTheme(
              border: OutlineInputBorder(),
              filled: true,
              fillColor: Colors.white,
            ),
            cardTheme: const CardThemeData(color: Colors.white, elevation: 0),
          ),
          home: authed ? const HomePage() : const LoginPage(),
        );
      },
    );
  }
}
