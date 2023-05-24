import 'package:flutter/material.dart';
import 'package:mizer/api/contracts/sequencer.dart';
import 'package:mizer/widgets/dialog/action_dialog.dart';
import 'package:mizer/widgets/hoverable.dart';

const double MAX_DIALOG_WIDTH = 512;
const double MAX_DIALOG_HEIGHT = 512;
const double TILE_SIZE = 96;

class SelectSequenceDialog extends StatelessWidget {
  final SequencerApi api;

  const SelectSequenceDialog({required this.api, Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return ActionDialog(
      title: "Select Sequence",
      content: Container(
        width: MAX_DIALOG_WIDTH,
        height: MAX_DIALOG_HEIGHT,
        child: FutureBuilder(
            future: api.getSequences(),
            builder: (context, AsyncSnapshot<Sequences> data) {
              List<Sequence> sequences = data.hasData ? data.data!.sequences : [];
              sequences.sort((lhs, rhs) => lhs.id - rhs.id);

              return GridView.count(
                  crossAxisCount: (MAX_DIALOG_WIDTH / TILE_SIZE).floor(),
                  mainAxisSpacing: 4,
                  crossAxisSpacing: 4,
                  children: sequences
                      .map((s) => Tile(
                            title: s.id.toString(),
                            child: Center(child: Text(s.name)),
                            onClick: () => Navigator.of(context).pop(s),
                          ))
                      .toList());
            }),
      ),
      actions: [
        PopupAction("Cancel", () => Navigator.of(context).pop()),
        PopupAction("New Sequence", () => api.addSequence().then((s) => Navigator.of(context).pop(s)))
      ],
    );
  }
}

class Tile extends StatelessWidget {
  final String? title;
  final Widget child;
  final Function()? onClick;

  const Tile({this.title, required this.child, this.onClick, Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Hoverable(
        builder: (hovered) {
          return Container(
            clipBehavior: Clip.antiAlias,
            decoration: BoxDecoration(
                border: Border.all(color: Colors.grey.shade800, width: 2),
                borderRadius: BorderRadius.circular(4),
                color: hovered ? Colors.grey.shade700 : Colors.grey.shade900),
            width: TILE_SIZE,
            height: TILE_SIZE,
            child: IntrinsicWidth(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.stretch,
                children: [
                  if (title != null)
                    Container(
                        color: Colors.grey.shade800,
                        child: Text(title!, textAlign: TextAlign.center)),
                  Expanded(child: child),
                ],
              ),
            ),
          );
        },
        onTap: this.onClick);
  }
}