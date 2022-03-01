///
//  Generated code. Do not modify.
//  source: settings.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

class RequestSettings extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'RequestSettings', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'mizer.settings'), createEmptyInstance: create)
    ..hasRequiredFields = false
  ;

  RequestSettings._() : super();
  factory RequestSettings() => create();
  factory RequestSettings.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory RequestSettings.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  RequestSettings clone() => RequestSettings()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  RequestSettings copyWith(void Function(RequestSettings) updates) => super.copyWith((message) => updates(message as RequestSettings)) as RequestSettings; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static RequestSettings create() => RequestSettings._();
  RequestSettings createEmptyInstance() => create();
  static $pb.PbList<RequestSettings> createRepeated() => $pb.PbList<RequestSettings>();
  @$core.pragma('dart2js:noInline')
  static RequestSettings getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<RequestSettings>(create);
  static RequestSettings? _defaultInstance;
}

class Settings extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'Settings', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'mizer.settings'), createEmptyInstance: create)
    ..aOM<Hotkeys>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'hotkeys', subBuilder: Hotkeys.create)
    ..hasRequiredFields = false
  ;

  Settings._() : super();
  factory Settings({
    Hotkeys? hotkeys,
  }) {
    final _result = create();
    if (hotkeys != null) {
      _result.hotkeys = hotkeys;
    }
    return _result;
  }
  factory Settings.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Settings.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Settings clone() => Settings()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Settings copyWith(void Function(Settings) updates) => super.copyWith((message) => updates(message as Settings)) as Settings; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Settings create() => Settings._();
  Settings createEmptyInstance() => create();
  static $pb.PbList<Settings> createRepeated() => $pb.PbList<Settings>();
  @$core.pragma('dart2js:noInline')
  static Settings getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Settings>(create);
  static Settings? _defaultInstance;

  @$pb.TagNumber(1)
  Hotkeys get hotkeys => $_getN(0);
  @$pb.TagNumber(1)
  set hotkeys(Hotkeys v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasHotkeys() => $_has(0);
  @$pb.TagNumber(1)
  void clearHotkeys() => clearField(1);
  @$pb.TagNumber(1)
  Hotkeys ensureHotkeys() => $_ensure(0);
}

class Hotkeys extends $pb.GeneratedMessage {
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'Hotkeys', package: const $pb.PackageName(const $core.bool.fromEnvironment('protobuf.omit_message_names') ? '' : 'mizer.settings'), createEmptyInstance: create)
    ..m<$core.String, $core.String>(1, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'global', entryClassName: 'Hotkeys.GlobalEntry', keyFieldType: $pb.PbFieldType.OS, valueFieldType: $pb.PbFieldType.OS, packageName: const $pb.PackageName('mizer.settings'))
    ..m<$core.String, $core.String>(2, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'layouts', entryClassName: 'Hotkeys.LayoutsEntry', keyFieldType: $pb.PbFieldType.OS, valueFieldType: $pb.PbFieldType.OS, packageName: const $pb.PackageName('mizer.settings'))
    ..m<$core.String, $core.String>(3, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'programmer', entryClassName: 'Hotkeys.ProgrammerEntry', keyFieldType: $pb.PbFieldType.OS, valueFieldType: $pb.PbFieldType.OS, packageName: const $pb.PackageName('mizer.settings'))
    ..m<$core.String, $core.String>(4, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'nodes', entryClassName: 'Hotkeys.NodesEntry', keyFieldType: $pb.PbFieldType.OS, valueFieldType: $pb.PbFieldType.OS, packageName: const $pb.PackageName('mizer.settings'))
    ..m<$core.String, $core.String>(5, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'patch', entryClassName: 'Hotkeys.PatchEntry', keyFieldType: $pb.PbFieldType.OS, valueFieldType: $pb.PbFieldType.OS, packageName: const $pb.PackageName('mizer.settings'))
    ..m<$core.String, $core.String>(6, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'sequencer', entryClassName: 'Hotkeys.SequencerEntry', keyFieldType: $pb.PbFieldType.OS, valueFieldType: $pb.PbFieldType.OS, packageName: const $pb.PackageName('mizer.settings'))
    ..m<$core.String, $core.String>(7, const $core.bool.fromEnvironment('protobuf.omit_field_names') ? '' : 'plan', entryClassName: 'Hotkeys.PlanEntry', keyFieldType: $pb.PbFieldType.OS, valueFieldType: $pb.PbFieldType.OS, packageName: const $pb.PackageName('mizer.settings'))
    ..hasRequiredFields = false
  ;

  Hotkeys._() : super();
  factory Hotkeys({
    $core.Map<$core.String, $core.String>? global,
    $core.Map<$core.String, $core.String>? layouts,
    $core.Map<$core.String, $core.String>? programmer,
    $core.Map<$core.String, $core.String>? nodes,
    $core.Map<$core.String, $core.String>? patch,
    $core.Map<$core.String, $core.String>? sequencer,
    $core.Map<$core.String, $core.String>? plan,
  }) {
    final _result = create();
    if (global != null) {
      _result.global.addAll(global);
    }
    if (layouts != null) {
      _result.layouts.addAll(layouts);
    }
    if (programmer != null) {
      _result.programmer.addAll(programmer);
    }
    if (nodes != null) {
      _result.nodes.addAll(nodes);
    }
    if (patch != null) {
      _result.patch.addAll(patch);
    }
    if (sequencer != null) {
      _result.sequencer.addAll(sequencer);
    }
    if (plan != null) {
      _result.plan.addAll(plan);
    }
    return _result;
  }
  factory Hotkeys.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory Hotkeys.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  Hotkeys clone() => Hotkeys()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  Hotkeys copyWith(void Function(Hotkeys) updates) => super.copyWith((message) => updates(message as Hotkeys)) as Hotkeys; // ignore: deprecated_member_use
  $pb.BuilderInfo get info_ => _i;
  @$core.pragma('dart2js:noInline')
  static Hotkeys create() => Hotkeys._();
  Hotkeys createEmptyInstance() => create();
  static $pb.PbList<Hotkeys> createRepeated() => $pb.PbList<Hotkeys>();
  @$core.pragma('dart2js:noInline')
  static Hotkeys getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Hotkeys>(create);
  static Hotkeys? _defaultInstance;

  @$pb.TagNumber(1)
  $core.Map<$core.String, $core.String> get global => $_getMap(0);

  @$pb.TagNumber(2)
  $core.Map<$core.String, $core.String> get layouts => $_getMap(1);

  @$pb.TagNumber(3)
  $core.Map<$core.String, $core.String> get programmer => $_getMap(2);

  @$pb.TagNumber(4)
  $core.Map<$core.String, $core.String> get nodes => $_getMap(3);

  @$pb.TagNumber(5)
  $core.Map<$core.String, $core.String> get patch => $_getMap(4);

  @$pb.TagNumber(6)
  $core.Map<$core.String, $core.String> get sequencer => $_getMap(5);

  @$pb.TagNumber(7)
  $core.Map<$core.String, $core.String> get plan => $_getMap(6);
}

