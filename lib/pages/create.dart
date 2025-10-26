import 'package:flutter/material.dart';
import 'package:flutter_form_builder/flutter_form_builder.dart';
import 'package:form_builder_validators/form_builder_validators.dart';
import 'package:go_router/go_router.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:zcvote/main.dart';
import 'package:zcvote/model.dart';
import 'package:zcvote/router.dart';
import 'package:zcvote/src/rust/api/data.dart';

class CreatePage extends ConsumerStatefulWidget {
  const CreatePage({super.key});

  @override
  ConsumerState<ConsumerStatefulWidget> createState() => CreatePageState();
}

class CreatePageState extends ConsumerState<CreatePage> {
  @override
  Widget build(BuildContext context) {
    final elections = ref.watch(listElectionsProvider);
    switch (elections) {
      case AsyncValue(:final value?):
        return ListView(
          children: [
            for (var v in value)
              ListTile(
                title: Text(v.name),
                onTap: () => context.push("/create/edit", extra: v.name),
              ),
          ],
        );
      case AsyncValue(error: != null):
        return Text("Error: ${elections.error}");
      case AsyncValue():
        return LinearProgressIndicator();
    }
  }

  void onNew() async {
    final nameController = TextEditingController();
    final confirmed =
        await showDialog<bool>(
          context: context,
          barrierDismissible: false,
          builder: (context) => AlertDialog(
            title: Text("Create a new Election"),
            content: SingleChildScrollView(
              child: FormBuilder(
                child: FormBuilderTextField(
                  name: "name",
                  decoration: InputDecoration(label: Text("Name")),
                  controller: nameController,
                  validator: FormBuilderValidators.required(),
                ),
              ),
            ),
            actions: [
              TextButton(
                onPressed: () => context.pop(false),
                child: Text("Cancel"),
              ),
              TextButton(onPressed: () => context.pop(true), child: Text("OK")),
            ],
          ),
        ) ??
        false;
    if (confirmed) {
      final app = ref.read(appProvider);
      final election = await app.newElection(name: nameController.text);
      logger.i(election.name);
      ref.invalidate(listElectionsProvider);
    }
  }
}

class CreateEditPage extends ConsumerStatefulWidget {
  final String name;
  const CreateEditPage(this.name, {super.key});

  @override
  ConsumerState<ConsumerStatefulWidget> createState() => CreateEditState();
}

class CreateEditState extends ConsumerState<CreateEditPage> with RouteAware {
  final startHeightController = TextEditingController();
  final endHeightController = TextEditingController();
  final questions = <Question>[];

  @override
  void didChangeDependencies() {
    super.didChangeDependencies();
    final route = ModalRoute.of(context);
    if (route is PageRoute) {
      routeObserver.subscribe(this, route);
    }
  }

  @override
  void dispose() {
    routeObserver.unsubscribe(this);
    super.dispose();
  }

  @override
  void didPop() {
    super.didPop();
    final electionNotifier = ref.read(electionProvider(widget.name).notifier);
    electionNotifier.build();
    electionNotifier.save(
      int.parse(startHeightController.text),
      int.parse(endHeightController.text),
      questions,
    );
  }

  @override
  Widget build(BuildContext context) {
    final election = ref.watch(electionProvider(widget.name));
    return election.when(
      data: (data) {
        startHeightController.text = data.startHeight.toString();
        endHeightController.text = data.endHeight.toString();

        return Scaffold(
          appBar: AppBar(
            actions: [
              IconButton(
                onPressed: () {
                  createPageKey.currentState?.onNew();
                },
                icon: Icon(Icons.add),
              ),
            ],
          ),
          body: SingleChildScrollView(
            child: Padding(
              padding: EdgeInsetsGeometry.symmetric(horizontal: 16),
              child: FormBuilder(
                child: Column(
                  children: [
                    FormBuilderTextField(
                      name: "name",
                      readOnly: true,
                      initialValue: data.name,
                    ),
                    FormBuilderTextField(
                      name: "startHeight",
                      controller: startHeightController,
                      validator: FormBuilderValidators.integer(
                        checkNullOrEmpty: true,
                      ),
                    ),
                    FormBuilderTextField(
                      name: "endHeight",
                      controller: endHeightController,
                      validator: FormBuilderValidators.integer(
                        checkNullOrEmpty: true,
                      ),
                    ),
                    TextButton(
                      onPressed: () => context.pop(),
                      child: Text("Go Back"),
                    ),
                  ],
                ),
              ),
            ),
          ),
        );
      },
      error: (error, _) => Text("$error"),
      loading: LinearProgressIndicator.new,
    );
  }
}
