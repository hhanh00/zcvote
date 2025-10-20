import 'package:flutter/material.dart';
import 'package:zcvote/router.dart';
import 'package:zcvote/src/rust/frb_generated.dart';
import 'package:zcvote/store.dart';

Future<void> main() async {
  await RustLib.init();
  await AppStoreBase.init("test.db");
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp.router(
      routerConfig: router,
    );
  }
}
