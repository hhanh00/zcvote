import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:zcvote/main.dart';
import 'package:zcvote/model.dart';

class VotePage extends StatefulWidget {
  const VotePage({super.key});

  @override
  State<StatefulWidget> createState() => VotePageState();
}

class VotePageState extends State<VotePage> {
  @override
  Widget build(BuildContext context) {
    return Center(
      child: Consumer(
        builder: (context, ref, _) => TextButton(
          onPressed: () async {
            final app = ref.read(appProvider);
            final election = await app.newElection(name: "TEST");
            logger.i(election.name);
          },
          child: Text("New"),
        ),
      ),
    );
  }
}
