import 'package:flutter/material.dart';
import 'package:mizer/api/contracts/programmer.dart';
import 'package:mizer/protos/fixtures.pb.dart';
import 'package:mizer/state/effects_bloc.dart';
import 'package:mizer/state/presets_bloc.dart';

import '../presets/presets_view.dart';

class ProgrammerControlList extends StatelessWidget {
  final ProgrammerState programmerState;
  final PresetsState presetsState;
  final EffectState effectsState;
  final List<FixtureControl> controls;

  const ProgrammerControlList(
      {required this.programmerState,
        required this.controls,
        required this.presetsState,
        required this.effectsState,
        Key? key})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Column(
        children: controls
            .where((control) => [
          FixtureControl.INTENSITY,
          FixtureControl.COLOR_MIXER,
          FixtureControl.COLOR_WHEEL,
          FixtureControl.PAN,
          FixtureControl.TILT,
        ].contains(control))
            .map((control) => PRESET_TYPES[control]!)
            .toSet()
            .map((presetType) => PresetGroup.build(
            presetType.toString(), presetsState.presets, effectsState, presetType))
            .toList());
  }
}
