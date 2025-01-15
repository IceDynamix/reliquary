// This file is generated by rust-protobuf 3.4.0. Do not edit
// .proto file is parsed by pure
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
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `OIJNFMCBFJC.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:OIJNFMCBFJC)
pub enum OIJNFMCBFJC {
    // @@protoc_insertion_point(enum_value:OIJNFMCBFJC.ROGUE_COMMON_MIRACLE_SELECT_SOURCE_TYPE_NONE)
    ROGUE_COMMON_MIRACLE_SELECT_SOURCE_TYPE_NONE = 0,
    // @@protoc_insertion_point(enum_value:OIJNFMCBFJC.ROGUE_COMMON_MIRACLE_SELECT_SOURCE_TYPE_DICE_ROLL)
    ROGUE_COMMON_MIRACLE_SELECT_SOURCE_TYPE_DICE_ROLL = 1,
    // @@protoc_insertion_point(enum_value:OIJNFMCBFJC.ROGUE_COMMON_MIRACLE_SELECT_SOURCE_TYPE_AEON)
    ROGUE_COMMON_MIRACLE_SELECT_SOURCE_TYPE_AEON = 2,
    // @@protoc_insertion_point(enum_value:OIJNFMCBFJC.ROGUE_COMMON_MIRACLE_SELECT_SOURCE_TYPE_BOARD_EVENT)
    ROGUE_COMMON_MIRACLE_SELECT_SOURCE_TYPE_BOARD_EVENT = 3,
    // @@protoc_insertion_point(enum_value:OIJNFMCBFJC.ROGUE_COMMON_MIRACLE_SELECT_SOURCE_TYPE_LEVEL_MECHANISM)
    ROGUE_COMMON_MIRACLE_SELECT_SOURCE_TYPE_LEVEL_MECHANISM = 4,
}

impl ::protobuf::Enum for OIJNFMCBFJC {
    const NAME: &'static str = "OIJNFMCBFJC";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<OIJNFMCBFJC> {
        match value {
            0 => ::std::option::Option::Some(OIJNFMCBFJC::ROGUE_COMMON_MIRACLE_SELECT_SOURCE_TYPE_NONE),
            1 => ::std::option::Option::Some(OIJNFMCBFJC::ROGUE_COMMON_MIRACLE_SELECT_SOURCE_TYPE_DICE_ROLL),
            2 => ::std::option::Option::Some(OIJNFMCBFJC::ROGUE_COMMON_MIRACLE_SELECT_SOURCE_TYPE_AEON),
            3 => ::std::option::Option::Some(OIJNFMCBFJC::ROGUE_COMMON_MIRACLE_SELECT_SOURCE_TYPE_BOARD_EVENT),
            4 => ::std::option::Option::Some(OIJNFMCBFJC::ROGUE_COMMON_MIRACLE_SELECT_SOURCE_TYPE_LEVEL_MECHANISM),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<OIJNFMCBFJC> {
        match str {
            "ROGUE_COMMON_MIRACLE_SELECT_SOURCE_TYPE_NONE" => ::std::option::Option::Some(OIJNFMCBFJC::ROGUE_COMMON_MIRACLE_SELECT_SOURCE_TYPE_NONE),
            "ROGUE_COMMON_MIRACLE_SELECT_SOURCE_TYPE_DICE_ROLL" => ::std::option::Option::Some(OIJNFMCBFJC::ROGUE_COMMON_MIRACLE_SELECT_SOURCE_TYPE_DICE_ROLL),
            "ROGUE_COMMON_MIRACLE_SELECT_SOURCE_TYPE_AEON" => ::std::option::Option::Some(OIJNFMCBFJC::ROGUE_COMMON_MIRACLE_SELECT_SOURCE_TYPE_AEON),
            "ROGUE_COMMON_MIRACLE_SELECT_SOURCE_TYPE_BOARD_EVENT" => ::std::option::Option::Some(OIJNFMCBFJC::ROGUE_COMMON_MIRACLE_SELECT_SOURCE_TYPE_BOARD_EVENT),
            "ROGUE_COMMON_MIRACLE_SELECT_SOURCE_TYPE_LEVEL_MECHANISM" => ::std::option::Option::Some(OIJNFMCBFJC::ROGUE_COMMON_MIRACLE_SELECT_SOURCE_TYPE_LEVEL_MECHANISM),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [OIJNFMCBFJC] = &[
        OIJNFMCBFJC::ROGUE_COMMON_MIRACLE_SELECT_SOURCE_TYPE_NONE,
        OIJNFMCBFJC::ROGUE_COMMON_MIRACLE_SELECT_SOURCE_TYPE_DICE_ROLL,
        OIJNFMCBFJC::ROGUE_COMMON_MIRACLE_SELECT_SOURCE_TYPE_AEON,
        OIJNFMCBFJC::ROGUE_COMMON_MIRACLE_SELECT_SOURCE_TYPE_BOARD_EVENT,
        OIJNFMCBFJC::ROGUE_COMMON_MIRACLE_SELECT_SOURCE_TYPE_LEVEL_MECHANISM,
    ];
}

impl ::protobuf::EnumFull for OIJNFMCBFJC {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("OIJNFMCBFJC").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for OIJNFMCBFJC {
    fn default() -> Self {
        OIJNFMCBFJC::ROGUE_COMMON_MIRACLE_SELECT_SOURCE_TYPE_NONE
    }
}

impl OIJNFMCBFJC {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<OIJNFMCBFJC>("OIJNFMCBFJC")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11OIJNFMCBFJC.proto*\x9e\x02\n\x0bOIJNFMCBFJC\x120\n,ROGUE_COMMON_MI\
    RACLE_SELECT_SOURCE_TYPE_NONE\x10\0\x125\n1ROGUE_COMMON_MIRACLE_SELECT_S\
    OURCE_TYPE_DICE_ROLL\x10\x01\x120\n,ROGUE_COMMON_MIRACLE_SELECT_SOURCE_T\
    YPE_AEON\x10\x02\x127\n3ROGUE_COMMON_MIRACLE_SELECT_SOURCE_TYPE_BOARD_EV\
    ENT\x10\x03\x12;\n7ROGUE_COMMON_MIRACLE_SELECT_SOURCE_TYPE_LEVEL_MECHANI\
    SM\x10\x04b\x06proto3\
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
            enums.push(OIJNFMCBFJC::generated_enum_descriptor_data());
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