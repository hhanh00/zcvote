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
              builder: (context, state) => PlaceHolderPage("vote"),
            ),
          ],
        ),
        StatefulShellBranch(
          observers: [routeObserver],
          routes: [
            GoRoute(
              path: '/create',
              builder: (context, state) => CreatePage(key: createPageKey),
              routes: [
                GoRoute(
                  path: "edit",
                  builder: (context, state) =>
                      CreateEditPage(state.extra as String),
                ),
              ],
            ),
          ],
        ),
        StatefulShellBranch(
          routes: [
            GoRoute(
              path: '/deploy',
              builder: (context, state) => PlaceHolderPage("deploy"),
            ),
          ],
        ),
        StatefulShellBranch(
          routes: [
            GoRoute(
              path: '/verify',
              builder: (context, state) => PlaceHolderPage("verify"),
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
    BottomTabItem(label: 'Create', icon: Icons.build, route: '/create'),
    BottomTabItem(label: 'Deploy', icon: Icons.hub, route: '/deploy'),
    BottomTabItem(label: 'Verify', icon: Icons.verified, route: '/verify'),
  ];

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final currentIndex = shell.currentIndex;

    return Scaffold(
      body: shell,
      bottomNavigationBar: NavigationBar(
        selectedIndex: currentIndex,
        onDestinationSelected: (index) {
          shell.goBranch(index);
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

  const BottomTabItem({
    required this.label,
    required this.icon,
    required this.route,
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
