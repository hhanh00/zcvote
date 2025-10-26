import 'package:riverpod/riverpod.dart';
import 'package:zcvote/src/rust/api/app.dart';
import 'package:zcvote/src/rust/api/data.dart';

final appProvider = Provider<App>((ref) => App(dbName: "zcvote.db"));
final listElectionsProvider = FutureProvider<List<Election>>((ref) async {
  final app = ref.watch(appProvider);
  final elections = await app.listElectionDefs();
  return elections;
});

class ElectionNotifier extends AsyncNotifier<Election> {
  final String name;
  ElectionNotifier(this.name);

  @override
  Future<Election> build() {
    final elections = ref.watch(listElectionsProvider.future);
    final election = elections.then((e) => e.firstWhere((e) => e.name == name));
    return election;
  }

  void save(int startHeight, int endHeight, List<Question> questions) async {
    final newState = await update(
      (prev) => prev.copyWith(
        startHeight: startHeight,
        endHeight: endHeight,
        questions: questions,
      ),
    );
    final app = ref.read(appProvider);
    app.saveElection(election: newState);
  }
}

final electionProvider =
    AsyncNotifierProvider.family<ElectionNotifier, Election, String>(
      ElectionNotifier.new,
    );
