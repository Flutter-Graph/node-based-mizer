///
//  Generated code. Do not modify.
//  source: fixtures.proto
//
// @dart = 2.7
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields

// ignore_for_file: UNDEFINED_SHOWN_NAME
import 'dart:core' as $core;
import 'package:protobuf/protobuf.dart' as $pb;

class FixtureControl extends $pb.ProtobufEnum {
  static const FixtureControl INTENSITY = FixtureControl._(0, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'INTENSITY');
  static const FixtureControl SHUTTER = FixtureControl._(1, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'SHUTTER');
  static const FixtureControl COLOR = FixtureControl._(2, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'COLOR');
  static const FixtureControl PAN = FixtureControl._(3, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'PAN');
  static const FixtureControl TILT = FixtureControl._(4, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'TILT');
  static const FixtureControl FOCUS = FixtureControl._(5, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'FOCUS');
  static const FixtureControl ZOOM = FixtureControl._(6, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'ZOOM');
  static const FixtureControl PRISM = FixtureControl._(7, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'PRISM');
  static const FixtureControl IRIS = FixtureControl._(8, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'IRIS');
  static const FixtureControl FROST = FixtureControl._(9, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'FROST');
  static const FixtureControl GENERIC = FixtureControl._(10, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'GENERIC');

  static const $core.List<FixtureControl> values = <FixtureControl> [
    INTENSITY,
    SHUTTER,
    COLOR,
    PAN,
    TILT,
    FOCUS,
    ZOOM,
    PRISM,
    IRIS,
    FROST,
    GENERIC,
  ];

  static final $core.Map<$core.int, FixtureControl> _byValue = $pb.ProtobufEnum.initByValue(values);
  static FixtureControl valueOf($core.int value) => _byValue[value];

  const FixtureControl._($core.int v, $core.String n) : super(v, n);
}

