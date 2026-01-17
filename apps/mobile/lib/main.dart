import "package:flutter/material.dart";

import "app.dart";
import "services/session.dart";

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await SessionStore.init();
  runApp(const App());
}
