import 'package:flutter/material.dart';
import 'package:zcvote/store.dart';

class VotePage extends StatefulWidget {
  const VotePage({super.key});

  @override
  State<StatefulWidget> createState() => VotePageState();
}

class VotePageState extends State<VotePage> {
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
    return SizedBox.shrink();
  }
}
