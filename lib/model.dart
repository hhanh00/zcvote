import 'package:riverpod/riverpod.dart';
import 'package:zcvote/src/rust/api/app.dart';

final appProvider = Provider<App>((ref) => App(dbName: "zcvote.db"));
