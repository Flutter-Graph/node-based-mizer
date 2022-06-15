import 'dart:developer';

import 'package:flutter/services.dart';
import 'package:mizer/api/contracts/programmer.dart';
import 'package:mizer/api/plugin/ffi/programmer.dart';
import 'package:mizer/protos/fixtures.pb.dart';

import 'ffi/api.dart';
import 'ffi/bindings.dart';
import 'ffi/programmer.dart';

export 'ffi/programmer.dart' show ProgrammerStatePointer;

class ProgrammerPluginApi implements ProgrammerApi {
  final FFIBindings bindings;
  final MethodChannel channel = const MethodChannel("mizer.live/programmer");
  final EventChannel stateEvents = const EventChannel("mizer.live/programmer/watch");

  ProgrammerPluginApi(this.bindings);

  @override
  Future<void> writeControl(WriteControlRequest request) async {
    await channel.invokeMethod("writeControl", request.writeToBuffer());
  }

  @override
  Future<void> selectFixtures(List<FixtureId> fixtureIds) async {
    await channel.invokeMethod("selectFixtures", SelectFixturesRequest(fixtures: fixtureIds).writeToBuffer());
  }

  @override
  Future<void> clear() async {
    await channel.invokeMethod("clear");
  }

  @override
  Future<void> highlight(bool highlight) async {
    await channel.invokeMethod("highlight", highlight);
  }

  @override
  Future<void> store(int sequenceId, StoreRequest_Mode storeMode) async {
    var request = StoreRequest(sequenceId: sequenceId, storeMode: storeMode);
    await channel.invokeMethod("store", request.writeToBuffer());
  }

  @override
  Stream<ProgrammerState> observe() {
    return stateEvents.receiveBroadcastStream().map((buffer) {
      log("$buffer");
      return ProgrammerState.fromBuffer(_convertBuffer(buffer));
    }).map((state) {
      log("$state");
      return state;
    });
  }

  @override
  Future<Presets> getPresets() async {
    var response = await channel.invokeMethod("getPresets");

    return Presets.fromBuffer(_convertBuffer(response));
  }

  @override
  Future<void> callPreset(PresetId id) async {
    await channel.invokeMethod("callPreset", id.writeToBuffer());
  }

  @override
  Future<Groups> getGroups() async {
    var response = await channel.invokeMethod("getGroups");

    return Groups.fromBuffer(_convertBuffer(response));
  }

  @override
  Future<void> selectGroup(int id) async {
    await channel.invokeMethod("selectGroup", id);
  }

  Future<ProgrammerStatePointer?> getProgrammerPointer() async {
    int pointer = await channel.invokeMethod("getProgrammerPointer");

    return this.bindings.openProgrammer(pointer);
  }

  @override
  Future<Group> addGroup(String name) async {
    var response = await channel.invokeMethod("addGroup", name);

    return Group.fromBuffer(_convertBuffer(response));
  }

  @override
  Future<void> assignFixturesToGroup(List<int> fixtures, Group group) async {
    var request = AssignFixturesToGroupRequest(
        id: group.id, fixtures: fixtures.map((id) => FixtureId(fixture: id)).toList());
    await channel.invokeMethod("assignFixturesToGroup", request.writeToBuffer());
  }

  @override
  Future<void> assignFixtureSelectionToGroup(Group group) async {
    await channel.invokeMethod("assignFixtureSelectionToGroup", group.id);
  }

  @override
  Future<void> callEffect(int id) async {
    await channel.invokeMethod("callEffect", id);
  }

  static List<int> _convertBuffer(List<Object> response) {
    return response.map((dynamic e) => e as int).toList();
  }
}
