import 'package:flutter/material.dart';
import 'package:zcvote/store.dart';

class HomePage extends StatefulWidget {
  const HomePage({super.key});

  @override
  State<StatefulWidget> createState() => HomePageState();
}

class HomePageState extends State<HomePage> {
  int x = 0;

  @override void initState() {
    super.initState();
    Future(() async {
      final r = await appStore.app.test();
      setState(() => x = r);
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(appBar: AppBar(title: Text('Home $x')),
    );
  }
}
