import 'package:collection/collection.dart';
import 'package:flutter/material.dart';
import 'package:flutter/scheduler.dart';
import 'package:flutter/services.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mizer/api/contracts/programmer.dart';
import 'package:mizer/api/plugin/ffi/plans.dart';
import 'package:mizer/extensions/fixture_id_extensions.dart';
import 'package:mizer/protos/fixtures.pb.dart';
import 'package:mizer/protos/plans.pb.dart';

const double fieldSize = 24;

class PlansFixturesLayer extends StatelessWidget {
  final Plan plan;
  final ProgrammerState? programmerState;
  final bool setupMode;
  final FixturesRefPointer fixturesPointer;

  const PlansFixturesLayer(
      {required this.plan,
      required this.programmerState,
      required this.setupMode,
      required this.fixturesPointer,
      Key? key})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    return CustomMultiChildLayout(
        delegate: PlanFixturesLayoutDelegate(plan), children: _getFixtures(context));
  }

  List<Widget> _getFixtures(BuildContext context) {
    return plan.positions.map((p) {
      var selected =
          programmerState?.activeFixtures.firstWhereOrNull((f) => f.overlaps(p.id)) != null;
      var tracked = programmerState?.fixtures.firstWhereOrNull((f) => f.overlaps(p.id)) != null;
      var child = Fixture2DView(
          fixture: p,
          ref: fixturesPointer,
          tracked: tracked,
          selected: selected,
          onSelect: () => this._addFixtureToSelection(context, p.id),
          onUnselect: () => this._removeFixtureFromSelection(context, p.id));
      return LayoutId(
          id: p.id,
          child: setupMode
              // TODO: hit detection seems off here, especially for larger views
              ? Draggable(
                  hitTestBehavior: HitTestBehavior.translucent,
                  data: p,
                  feedback: _getDragFeedback(context, p),
                  child: MouseRegion(child: child, cursor: SystemMouseCursors.move),
                )
              : MouseRegion(child: child, cursor: SystemMouseCursors.click));
    }).toList();
  }

  Widget _getDragFeedback(BuildContext context, FixturePosition position) {
    var textStyle = Theme.of(context).textTheme.bodyMedium;
    if (programmerState?.fixtures.isEmpty ?? true) {
      return Fixture2DView(
          fixture: position, ref: fixturesPointer!, selected: true, textStyle: textStyle);
    }

    var selectedFixtures =
        plan.positions.where((p) => programmerState?.fixtures.contains(p.id) ?? false);

    return Row(
      mainAxisAlignment: MainAxisAlignment.start,
      children: selectedFixtures
          .map((p) => Fixture2DView(
              fixture: p, ref: fixturesPointer!, selected: true, textStyle: textStyle))
          .toList(),
    );
  }

  void _addFixtureToSelection(BuildContext context, FixtureId fixtureId) {
    ProgrammerApi programmerApi = context.read();
    programmerApi.selectFixtures([fixtureId]);
  }

  void _removeFixtureFromSelection(BuildContext context, FixtureId fixtureId) {
    ProgrammerApi programmerApi = context.read();
    programmerApi.unselectFixtures([fixtureId]);
  }
}

class Fixture2DView extends StatefulWidget {
  final FixturePosition fixture;
  final FixturesRefPointer ref;
  final bool tracked;
  final bool selected;
  final TextStyle? textStyle;
  final Function()? onSelect;
  final Function()? onUnselect;

  const Fixture2DView(
      {required this.fixture,
      required this.ref,
      this.onSelect,
      this.onUnselect,
      this.tracked = false,
      this.selected = false,
      this.textStyle,
      Key? key})
      : super(key: key);

  @override
  State<Fixture2DView> createState() => _Fixture2DViewState();
}

class _Fixture2DViewState extends State<Fixture2DView> with SingleTickerProviderStateMixin {
  Ticker? ticker;
  FixtureState state = FixtureState();

  @override
  void initState() {
    super.initState();
    ticker = this.createTicker((elapsed) {
      setState(() {
        state = widget.ref.readState(widget.fixture.id);
      });
    });
    ticker!.start();
  }

  @override
  void dispose() {
    ticker?.stop(canceled: true);
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    double fontSize = 6;
    var color = widget.tracked ? Colors.deepOrange : null;
    var textStyle = (widget.textStyle ?? TextStyle()).copyWith(fontSize: fontSize, color: color);
    return GestureDetector(
      onTap: () {
        if (RawKeyboard.instance.keysPressed.any((key) => [
              LogicalKeyboardKey.shift,
              LogicalKeyboardKey.shiftLeft,
              LogicalKeyboardKey.shiftRight,
            ].contains(key))) {
          if (widget.onUnselect != null) {
            widget.onUnselect!();
          }
        } else {
          if (widget.onSelect != null) {
            widget.onSelect!();
          }
        }
      },
      child: Container(
          width: fieldSize,
          height: fieldSize,
          padding: const EdgeInsets.all(2),
          child: Container(
              decoration: ShapeDecoration(
                shape: RoundedRectangleBorder(
                  borderRadius: BorderRadius.circular(2),
                  side: BorderSide(
                    color: widget.selected ? Colors.grey.shade500 : Colors.grey.shade800,
                    width: 2,
                    style: BorderStyle.solid,
                  ),
                ),
                color: state.getColor(),
              ),
              child: Align(
                  alignment: Alignment.topLeft,
                  child: Text(widget.fixture.id.toDisplay(), style: textStyle)))),
    );
  }
}

class PlanFixturesLayoutDelegate extends MultiChildLayoutDelegate {
  final Plan plan;

  PlanFixturesLayoutDelegate(this.plan);

  @override
  void performLayout(Size size) {
    for (var fixture in plan.positions) {
      var size = Size.square(fieldSize);
      layoutChild(fixture.id, BoxConstraints.tight(size));
      var offset = _convertToScreenPosition(fixture);
      positionChild(fixture.id, offset);
    }
  }

  @override
  bool shouldRelayout(covariant PlanFixturesLayoutDelegate oldDelegate) {
    return plan.positions != oldDelegate.plan.positions;
  }
}

Offset _convertToScreenPosition(FixturePosition fixture) {
  return Offset(fixture.x.toDouble(), fixture.y.toDouble()) * fieldSize;
}
