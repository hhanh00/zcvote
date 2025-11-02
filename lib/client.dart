import 'dart:async';
import 'dart:io';

import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:zcvote/main.dart';
import 'package:zcvote/model.dart';

Future<void> submitVote(WidgetRef ref) async {
  final app = ref.read(appProvider);
  final election = await app.downloadElection(url: "http://mini:8000", id: "f13ad102bc23215a21ef659bc2d22a0ff786a78a5db9de591a9bd57ab815b035");
  final progress = election.downloadBlocks(app: app, url: LwdURL);

  final c = Completer();
  progress.listen((p) {
    print(p);
  }, onDone: c.complete);
  await c.future;

  final seed = Platform.environment["TEST_SEED"]!;
  await app.scan(seed: seed, start: election.start(), end: election.end());
  print("Scan completed");
}
