import 'package:riverpod/riverpod.dart';
import 'package:zcvote/src/rust/api/app.dart';
import 'package:zcvote/src/rust/api/data.dart';

final appProvider = Provider<App>((ref) => App(dbName: "zcvote.db"));
final listElectionsProvider = FutureProvider<List<Election>>((ref) async {
  final app = ref.watch(appProvider);
  final elections = await app.listElectionDefs();
  return elections;
});
