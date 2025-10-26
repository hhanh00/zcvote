import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:zcvote/pages/create.dart';

final navigatorKey = GlobalKey<NavigatorState>();
final RouteObserver<ModalRoute<void>> routeObserver =
    RouteObserver<ModalRoute<void>>();

final createPageKey = GlobalKey<CreatePageState>();

var router = GoRouter(
  initialLocation: '/vote',
  observers: [routeObserver],
  navigatorKey: navigatorKey,
  routes: [
    StatefulShellRoute.indexedStack(
      builder: (context, state, shell) {
        return ScaffoldWithNavBar(shell: shell);
      },
      branches: [
        StatefulShellBranch(
          routes: [
            GoRoute(
              path: '/vote',
              pageBuilder: (context, state) =>
                  NoTransitionPage(child: PlaceHolderPage("vote")),
            ),
          ],
        ),
        StatefulShellBranch(
          routes: [
            GoRoute(
              path: '/create',
              pageBuilder: (context, state) =>
                  NoTransitionPage(child: CreatePage(key: createPageKey)),
              routes: [
                GoRoute(
                  path: "edit",
                  pageBuilder: (context, state) =>
                      NoTransitionPage(child: CreateEditPage(state.extra as String)),
                ),
              ],
            ),
          ],
        ),
        StatefulShellBranch(
          routes: [
            GoRoute(
              path: '/deploy',
              pageBuilder: (context, state) =>
                  const NoTransitionPage(child: PlaceHolderPage("deploy")),
            ),
          ],
        ),
        StatefulShellBranch(
          routes: [
            GoRoute(
              path: '/verify',
              pageBuilder: (context, state) =>
                  const NoTransitionPage(child: PlaceHolderPage("verify")),
            ),
          ],
        ),
      ],
    ),
  ],
);

class ScaffoldWithNavBar extends ConsumerWidget {
  final StatefulNavigationShell shell;

  const ScaffoldWithNavBar({super.key, required this.shell});

  static final tabs = [
    BottomTabItem(label: 'Vote', icon: Icons.how_to_vote, route: '/vote'),
    BottomTabItem(
      label: 'Create',
      icon: Icons.build,
      route: '/create',
      actions: [
        IconButton(
          onPressed: () {
            createPageKey.currentState?.onNew();
          },
          icon: Icon(Icons.add),
        ),
      ],
    ),
    BottomTabItem(label: 'Deploy', icon: Icons.hub, route: '/deploy'),
    BottomTabItem(label: 'Verify', icon: Icons.verified, route: '/verify'),
  ];

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final currentIndex = shell.currentIndex;
    final current = tabs[currentIndex];
    final actions = current.actions ?? [];

    return Scaffold(
      appBar: AppBar(title: Text(current.label), actions: actions),
      body: shell,
      bottomNavigationBar: NavigationBar(
        selectedIndex: currentIndex,
        onDestinationSelected: (index) {
          context.go(tabs[index].route);
        },
        destinations: [
          for (final t in tabs)
            NavigationDestination(icon: Icon(t.icon), label: t.label),
        ],
      ),
    );
  }
}

class BottomTabItem {
  final String label;
  final IconData icon;
  final String route;
  final List<Widget>? actions;

  const BottomTabItem({
    required this.label,
    required this.icon,
    required this.route,
    this.actions,
  });
}

class PlaceHolderPage extends StatelessWidget {
  final String label;
  const PlaceHolderPage(this.label, {super.key});

  @override
  Widget build(BuildContext context) {
    final t = Theme.of(context).textTheme;
    return Center(child: Text(label, style: t.headlineSmall));
  }
}
