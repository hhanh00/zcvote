import 'package:flutter/material.dart';
import 'package:flutter_form_builder/flutter_form_builder.dart';
import 'package:form_builder_validators/form_builder_validators.dart';
import 'package:go_router/go_router.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:zcvote/main.dart';
import 'package:zcvote/model.dart';

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
                onTap: () => context.go("/create/edit", extra: v.name),
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

class CreateEditState extends ConsumerState<CreateEditPage> {
  @override
  Widget build(BuildContext context) {
    final election = ref.watch(electionProvider(widget.name));
    return election.when(
      data: (data) {
        return Text(data.name);
      },
      error: (error, _) => Text("$error"),
      loading: LinearProgressIndicator.new,
    );
  }
}
