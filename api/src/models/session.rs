// This file is generated by rust-protobuf 2.18.2. Do not edit
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
//! Generated file from `session.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_2;

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct ClientAnnouncement {
    // message fields
    pub name: ::std::string::String,
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a ClientAnnouncement {
    fn default() -> &'a ClientAnnouncement {
        <ClientAnnouncement as ::protobuf::Message>::default_instance()
    }
}

impl ClientAnnouncement {
    pub fn new() -> ClientAnnouncement {
        ::std::default::Default::default()
    }

    // string name = 1;


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
}

impl ::protobuf::Message for ClientAnnouncement {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
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
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
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

    fn new() -> ClientAnnouncement {
        ClientAnnouncement::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "name",
                |m: &ClientAnnouncement| { &m.name },
                |m: &mut ClientAnnouncement| { &mut m.name },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<ClientAnnouncement>(
                "ClientAnnouncement",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static ClientAnnouncement {
        static instance: ::protobuf::rt::LazyV2<ClientAnnouncement> = ::protobuf::rt::LazyV2::INIT;
        instance.get(ClientAnnouncement::new)
    }
}

impl ::protobuf::Clear for ClientAnnouncement {
    fn clear(&mut self) {
        self.name.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ClientAnnouncement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClientAnnouncement {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct SessionRequest {
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a SessionRequest {
    fn default() -> &'a SessionRequest {
        <SessionRequest as ::protobuf::Message>::default_instance()
    }
}

impl SessionRequest {
    pub fn new() -> SessionRequest {
        ::std::default::Default::default()
    }
}

impl ::protobuf::Message for SessionRequest {
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

    fn new() -> SessionRequest {
        SessionRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let fields = ::std::vec::Vec::new();
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<SessionRequest>(
                "SessionRequest",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static SessionRequest {
        static instance: ::protobuf::rt::LazyV2<SessionRequest> = ::protobuf::rt::LazyV2::INIT;
        instance.get(SessionRequest::new)
    }
}

impl ::protobuf::Clear for SessionRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SessionRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SessionRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct Session {
    // message fields
    pub devices: ::protobuf::RepeatedField<SessionDevice>,
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Session {
    fn default() -> &'a Session {
        <Session as ::protobuf::Message>::default_instance()
    }
}

impl Session {
    pub fn new() -> Session {
        ::std::default::Default::default()
    }

    // repeated .mizer.SessionDevice devices = 1;


    pub fn get_devices(&self) -> &[SessionDevice] {
        &self.devices
    }
    pub fn clear_devices(&mut self) {
        self.devices.clear();
    }

    // Param is passed by value, moved
    pub fn set_devices(&mut self, v: ::protobuf::RepeatedField<SessionDevice>) {
        self.devices = v;
    }

    // Mutable pointer to the field.
    pub fn mut_devices(&mut self) -> &mut ::protobuf::RepeatedField<SessionDevice> {
        &mut self.devices
    }

    // Take field
    pub fn take_devices(&mut self) -> ::protobuf::RepeatedField<SessionDevice> {
        ::std::mem::replace(&mut self.devices, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for Session {
    fn is_initialized(&self) -> bool {
        for v in &self.devices {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.devices)?;
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
        for value in &self.devices {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.devices {
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

    fn new() -> Session {
        Session::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SessionDevice>>(
                "devices",
                |m: &Session| { &m.devices },
                |m: &mut Session| { &mut m.devices },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Session>(
                "Session",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Session {
        static instance: ::protobuf::rt::LazyV2<Session> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Session::new)
    }
}

impl ::protobuf::Clear for Session {
    fn clear(&mut self) {
        self.devices.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Session {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Session {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct SessionDevice {
    // message fields
    pub name: ::std::string::String,
    pub ips: ::protobuf::RepeatedField<::std::string::String>,
    pub clock: ::protobuf::SingularPtrField<DeviceClock>,
    pub ping: f64,
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a SessionDevice {
    fn default() -> &'a SessionDevice {
        <SessionDevice as ::protobuf::Message>::default_instance()
    }
}

impl SessionDevice {
    pub fn new() -> SessionDevice {
        ::std::default::Default::default()
    }

    // string name = 1;


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

    // repeated string ips = 2;


    pub fn get_ips(&self) -> &[::std::string::String] {
        &self.ips
    }
    pub fn clear_ips(&mut self) {
        self.ips.clear();
    }

    // Param is passed by value, moved
    pub fn set_ips(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.ips = v;
    }

    // Mutable pointer to the field.
    pub fn mut_ips(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.ips
    }

    // Take field
    pub fn take_ips(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.ips, ::protobuf::RepeatedField::new())
    }

    // .mizer.DeviceClock clock = 3;


    pub fn get_clock(&self) -> &DeviceClock {
        self.clock.as_ref().unwrap_or_else(|| <DeviceClock as ::protobuf::Message>::default_instance())
    }
    pub fn clear_clock(&mut self) {
        self.clock.clear();
    }

    pub fn has_clock(&self) -> bool {
        self.clock.is_some()
    }

    // Param is passed by value, moved
    pub fn set_clock(&mut self, v: DeviceClock) {
        self.clock = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_clock(&mut self) -> &mut DeviceClock {
        if self.clock.is_none() {
            self.clock.set_default();
        }
        self.clock.as_mut().unwrap()
    }

    // Take field
    pub fn take_clock(&mut self) -> DeviceClock {
        self.clock.take().unwrap_or_else(|| DeviceClock::new())
    }

    // double ping = 4;


    pub fn get_ping(&self) -> f64 {
        self.ping
    }
    pub fn clear_ping(&mut self) {
        self.ping = 0.;
    }

    // Param is passed by value, moved
    pub fn set_ping(&mut self, v: f64) {
        self.ping = v;
    }
}

impl ::protobuf::Message for SessionDevice {
    fn is_initialized(&self) -> bool {
        for v in &self.clock {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.ips)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.clock)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.ping = tmp;
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
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        for value in &self.ips {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        if let Some(ref v) = self.clock.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.ping != 0. {
            my_size += 9;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        for v in &self.ips {
            os.write_string(2, &v)?;
        };
        if let Some(ref v) = self.clock.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.ping != 0. {
            os.write_double(4, self.ping)?;
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

    fn new() -> SessionDevice {
        SessionDevice::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "name",
                |m: &SessionDevice| { &m.name },
                |m: &mut SessionDevice| { &mut m.name },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "ips",
                |m: &SessionDevice| { &m.ips },
                |m: &mut SessionDevice| { &mut m.ips },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DeviceClock>>(
                "clock",
                |m: &SessionDevice| { &m.clock },
                |m: &mut SessionDevice| { &mut m.clock },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "ping",
                |m: &SessionDevice| { &m.ping },
                |m: &mut SessionDevice| { &mut m.ping },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<SessionDevice>(
                "SessionDevice",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static SessionDevice {
        static instance: ::protobuf::rt::LazyV2<SessionDevice> = ::protobuf::rt::LazyV2::INIT;
        instance.get(SessionDevice::new)
    }
}

impl ::protobuf::Clear for SessionDevice {
    fn clear(&mut self) {
        self.name.clear();
        self.ips.clear();
        self.clock.clear();
        self.ping = 0.;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SessionDevice {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SessionDevice {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct DeviceClock {
    // message fields
    pub master: bool,
    pub drift: f64,
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a DeviceClock {
    fn default() -> &'a DeviceClock {
        <DeviceClock as ::protobuf::Message>::default_instance()
    }
}

impl DeviceClock {
    pub fn new() -> DeviceClock {
        ::std::default::Default::default()
    }

    // bool master = 1;


    pub fn get_master(&self) -> bool {
        self.master
    }
    pub fn clear_master(&mut self) {
        self.master = false;
    }

    // Param is passed by value, moved
    pub fn set_master(&mut self, v: bool) {
        self.master = v;
    }

    // double drift = 2;


    pub fn get_drift(&self) -> f64 {
        self.drift
    }
    pub fn clear_drift(&mut self) {
        self.drift = 0.;
    }

    // Param is passed by value, moved
    pub fn set_drift(&mut self, v: f64) {
        self.drift = v;
    }
}

impl ::protobuf::Message for DeviceClock {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.master = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.drift = tmp;
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
        if self.master != false {
            my_size += 2;
        }
        if self.drift != 0. {
            my_size += 9;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.master != false {
            os.write_bool(1, self.master)?;
        }
        if self.drift != 0. {
            os.write_double(2, self.drift)?;
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

    fn new() -> DeviceClock {
        DeviceClock::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "master",
                |m: &DeviceClock| { &m.master },
                |m: &mut DeviceClock| { &mut m.master },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "drift",
                |m: &DeviceClock| { &m.drift },
                |m: &mut DeviceClock| { &mut m.drift },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<DeviceClock>(
                "DeviceClock",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static DeviceClock {
        static instance: ::protobuf::rt::LazyV2<DeviceClock> = ::protobuf::rt::LazyV2::INIT;
        instance.get(DeviceClock::new)
    }
}

impl ::protobuf::Clear for DeviceClock {
    fn clear(&mut self) {
        self.master = false;
        self.drift = 0.;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DeviceClock {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DeviceClock {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\rsession.proto\x12\x05mizer\"(\n\x12ClientAnnouncement\x12\x12\n\x04n\
    ame\x18\x01\x20\x01(\tR\x04name\"\x10\n\x0eSessionRequest\"9\n\x07Sessio\
    n\x12.\n\x07devices\x18\x01\x20\x03(\x0b2\x14.mizer.SessionDeviceR\x07de\
    vices\"s\n\rSessionDevice\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\
    \x12\x10\n\x03ips\x18\x02\x20\x03(\tR\x03ips\x12(\n\x05clock\x18\x03\x20\
    \x01(\x0b2\x12.mizer.DeviceClockR\x05clock\x12\x12\n\x04ping\x18\x04\x20\
    \x01(\x01R\x04ping\";\n\x0bDeviceClock\x12\x16\n\x06master\x18\x01\x20\
    \x01(\x08R\x06master\x12\x14\n\x05drift\x18\x02\x20\x01(\x01R\x05drift2\
    \x81\x01\n\nSessionApi\x127\n\nGetSession\x12\x15.mizer.SessionRequest\
    \x1a\x0e.mizer.Session\"\00\x01\x12:\n\x0bJoinSession\x12\x19.mizer.Clie\
    ntAnnouncement\x1a\x0e.mizer.Session\"\0b\x06proto3\
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