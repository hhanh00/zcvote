import 'package:riverpod/riverpod.dart';
import 'package:zcvote/main.dart';
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

  void flush(Election election) {
    final app = ref.read(appProvider);
    app.storeElection(election: election);
  }

  void saveStartHeight(int startHeight) async {
    final newState = await update(
      (prev) => prev.copyWith(startHeight: startHeight),
    );
    flush(newState);
  }

  void saveEndHeight(int endHeight) async {
    final newState = await update(
      (prev) => prev.copyWith(endHeight: endHeight),
    );
    flush(newState);
  }

  void saveQuestions(List<Question> questions) async {
    final newState = await update(
      (prev) => prev.copyWith(questions: questions),
    );
    flush(newState);
  }

  void unlock() async {
    final newState = await update(
      (prev) => prev.copyWith(locked: false),
    );
    flush(newState);
  }

  Stream<String> finalize() {
    final app = ref.read(appProvider);
    return app.finalize(election: state.value!, lwd: LwdURL);

  }
}

final electionProvider =
    AsyncNotifierProvider.family<ElectionNotifier, Election, String>(
      ElectionNotifier.new,
    );
