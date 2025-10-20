import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:zcvote/pages/home.dart';

final navigatorKey = GlobalKey<NavigatorState>();
final RouteObserver<ModalRoute<void>> routeObserver =
    RouteObserver<ModalRoute<void>>();

var router = GoRouter(
  initialLocation: '/',
  observers: [routeObserver],
  navigatorKey: navigatorKey,
  routes: [GoRoute(path: '/', builder: (context, state) => HomePage())],
);
