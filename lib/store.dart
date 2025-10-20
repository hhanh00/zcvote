import 'package:mobx/mobx.dart';
import 'package:zcvote/src/rust/api/app.dart';

part 'store.g.dart';

AppStore get appStore => AppStoreBase.instance;

class AppStore = AppStoreBase with _$AppStore;

abstract class AppStoreBase with Store {
  late App app;

  static Future<void> init(String dbName) async {
    final app = await App.connect(dbName: dbName);
    appStore.app = app;
  }

  static AppStore instance = AppStore();
}
