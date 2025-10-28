import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:logger/logger.dart';
import 'package:zcvote/router.dart';
import 'package:zcvote/src/rust/frb_generated.dart';

Logger logger = Logger();

const LwdURL = "https://zec.rocks"; // TODO: Make this configurable

Future<void> main() async {
  await RustLib.init();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return ProviderScope(child: MaterialApp.router(
      routerConfig: router,
      themeMode: ThemeMode.system,
      theme: ThemeData.light(),
      darkTheme: ThemeData.dark(),
      debugShowCheckedModeBanner: false,
    ));
  }
}
