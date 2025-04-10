// This file is generated by rust-protobuf 3.7.1. Do not edit
// .proto file is parsed by pure
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `LAIAPKNDBPH.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:LAIAPKNDBPH)
pub enum LAIAPKNDBPH {
    // @@protoc_insertion_point(enum_value:LAIAPKNDBPH.ROGUE_COMMON_MIRACLE_DISPLAY_TYPE_NONE)
    ROGUE_COMMON_MIRACLE_DISPLAY_TYPE_NONE = 0,
    // @@protoc_insertion_point(enum_value:LAIAPKNDBPH.ROGUE_COMMON_MIRACLE_DISPLAY_TYPE_ADD)
    ROGUE_COMMON_MIRACLE_DISPLAY_TYPE_ADD = 1,
    // @@protoc_insertion_point(enum_value:LAIAPKNDBPH.ROGUE_COMMON_MIRACLE_DISPLAY_TYPE_REMOVE)
    ROGUE_COMMON_MIRACLE_DISPLAY_TYPE_REMOVE = 2,
    // @@protoc_insertion_point(enum_value:LAIAPKNDBPH.ROGUE_COMMON_MIRACLE_DISPLAY_TYPE_REPAIR)
    ROGUE_COMMON_MIRACLE_DISPLAY_TYPE_REPAIR = 3,
}

impl ::protobuf::Enum for LAIAPKNDBPH {
    const NAME: &'static str = "LAIAPKNDBPH";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<LAIAPKNDBPH> {
        match value {
            0 => ::std::option::Option::Some(LAIAPKNDBPH::ROGUE_COMMON_MIRACLE_DISPLAY_TYPE_NONE),
            1 => ::std::option::Option::Some(LAIAPKNDBPH::ROGUE_COMMON_MIRACLE_DISPLAY_TYPE_ADD),
            2 => ::std::option::Option::Some(LAIAPKNDBPH::ROGUE_COMMON_MIRACLE_DISPLAY_TYPE_REMOVE),
            3 => ::std::option::Option::Some(LAIAPKNDBPH::ROGUE_COMMON_MIRACLE_DISPLAY_TYPE_REPAIR),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<LAIAPKNDBPH> {
        match str {
            "ROGUE_COMMON_MIRACLE_DISPLAY_TYPE_NONE" => ::std::option::Option::Some(LAIAPKNDBPH::ROGUE_COMMON_MIRACLE_DISPLAY_TYPE_NONE),
            "ROGUE_COMMON_MIRACLE_DISPLAY_TYPE_ADD" => ::std::option::Option::Some(LAIAPKNDBPH::ROGUE_COMMON_MIRACLE_DISPLAY_TYPE_ADD),
            "ROGUE_COMMON_MIRACLE_DISPLAY_TYPE_REMOVE" => ::std::option::Option::Some(LAIAPKNDBPH::ROGUE_COMMON_MIRACLE_DISPLAY_TYPE_REMOVE),
            "ROGUE_COMMON_MIRACLE_DISPLAY_TYPE_REPAIR" => ::std::option::Option::Some(LAIAPKNDBPH::ROGUE_COMMON_MIRACLE_DISPLAY_TYPE_REPAIR),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [LAIAPKNDBPH] = &[
        LAIAPKNDBPH::ROGUE_COMMON_MIRACLE_DISPLAY_TYPE_NONE,
        LAIAPKNDBPH::ROGUE_COMMON_MIRACLE_DISPLAY_TYPE_ADD,
        LAIAPKNDBPH::ROGUE_COMMON_MIRACLE_DISPLAY_TYPE_REMOVE,
        LAIAPKNDBPH::ROGUE_COMMON_MIRACLE_DISPLAY_TYPE_REPAIR,
    ];
}

impl ::protobuf::EnumFull for LAIAPKNDBPH {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("LAIAPKNDBPH").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for LAIAPKNDBPH {
    fn default() -> Self {
        LAIAPKNDBPH::ROGUE_COMMON_MIRACLE_DISPLAY_TYPE_NONE
    }
}

impl LAIAPKNDBPH {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<LAIAPKNDBPH>("LAIAPKNDBPH")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11LAIAPKNDBPH.proto*\xc0\x01\n\x0bLAIAPKNDBPH\x12*\n&ROGUE_COMMON_MI\
    RACLE_DISPLAY_TYPE_NONE\x10\0\x12)\n%ROGUE_COMMON_MIRACLE_DISPLAY_TYPE_A\
    DD\x10\x01\x12,\n(ROGUE_COMMON_MIRACLE_DISPLAY_TYPE_REMOVE\x10\x02\x12,\
    \n(ROGUE_COMMON_MIRACLE_DISPLAY_TYPE_REPAIR\x10\x03b\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(0);
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(LAIAPKNDBPH::generated_enum_descriptor_data());
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
