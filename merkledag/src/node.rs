// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct PBLink {
    // message fields
    Hash: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    Name: ::protobuf::SingularField<::std::string::String>,
    Tsize: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PBLink {}

impl PBLink {
    pub fn new() -> PBLink {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PBLink {
        static mut instance: ::protobuf::lazy::Lazy<PBLink> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PBLink,
        };
        unsafe {
            instance.get(PBLink::new)
        }
    }

    // optional bytes Hash = 1;

    pub fn clear_Hash(&mut self) {
        self.Hash.clear();
    }

    pub fn has_Hash(&self) -> bool {
        self.Hash.is_some()
    }

    // Param is passed by value, moved
    pub fn set_Hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.Hash = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_Hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.Hash.is_none() {
            self.Hash.set_default();
        }
        self.Hash.as_mut().unwrap()
    }

    // Take field
    pub fn take_Hash(&mut self) -> ::std::vec::Vec<u8> {
        self.Hash.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_Hash(&self) -> &[u8] {
        match self.Hash.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_Hash_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.Hash
    }

    fn mut_Hash_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.Hash
    }

    // optional string Name = 2;

    pub fn clear_Name(&mut self) {
        self.Name.clear();
    }

    pub fn has_Name(&self) -> bool {
        self.Name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_Name(&mut self, v: ::std::string::String) {
        self.Name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_Name(&mut self) -> &mut ::std::string::String {
        if self.Name.is_none() {
            self.Name.set_default();
        }
        self.Name.as_mut().unwrap()
    }

    // Take field
    pub fn take_Name(&mut self) -> ::std::string::String {
        self.Name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_Name(&self) -> &str {
        match self.Name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_Name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.Name
    }

    fn mut_Name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.Name
    }

    // optional uint64 Tsize = 3;

    pub fn clear_Tsize(&mut self) {
        self.Tsize = ::std::option::Option::None;
    }

    pub fn has_Tsize(&self) -> bool {
        self.Tsize.is_some()
    }

    // Param is passed by value, moved
    pub fn set_Tsize(&mut self, v: u64) {
        self.Tsize = ::std::option::Option::Some(v);
    }

    pub fn get_Tsize(&self) -> u64 {
        self.Tsize.unwrap_or(0)
    }

    fn get_Tsize_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.Tsize
    }

    fn mut_Tsize_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.Tsize
    }
}

impl ::protobuf::Message for PBLink {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.Hash)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.Name)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.Tsize = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.Hash.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        if let Some(ref v) = self.Name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.Tsize {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.Hash.as_ref() {
            os.write_bytes(1, &v)?;
        }
        if let Some(ref v) = self.Name.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(v) = self.Tsize {
            os.write_uint64(3, v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PBLink {
    fn new() -> PBLink {
        PBLink::new()
    }

    fn descriptor_static(_: ::std::option::Option<PBLink>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "Hash",
                    PBLink::get_Hash_for_reflect,
                    PBLink::mut_Hash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "Name",
                    PBLink::get_Name_for_reflect,
                    PBLink::mut_Name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "Tsize",
                    PBLink::get_Tsize_for_reflect,
                    PBLink::mut_Tsize_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PBLink>(
                    "PBLink",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PBLink {
    fn clear(&mut self) {
        self.clear_Hash();
        self.clear_Name();
        self.clear_Tsize();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PBLink {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PBLink {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PBNode {
    // message fields
    Links: ::protobuf::RepeatedField<PBLink>,
    Data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PBNode {}

impl PBNode {
    pub fn new() -> PBNode {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PBNode {
        static mut instance: ::protobuf::lazy::Lazy<PBNode> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PBNode,
        };
        unsafe {
            instance.get(PBNode::new)
        }
    }

    // repeated .merkledag.PBLink Links = 2;

    pub fn clear_Links(&mut self) {
        self.Links.clear();
    }

    // Param is passed by value, moved
    pub fn set_Links(&mut self, v: ::protobuf::RepeatedField<PBLink>) {
        self.Links = v;
    }

    // Mutable pointer to the field.
    pub fn mut_Links(&mut self) -> &mut ::protobuf::RepeatedField<PBLink> {
        &mut self.Links
    }

    // Take field
    pub fn take_Links(&mut self) -> ::protobuf::RepeatedField<PBLink> {
        ::std::mem::replace(&mut self.Links, ::protobuf::RepeatedField::new())
    }

    pub fn get_Links(&self) -> &[PBLink] {
        &self.Links
    }

    fn get_Links_for_reflect(&self) -> &::protobuf::RepeatedField<PBLink> {
        &self.Links
    }

    fn mut_Links_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<PBLink> {
        &mut self.Links
    }

    // optional bytes Data = 1;

    pub fn clear_Data(&mut self) {
        self.Data.clear();
    }

    pub fn has_Data(&self) -> bool {
        self.Data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_Data(&mut self, v: ::std::vec::Vec<u8>) {
        self.Data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_Data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.Data.is_none() {
            self.Data.set_default();
        }
        self.Data.as_mut().unwrap()
    }

    // Take field
    pub fn take_Data(&mut self) -> ::std::vec::Vec<u8> {
        self.Data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_Data(&self) -> &[u8] {
        match self.Data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_Data_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.Data
    }

    fn mut_Data_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.Data
    }
}

impl ::protobuf::Message for PBNode {
    fn is_initialized(&self) -> bool {
        for v in &self.Links {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.Links)?;
                },
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.Data)?;
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
        for value in &self.Links {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.Data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.Links {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.Data.as_ref() {
            os.write_bytes(1, &v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PBNode {
    fn new() -> PBNode {
        PBNode::new()
    }

    fn descriptor_static(_: ::std::option::Option<PBNode>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PBLink>>(
                    "Links",
                    PBNode::get_Links_for_reflect,
                    PBNode::mut_Links_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "Data",
                    PBNode::get_Data_for_reflect,
                    PBNode::mut_Data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PBNode>(
                    "PBNode",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PBNode {
    fn clear(&mut self) {
        self.clear_Links();
        self.clear_Data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PBNode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PBNode {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\nnode.proto\x12\tmerkledag\"F\n\x06PBLink\x12\x12\n\x04Hash\x18\x01\
    \x20\x01(\x0cR\x04Hash\x12\x12\n\x04Name\x18\x02\x20\x01(\tR\x04Name\x12\
    \x14\n\x05Tsize\x18\x03\x20\x01(\x04R\x05Tsize\"E\n\x06PBNode\x12'\n\x05\
    Links\x18\x02\x20\x03(\x0b2\x11.merkledag.PBLinkR\x05Links\x12\x12\n\x04\
    Data\x18\x01\x20\x01(\x0cR\x04DataJ\xfc\x04\n\x06\x12\x04\0\0\x19\x01\n\
    \x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x01\x08\x11\n$\n\
    \x02\x04\0\x12\x04\x05\0\x0f\x01\x1a\x18\x20An\x20IPFS\x20MerkleDAG\x20L\
    ink\n\n\n\n\x03\x04\0\x01\x12\x03\x05\x08\x0e\n-\n\x04\x04\0\x02\0\x12\
    \x03\x08\x02\x1a\x1a\x20\x20multihash\x20of\x20the\x20target\x20object\n\
    \n\x0c\n\x05\x04\0\x02\0\x04\x12\x03\x08\x02\n\n\x0c\n\x05\x04\0\x02\0\
    \x05\x12\x03\x08\x0b\x10\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x08\x11\x15\
    \n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x08\x18\x19\n;\n\x04\x04\0\x02\x01\
    \x12\x03\x0b\x02\x1b\x1a.\x20utf\x20string\x20name.\x20should\x20be\x20u\
    nique\x20per\x20object\n\n\x0c\n\x05\x04\0\x02\x01\x04\x12\x03\x0b\x02\n\
    \n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x0b\x0b\x11\n\x0c\n\x05\x04\0\x02\
    \x01\x01\x12\x03\x0b\x12\x16\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x0b\
    \x19\x1a\n/\n\x04\x04\0\x02\x02\x12\x03\x0e\x02\x1c\x1a\"\x20cumulative\
    \x20size\x20of\x20target\x20object\n\n\x0c\n\x05\x04\0\x02\x02\x04\x12\
    \x03\x0e\x02\n\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03\x0e\x0b\x11\n\x0c\n\
    \x05\x04\0\x02\x02\x01\x12\x03\x0e\x12\x17\n\x0c\n\x05\x04\0\x02\x02\x03\
    \x12\x03\x0e\x1a\x1b\n$\n\x02\x04\x01\x12\x04\x12\0\x19\x01\x1a\x18\x20A\
    n\x20IPFS\x20MerkleDAG\x20Node\n\n\n\n\x03\x04\x01\x01\x12\x03\x12\x08\
    \x0e\n$\n\x04\x04\x01\x02\0\x12\x03\x15\x02\x1c\x1a\x17\x20refs\x20to\
    \x20other\x20objects\n\n\x0c\n\x05\x04\x01\x02\0\x04\x12\x03\x15\x02\n\n\
    \x0c\n\x05\x04\x01\x02\0\x06\x12\x03\x15\x0b\x11\n\x0c\n\x05\x04\x01\x02\
    \0\x01\x12\x03\x15\x12\x17\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x15\x1a\
    \x1b\n\x1f\n\x04\x04\x01\x02\x01\x12\x03\x18\x02\x1a\x1a\x12\x20opaque\
    \x20user\x20data\n\n\x0c\n\x05\x04\x01\x02\x01\x04\x12\x03\x18\x02\n\n\
    \x0c\n\x05\x04\x01\x02\x01\x05\x12\x03\x18\x0b\x10\n\x0c\n\x05\x04\x01\
    \x02\x01\x01\x12\x03\x18\x11\x15\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\
    \x18\x18\x19\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
