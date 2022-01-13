import 'package:flutter/material.dart';

import 'decoration.dart';

class ButtonInput extends StatefulWidget {
  final Function(double) onValue;
  final String? label;
  final Color? color;

  ButtonInput({ this.label, required this.onValue, this.color });

  @override
  _ButtonInputState createState() => _ButtonInputState();
}

class _ButtonInputState extends State<ButtonInput> {
  bool pressed = false;

  @override
  Widget build(BuildContext context) {
    return Container(
        decoration: ControlDecoration(color: widget.color),
        padding: const EdgeInsets.all(4),
        child: MouseRegion(
          cursor: SystemMouseCursors.click,
          child: GestureDetector(
              onTapDown: (_) {
                setState(() => this.pressed = true);
                this.widget.onValue(1);
              },
              onTapUp: (_) {
                setState(() => this.pressed = false);
                this.widget.onValue(0);
              },
              child: Container(
                child: widget.label == null ? null : Center(child: Text(widget.label!, textAlign: TextAlign.center, style: Theme.of(context).textTheme.bodySmall)),
                decoration: ShapeDecoration(
                  color: this.pressed ? Colors.black45 : Colors.transparent,
                  shape: RoundedRectangleBorder(
                    borderRadius: BorderRadius.all(Radius.circular(4)),
                  ),
                ),
              )),
        ));
  }
}
