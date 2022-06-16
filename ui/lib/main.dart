// @dart=2.11
import 'package:flutter/widgets.dart';
import 'package:mizer/app.dart';
import 'package:mizer/windows/dmx_monitor_window.dart';
import 'package:mizer/windows/main_window.dart';
import 'package:mizer/windows/midi_monitor_window.dart';
import 'package:mizer/windows/preferences_window.dart';
import 'package:mizer/windows/smart_window.dart';
import 'package:nativeshell/nativeshell.dart';

import 'i18n.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await MizerI18n.loadTranslations();
  runApp(MizerIntegratedUi());
}

class MizerIntegratedUi extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return MizerApp(child: WindowWidget(onCreateState: (initData) {
      WindowState state;

      state ??= DmxMonitorWindow.fromInitData(initData);
      state ??= MidiMonitorWindow.fromInitData(initData);
      state ??= PreferencesWindow.fromInitData(initData);
      state ??= SmartWindow.fromInitData(initData);
      state ??= MainWindowState();

      return state;
    }));
  }
}
