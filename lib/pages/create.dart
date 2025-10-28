import 'package:flutter/material.dart';
import 'package:flutter_form_builder/flutter_form_builder.dart';
import 'package:form_builder_validators/form_builder_validators.dart';
import 'package:gap/gap.dart';
import 'package:go_router/go_router.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:zcvote/main.dart';
import 'package:zcvote/model.dart';
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
        return Scaffold(
          appBar: AppBar(
            title: Text("Create Elections"),
            actions: [IconButton(onPressed: onNew, icon: Icon(Icons.note_add))],
          ),
          body: ListView(
          children: [
            for (var v in value)
              ListTile(
                title: Text(v.name),
                onTap: () => context.push("/create/edit", extra: v.name),
              ),
          ],
          ),
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
      await app.newElection(name: nameController.text);
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
    final electionNotifier = ref.watch(electionProvider(widget.name).notifier);

    return election.when(
      data: (data) {
        final locked = data.locked;

        return Scaffold(
          appBar: AppBar(
            actions: [
              locked ? IconButton(
                onPressed: onUnlock,
                icon: Icon(Icons.lock_open),
              ) : IconButton(
                onPressed: onFinalize,
                icon: Icon(Icons.flag),
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
                      decoration: InputDecoration(label: Text("Start Height")),
                      initialValue: data.startHeight.toString(),
                      readOnly: locked,
                      onChanged: (v) {
                        if (locked || v == null || v.isEmpty) return;
                        electionNotifier.saveStartHeight(int.parse(v));
                      },
                      validator: FormBuilderValidators.integer(
                        checkNullOrEmpty: true,
                      ),
                    ),
                    FormBuilderTextField(
                      name: "endHeight",
                      decoration: InputDecoration(label: Text("End Height")),
                      initialValue: data.endHeight.toString(),
                      readOnly: locked,
                      onChanged: (v) {
                        if (locked || v == null || v.isEmpty) return;
                        electionNotifier.saveEndHeight(int.parse(v));
                      },
                      validator: FormBuilderValidators.integer(
                        checkNullOrEmpty: true,
                      ),
                    ),
                    Gap(8),
                    QuestionListFormField(
                      name: "questions",
                      initialValue: data.questions,
                      readOnly: locked,
                      onChanged: (q) {
                        if (locked || q == null) return;
                        electionNotifier.saveQuestions(q);
                      },
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

  void onUnlock() {
    // TODO: Add confirmation
    final electionNotifier = ref.read(electionProvider(widget.name).notifier);
    electionNotifier.unlock();
  }

  void onFinalize() async {
    // TODO: Add confirmation
    final electionNotifier = ref.read(electionProvider(widget.name).notifier);
    final progress = electionNotifier.finalize();
    progress.listen((m) => logger.i(m),
    onDone: () {
      ref.invalidate(listElectionsProvider);
    });
  }
}

class QuestionListFormField extends StatefulWidget {
  final String name;
  final List<Question>? initialValue;
  final bool readOnly;
  final void Function(List<Question>?)? onChanged;

  const QuestionListFormField({
    required this.name,
    this.initialValue,
    this.readOnly = false,
    this.onChanged,
    super.key,
  });

  @override
  State<StatefulWidget> createState() => QuestionListFormFieldState();
}

class QuestionListFormFieldState extends State<QuestionListFormField> {
  late List<Question> questions = List.of(widget.initialValue ?? []);
  int? selected;
  final formKey = GlobalKey<FormBuilderState>();

  @override
  Widget build(BuildContext context) {
    return FormBuilderField<List<Question>>(
      name: widget.name,
      initialValue: questions,
      onChanged: widget.onChanged,
      builder: (state) {
        return FormBuilder(
          key: formKey,
          child: Column(
            children: [
              SizedBox(
                height: 200,
                child: Row(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Expanded(
                      child: ListView(
                        shrinkWrap: true,
                        children: [
                          for (var (i, q) in questions.indexed)
                            ListTile(
                              title: Text(q.question),
                              onTap: () => setState(() {
                                if (selected == i) {
                                  selected = null;
                                } else {
                                  selected = i;
                                  final f = formKey.currentState!.fields;
                                  final q = questions[i];
                                  f["question"]?.didChange(q.question);
                                  f["answers"]?.didChange(
                                    q.choices.map((a) => a.choice).join("\n"),
                                  );
                                }
                              }),
                            ),
                        ],
                      ),
                    ),
                    if (!widget.readOnly) SizedBox(
                      width: 40,
                      child: Column(
                        children: [
                          IconButton(
                            icon: Icon(Icons.add),
                            onPressed: selected == null ? onAdd : null,
                          ),
                          Gap(8),
                          IconButton(
                            icon: Icon(Icons.arrow_upward),
                            onPressed: selected != null ? onUp : null,
                          ),
                          IconButton(
                            icon: Icon(Icons.arrow_downward),
                            onPressed: selected != null ? onDown : null,
                          ),
                          Divider(),
                          IconButton(
                            icon: Icon(Icons.delete),
                            onPressed: selected != null ? onDelete : null,
                          ),
                        ],
                      ),
                    ),
                  ],
                ),
              ),
              Gap(16),
              if (selected != null)
                FormBuilderTextField(
                  name: "question",
                  decoration: InputDecoration(label: Text("Question")),
                  initialValue: questions[selected!].question,
                  readOnly: widget.readOnly,
                  onChanged: (v) {
                    setState(() {
                      final q = questions[selected!];
                      questions[selected!] = q.copyWith(question: v!);
                      widget.onChanged?.call(questions);
                    });
                  },
                ),
              if (selected != null)
                FormBuilderTextField(
                  name: "answers",
                  decoration: InputDecoration(
                    label: Text("Answers (one per line)"),
                  ),
                  minLines: 10,
                  maxLines: 30,
                  initialValue: questions[selected!].choices
                      .map((a) => a.choice)
                      .join("\n"),
                  readOnly: widget.readOnly,
                  onChanged: (v) {
                    setState(() {
                      final q = questions[selected!];
                      questions[selected!] = q.copyWith(
                        choices: v!
                            .split("\n")
                            .map((a) => CandidateChoice(choice: a))
                            .toList(),
                      );
                      widget.onChanged?.call(questions);
                    });
                  },
                ),
            ],
          ),
        );
      },
    );
  }

  void onAdd() async {
    setState(
      () => questions.add(Question(question: "New Question", choices: [])),
    );
  }

  // TODO
  void onUp() async {}
  void onDown() async {}
  void onDelete() async {}
}

// TODOs
// Show progress windows
// Add confirmations
// Add decoration labels to forms
// Save election files
// Fix other todos
