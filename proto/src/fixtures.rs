// This file is generated by rust-protobuf 2.18.1. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![rustfmt::skip]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `fixtures.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_1;

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct GetFixturesRequest {
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a GetFixturesRequest {
    fn default() -> &'a GetFixturesRequest {
        <GetFixturesRequest as ::protobuf::Message>::default_instance()
    }
}

impl GetFixturesRequest {
    pub fn new() -> GetFixturesRequest {
        ::std::default::Default::default()
    }
}

impl ::protobuf::Message for GetFixturesRequest {
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

    fn new() -> GetFixturesRequest {
        GetFixturesRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let fields = ::std::vec::Vec::new();
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<GetFixturesRequest>(
                "GetFixturesRequest",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static GetFixturesRequest {
        static instance: ::protobuf::rt::LazyV2<GetFixturesRequest> = ::protobuf::rt::LazyV2::INIT;
        instance.get(GetFixturesRequest::new)
    }
}

impl ::protobuf::Clear for GetFixturesRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetFixturesRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetFixturesRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct Fixtures {
    // message fields
    pub fixtures: ::protobuf::RepeatedField<Fixture>,
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Fixtures {
    fn default() -> &'a Fixtures {
        <Fixtures as ::protobuf::Message>::default_instance()
    }
}

impl Fixtures {
    pub fn new() -> Fixtures {
        ::std::default::Default::default()
    }

    // repeated .mizer.Fixture fixtures = 1;


    pub fn get_fixtures(&self) -> &[Fixture] {
        &self.fixtures
    }
    pub fn clear_fixtures(&mut self) {
        self.fixtures.clear();
    }

    // Param is passed by value, moved
    pub fn set_fixtures(&mut self, v: ::protobuf::RepeatedField<Fixture>) {
        self.fixtures = v;
    }

    // Mutable pointer to the field.
    pub fn mut_fixtures(&mut self) -> &mut ::protobuf::RepeatedField<Fixture> {
        &mut self.fixtures
    }

    // Take field
    pub fn take_fixtures(&mut self) -> ::protobuf::RepeatedField<Fixture> {
        ::std::mem::replace(&mut self.fixtures, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for Fixtures {
    fn is_initialized(&self) -> bool {
        for v in &self.fixtures {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.fixtures)?;
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
        for value in &self.fixtures {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.fixtures {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

    fn new() -> Fixtures {
        Fixtures::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Fixture>>(
                "fixtures",
                |m: &Fixtures| { &m.fixtures },
                |m: &mut Fixtures| { &mut m.fixtures },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Fixtures>(
                "Fixtures",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Fixtures {
        static instance: ::protobuf::rt::LazyV2<Fixtures> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Fixtures::new)
    }
}

impl ::protobuf::Clear for Fixtures {
    fn clear(&mut self) {
        self.fixtures.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Fixtures {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Fixtures {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct Fixture {
    // message fields
    pub id: ::std::string::String,
    pub name: ::std::string::String,
    pub manufacturer: ::std::string::String,
    pub universe: i32,
    pub channel: i32,
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Fixture {
    fn default() -> &'a Fixture {
        <Fixture as ::protobuf::Message>::default_instance()
    }
}

impl Fixture {
    pub fn new() -> Fixture {
        ::std::default::Default::default()
    }

    // string id = 1;


    pub fn get_id(&self) -> &str {
        &self.id
    }
    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::string::String {
        &mut self.id
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.id, ::std::string::String::new())
    }

    // string name = 2;


    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    // string manufacturer = 3;


    pub fn get_manufacturer(&self) -> &str {
        &self.manufacturer
    }
    pub fn clear_manufacturer(&mut self) {
        self.manufacturer.clear();
    }

    // Param is passed by value, moved
    pub fn set_manufacturer(&mut self, v: ::std::string::String) {
        self.manufacturer = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_manufacturer(&mut self) -> &mut ::std::string::String {
        &mut self.manufacturer
    }

    // Take field
    pub fn take_manufacturer(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.manufacturer, ::std::string::String::new())
    }

    // int32 universe = 4;


    pub fn get_universe(&self) -> i32 {
        self.universe
    }
    pub fn clear_universe(&mut self) {
        self.universe = 0;
    }

    // Param is passed by value, moved
    pub fn set_universe(&mut self, v: i32) {
        self.universe = v;
    }

    // int32 channel = 5;


    pub fn get_channel(&self) -> i32 {
        self.channel
    }
    pub fn clear_channel(&mut self) {
        self.channel = 0;
    }

    // Param is passed by value, moved
    pub fn set_channel(&mut self, v: i32) {
        self.channel = v;
    }
}

impl ::protobuf::Message for Fixture {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.manufacturer)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.universe = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.channel = tmp;
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
        if !self.id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.id);
        }
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.name);
        }
        if !self.manufacturer.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.manufacturer);
        }
        if self.universe != 0 {
            my_size += ::protobuf::rt::value_size(4, self.universe, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.channel != 0 {
            my_size += ::protobuf::rt::value_size(5, self.channel, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.id.is_empty() {
            os.write_string(1, &self.id)?;
        }
        if !self.name.is_empty() {
            os.write_string(2, &self.name)?;
        }
        if !self.manufacturer.is_empty() {
            os.write_string(3, &self.manufacturer)?;
        }
        if self.universe != 0 {
            os.write_int32(4, self.universe)?;
        }
        if self.channel != 0 {
            os.write_int32(5, self.channel)?;
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

    fn new() -> Fixture {
        Fixture::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "id",
                |m: &Fixture| { &m.id },
                |m: &mut Fixture| { &mut m.id },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "name",
                |m: &Fixture| { &m.name },
                |m: &mut Fixture| { &mut m.name },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "manufacturer",
                |m: &Fixture| { &m.manufacturer },
                |m: &mut Fixture| { &mut m.manufacturer },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                "universe",
                |m: &Fixture| { &m.universe },
                |m: &mut Fixture| { &mut m.universe },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                "channel",
                |m: &Fixture| { &m.channel },
                |m: &mut Fixture| { &mut m.channel },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Fixture>(
                "Fixture",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Fixture {
        static instance: ::protobuf::rt::LazyV2<Fixture> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Fixture::new)
    }
}

impl ::protobuf::Clear for Fixture {
    fn clear(&mut self) {
        self.id.clear();
        self.name.clear();
        self.manufacturer.clear();
        self.universe = 0;
        self.channel = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Fixture {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Fixture {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0efixtures.proto\x12\x05mizer\"\x14\n\x12GetFixturesRequest\"6\n\x08\
    Fixtures\x12*\n\x08fixtures\x18\x01\x20\x03(\x0b2\x0e.mizer.FixtureR\x08\
    fixtures\"\x87\x01\n\x07Fixture\x12\x0e\n\x02id\x18\x01\x20\x01(\tR\x02i\
    d\x12\x12\n\x04name\x18\x02\x20\x01(\tR\x04name\x12\"\n\x0cmanufacturer\
    \x18\x03\x20\x01(\tR\x0cmanufacturer\x12\x1a\n\x08universe\x18\x04\x20\
    \x01(\x05R\x08universe\x12\x18\n\x07channel\x18\x05\x20\x01(\x05R\x07cha\
    nnel2J\n\x0bFixturesApi\x12;\n\x0bGetFixtures\x12\x19.mizer.GetFixturesR\
    equest\x1a\x0f.mizer.Fixtures\"\0b\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}