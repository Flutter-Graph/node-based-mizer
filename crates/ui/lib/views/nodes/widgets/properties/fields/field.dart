import 'package:flutter/material.dart';

class Field extends StatelessWidget {
  final String label;
  final Widget child;
  final Widget? suffix;

  Field({required this.label, required this.child, this.suffix});

  @override
  Widget build(BuildContext context) {
    var textTheme = Theme.of(context).textTheme;

    return Row(
      mainAxisAlignment: MainAxisAlignment.start,
      children: [
        Flexible(flex: 1, fit: FlexFit.tight, child: Text(this.label, textAlign: TextAlign.start, style: textTheme.bodySmall)),
        Container(width: 8),
        Flexible(flex: 2, fit: FlexFit.tight, child: child),
        if (suffix != null) suffix!
      ],
    );
  }
}
