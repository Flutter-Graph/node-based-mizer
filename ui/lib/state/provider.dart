import 'package:flutter/widgets.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mizer/state/nodes_view.dart';
import 'package:mizer/state/sequencer_bloc.dart';

import 'fixtures_bloc.dart';
import 'layouts_bloc.dart';
import 'media_bloc.dart';
import 'nodes_bloc.dart';
import 'presets_bloc.dart';
import 'session_bloc.dart';
import 'settings_bloc.dart';

class StateProvider extends StatelessWidget {
  final Widget child;

  StateProvider({ required this.child });

  @override
  Widget build(BuildContext context) {
    return MultiBlocProvider(
      child: NodesViewState(child: this.child),
      providers: [
        BlocProvider(create: (context) => NodesBloc(context.read())),
        BlocProvider(create: (context) => SessionBloc(context.read())),
        BlocProvider(create: (context) => FixturesBloc(context.read())),
        BlocProvider(create: (context) => MediaBloc(context.read())),
        BlocProvider(create: (context) => LayoutsBloc(context.read())),
        BlocProvider(create: (context) => SettingsBloc(context.read())),
        BlocProvider(create: (context) => SequencerBloc(context.read())),
        BlocProvider(create: (context) => PresetsBloc(context.read())),
      ],
    );
  }
}
