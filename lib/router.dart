import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:zcvote/pages/home.dart';
import 'package:zcvote/src/rust/api/app.dart';

final navigatorKey = GlobalKey<NavigatorState>();
final RouteObserver<ModalRoute<void>> routeObserver =
    RouteObserver<ModalRoute<void>>();

var router = GoRouter(
  initialLocation: '/vote',
  observers: [routeObserver],
  navigatorKey: navigatorKey,
  routes: [
    ShellRoute(
      builder: (context, state, child) {
        return ScaffoldWithNavBar(child: child);
      },
      routes: [
        GoRoute(
          path: '/vote',
          pageBuilder: (context, state) =>
              const NoTransitionPage(child: VotePage()),
        ),
        GoRoute(
          path: '/create',
          pageBuilder: (context, state) =>
              const NoTransitionPage(child: PlaceHolderPage("create")),
        ),
        GoRoute(
          path: '/deploy',
          pageBuilder: (context, state) =>
              const NoTransitionPage(child: PlaceHolderPage("deploy")),
        ),
        GoRoute(
          path: '/verify',
          pageBuilder: (context, state) =>
              const NoTransitionPage(child: PlaceHolderPage("verify")),
        ),
      ],
    ),
  ],
);

class ScaffoldWithNavBar extends StatelessWidget {
  final Widget child;

  const ScaffoldWithNavBar({super.key, required this.child});

  static final tabs = [
    BottomTabItem(label: 'Vote', icon: Icons.how_to_vote, route: '/vote'),
    BottomTabItem(label: 'Create', icon: Icons.build, route: '/create'),
    BottomTabItem(label: 'Deploy', icon: Icons.hub, route: '/deploy'),
    BottomTabItem(label: 'Verify', icon: Icons.verified, route: '/verify'),
  ];

  @override
  Widget build(BuildContext context) {
    final currentLocation = GoRouterState.of(context).uri.toString();
    int currentIndex = tabs.indexWhere(
      (t) => currentLocation.startsWith(t.route),
    );
    if (currentIndex == -1) currentIndex = 0;
    final current = tabs[currentIndex];

    return Scaffold(
      appBar: AppBar(title: Text(current.label)),
      body: child,
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
