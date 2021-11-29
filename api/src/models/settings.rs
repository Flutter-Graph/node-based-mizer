// This file is generated by rust-protobuf 2.25.0. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `settings.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_25_0;

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(feature = "with-serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct RequestSettings {
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a RequestSettings {
    fn default() -> &'a RequestSettings {
        <RequestSettings as ::protobuf::Message>::default_instance()
    }
}

impl RequestSettings {
    pub fn new() -> RequestSettings {
        ::std::default::Default::default()
    }
}

impl ::protobuf::Message for RequestSettings {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> RequestSettings {
        RequestSettings::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let fields = ::std::vec::Vec::new();
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<RequestSettings>(
                "RequestSettings",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static RequestSettings {
        static instance: ::protobuf::rt::LazyV2<RequestSettings> = ::protobuf::rt::LazyV2::INIT;
        instance.get(RequestSettings::new)
    }
}

impl ::protobuf::Clear for RequestSettings {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RequestSettings {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestSettings {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(feature = "with-serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Settings {
    // message fields
    pub hotkeys: ::protobuf::SingularPtrField<Hotkeys>,
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Settings {
    fn default() -> &'a Settings {
        <Settings as ::protobuf::Message>::default_instance()
    }
}

impl Settings {
    pub fn new() -> Settings {
        ::std::default::Default::default()
    }

    // .mizer.settings.Hotkeys hotkeys = 1;


    pub fn get_hotkeys(&self) -> &Hotkeys {
        self.hotkeys.as_ref().unwrap_or_else(|| <Hotkeys as ::protobuf::Message>::default_instance())
    }
    pub fn clear_hotkeys(&mut self) {
        self.hotkeys.clear();
    }

    pub fn has_hotkeys(&self) -> bool {
        self.hotkeys.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hotkeys(&mut self, v: Hotkeys) {
        self.hotkeys = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hotkeys(&mut self) -> &mut Hotkeys {
        if self.hotkeys.is_none() {
            self.hotkeys.set_default();
        }
        self.hotkeys.as_mut().unwrap()
    }

    // Take field
    pub fn take_hotkeys(&mut self) -> Hotkeys {
        self.hotkeys.take().unwrap_or_else(|| Hotkeys::new())
    }
}

impl ::protobuf::Message for Settings {
    fn is_initialized(&self) -> bool {
        for v in &self.hotkeys {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.hotkeys)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.hotkeys.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.hotkeys.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Settings {
        Settings::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Hotkeys>>(
                "hotkeys",
                |m: &Settings| { &m.hotkeys },
                |m: &mut Settings| { &mut m.hotkeys },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Settings>(
                "Settings",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Settings {
        static instance: ::protobuf::rt::LazyV2<Settings> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Settings::new)
    }
}

impl ::protobuf::Clear for Settings {
    fn clear(&mut self) {
        self.hotkeys.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Settings {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Settings {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(feature = "with-serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Hotkeys {
    // message fields
    pub global: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    pub layouts: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    pub fixtures: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    pub nodes: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Hotkeys {
    fn default() -> &'a Hotkeys {
        <Hotkeys as ::protobuf::Message>::default_instance()
    }
}

impl Hotkeys {
    pub fn new() -> Hotkeys {
        ::std::default::Default::default()
    }

    // repeated .mizer.settings.Hotkeys.GlobalEntry global = 1;


    pub fn get_global(&self) -> &::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &self.global
    }
    pub fn clear_global(&mut self) {
        self.global.clear();
    }

    // Param is passed by value, moved
    pub fn set_global(&mut self, v: ::std::collections::HashMap<::std::string::String, ::std::string::String>) {
        self.global = v;
    }

    // Mutable pointer to the field.
    pub fn mut_global(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &mut self.global
    }

    // Take field
    pub fn take_global(&mut self) -> ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        ::std::mem::replace(&mut self.global, ::std::collections::HashMap::new())
    }

    // repeated .mizer.settings.Hotkeys.LayoutsEntry layouts = 2;


    pub fn get_layouts(&self) -> &::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &self.layouts
    }
    pub fn clear_layouts(&mut self) {
        self.layouts.clear();
    }

    // Param is passed by value, moved
    pub fn set_layouts(&mut self, v: ::std::collections::HashMap<::std::string::String, ::std::string::String>) {
        self.layouts = v;
    }

    // Mutable pointer to the field.
    pub fn mut_layouts(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &mut self.layouts
    }

    // Take field
    pub fn take_layouts(&mut self) -> ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        ::std::mem::replace(&mut self.layouts, ::std::collections::HashMap::new())
    }

    // repeated .mizer.settings.Hotkeys.FixturesEntry fixtures = 3;


    pub fn get_fixtures(&self) -> &::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &self.fixtures
    }
    pub fn clear_fixtures(&mut self) {
        self.fixtures.clear();
    }

    // Param is passed by value, moved
    pub fn set_fixtures(&mut self, v: ::std::collections::HashMap<::std::string::String, ::std::string::String>) {
        self.fixtures = v;
    }

    // Mutable pointer to the field.
    pub fn mut_fixtures(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &mut self.fixtures
    }

    // Take field
    pub fn take_fixtures(&mut self) -> ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        ::std::mem::replace(&mut self.fixtures, ::std::collections::HashMap::new())
    }

    // repeated .mizer.settings.Hotkeys.NodesEntry nodes = 4;


    pub fn get_nodes(&self) -> &::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &self.nodes
    }
    pub fn clear_nodes(&mut self) {
        self.nodes.clear();
    }

    // Param is passed by value, moved
    pub fn set_nodes(&mut self, v: ::std::collections::HashMap<::std::string::String, ::std::string::String>) {
        self.nodes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_nodes(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &mut self.nodes
    }

    // Take field
    pub fn take_nodes(&mut self) -> ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        ::std::mem::replace(&mut self.nodes, ::std::collections::HashMap::new())
    }
}

impl ::protobuf::Message for Hotkeys {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(wire_type, is, &mut self.global)?;
                },
                2 => {
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(wire_type, is, &mut self.layouts)?;
                },
                3 => {
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(wire_type, is, &mut self.fixtures)?;
                },
                4 => {
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(wire_type, is, &mut self.nodes)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(1, &self.global);
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(2, &self.layouts);
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(3, &self.fixtures);
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(4, &self.nodes);
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(1, &self.global, os)?;
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(2, &self.layouts, os)?;
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(3, &self.fixtures, os)?;
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(4, &self.nodes, os)?;
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Hotkeys {
        Hotkeys::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(
                "global",
                |m: &Hotkeys| { &m.global },
                |m: &mut Hotkeys| { &mut m.global },
            ));
            fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(
                "layouts",
                |m: &Hotkeys| { &m.layouts },
                |m: &mut Hotkeys| { &mut m.layouts },
            ));
            fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(
                "fixtures",
                |m: &Hotkeys| { &m.fixtures },
                |m: &mut Hotkeys| { &mut m.fixtures },
            ));
            fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(
                "nodes",
                |m: &Hotkeys| { &m.nodes },
                |m: &mut Hotkeys| { &mut m.nodes },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Hotkeys>(
                "Hotkeys",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Hotkeys {
        static instance: ::protobuf::rt::LazyV2<Hotkeys> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Hotkeys::new)
    }
}

impl ::protobuf::Clear for Hotkeys {
    fn clear(&mut self) {
        self.global.clear();
        self.layouts.clear();
        self.fixtures.clear();
        self.nodes.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Hotkeys {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Hotkeys {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0esettings.proto\x12\x0emizer.settings\"\x11\n\x0fRequestSettings\"=\
    \n\x08Settings\x121\n\x07hotkeys\x18\x01\x20\x01(\x0b2\x17.mizer.setting\
    s.HotkeysR\x07hotkeys\"\xf1\x03\n\x07Hotkeys\x12;\n\x06global\x18\x01\
    \x20\x03(\x0b2#.mizer.settings.Hotkeys.GlobalEntryR\x06global\x12>\n\x07\
    layouts\x18\x02\x20\x03(\x0b2$.mizer.settings.Hotkeys.LayoutsEntryR\x07l\
    ayouts\x12A\n\x08fixtures\x18\x03\x20\x03(\x0b2%.mizer.settings.Hotkeys.\
    FixturesEntryR\x08fixtures\x128\n\x05nodes\x18\x04\x20\x03(\x0b2\".mizer\
    .settings.Hotkeys.NodesEntryR\x05nodes\x1a9\n\x0bGlobalEntry\x12\x10\n\
    \x03key\x18\x01\x20\x01(\tR\x03key\x12\x14\n\x05value\x18\x02\x20\x01(\t\
    R\x05value:\x028\x01\x1a:\n\x0cLayoutsEntry\x12\x10\n\x03key\x18\x01\x20\
    \x01(\tR\x03key\x12\x14\n\x05value\x18\x02\x20\x01(\tR\x05value:\x028\
    \x01\x1a;\n\rFixturesEntry\x12\x10\n\x03key\x18\x01\x20\x01(\tR\x03key\
    \x12\x14\n\x05value\x18\x02\x20\x01(\tR\x05value:\x028\x01\x1a8\n\nNodes\
    Entry\x12\x10\n\x03key\x18\x01\x20\x01(\tR\x03key\x12\x14\n\x05value\x18\
    \x02\x20\x01(\tR\x05value:\x028\x012Z\n\x0bSettingsApi\x12K\n\x0cLoadSet\
    tings\x12\x1f.mizer.settings.RequestSettings\x1a\x18.mizer.settings.Sett\
    ings\"\0b\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}