import 'package:flutter/widgets.dart';
import 'package:mizer/api/contracts/programmer.dart';
import 'package:mizer/protos/fixtures.extensions.dart';
import 'package:mizer/protos/fixtures.pb.dart';

import 'fixture_group_control.dart';

const CONTROLS = [
  FixtureControl.ZOOM,
  FixtureControl.FOCUS,
  FixtureControl.PRISM,
  FixtureControl.FROST,
  FixtureControl.IRIS,
];

const NAMES = {
  FixtureControl.ZOOM: "Zoom",
  FixtureControl.FOCUS: "Focus",
  FixtureControl.PRISM: "Prism",
  FixtureControl.FROST: "Frost",
  FixtureControl.IRIS: "Iris",
};

class BeamSheet extends StatelessWidget {
  final List<FixtureInstance> fixtures;

  const BeamSheet({required this.fixtures, Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Container(
      child: controls.isNotEmpty
          ? ListView(
              scrollDirection: Axis.horizontal,
              children: controls
                  .map((control) => FixtureGroupControl(control))
                  .toList())
          : null,
    );
  }

  Iterable<Control> get controls {
    if (fixtures.isEmpty) {
      return [];
    }
    return fixtures.first.controls
        .where((e) => CONTROLS.contains(e.control))
        .map((control) => Control(NAMES[control.control],
            fader: control.fader,
            update: (v) => WriteControlRequest(
                  control: control.control,
                  fader: v,
                )));
  }
}