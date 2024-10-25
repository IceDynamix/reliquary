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

//! Generated file from `DMMJGAAJOJP.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:DMMJGAAJOJP)
pub enum DMMJGAAJOJP {
    // @@protoc_insertion_point(enum_value:DMMJGAAJOJP.ROGUE_BUFF_SOURCE_TYPE_NONE)
    ROGUE_BUFF_SOURCE_TYPE_NONE = 0,
    // @@protoc_insertion_point(enum_value:DMMJGAAJOJP.ROGUE_BUFF_SOURCE_TYPE_SELECT)
    ROGUE_BUFF_SOURCE_TYPE_SELECT = 1,
    // @@protoc_insertion_point(enum_value:DMMJGAAJOJP.ROGUE_BUFF_SOURCE_TYPE_ENHANCE)
    ROGUE_BUFF_SOURCE_TYPE_ENHANCE = 2,
    // @@protoc_insertion_point(enum_value:DMMJGAAJOJP.ROGUE_BUFF_SOURCE_TYPE_MIRACLE)
    ROGUE_BUFF_SOURCE_TYPE_MIRACLE = 3,
    // @@protoc_insertion_point(enum_value:DMMJGAAJOJP.ROGUE_BUFF_SOURCE_TYPE_DIALOGUE)
    ROGUE_BUFF_SOURCE_TYPE_DIALOGUE = 4,
    // @@protoc_insertion_point(enum_value:DMMJGAAJOJP.ROGUE_BUFF_SOURCE_TYPE_BONUS)
    ROGUE_BUFF_SOURCE_TYPE_BONUS = 5,
    // @@protoc_insertion_point(enum_value:DMMJGAAJOJP.ROGUE_BUFF_SOURCE_TYPE_MAZE_SKILL)
    ROGUE_BUFF_SOURCE_TYPE_MAZE_SKILL = 6,
    // @@protoc_insertion_point(enum_value:DMMJGAAJOJP.ROGUE_BUFF_SOURCE_TYPE_SHOP)
    ROGUE_BUFF_SOURCE_TYPE_SHOP = 7,
    // @@protoc_insertion_point(enum_value:DMMJGAAJOJP.ROGUE_BUFF_SOURCE_TYPE_LEVEL_MECHANISM)
    ROGUE_BUFF_SOURCE_TYPE_LEVEL_MECHANISM = 8,
    // @@protoc_insertion_point(enum_value:DMMJGAAJOJP.ROGUE_BUFF_SOURCE_TYPE_ENDLESS_LEVEL_START)
    ROGUE_BUFF_SOURCE_TYPE_ENDLESS_LEVEL_START = 9,
}

impl ::protobuf::Enum for DMMJGAAJOJP {
    const NAME: &'static str = "DMMJGAAJOJP";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DMMJGAAJOJP> {
        match value {
            0 => ::std::option::Option::Some(DMMJGAAJOJP::ROGUE_BUFF_SOURCE_TYPE_NONE),
            1 => ::std::option::Option::Some(DMMJGAAJOJP::ROGUE_BUFF_SOURCE_TYPE_SELECT),
            2 => ::std::option::Option::Some(DMMJGAAJOJP::ROGUE_BUFF_SOURCE_TYPE_ENHANCE),
            3 => ::std::option::Option::Some(DMMJGAAJOJP::ROGUE_BUFF_SOURCE_TYPE_MIRACLE),
            4 => ::std::option::Option::Some(DMMJGAAJOJP::ROGUE_BUFF_SOURCE_TYPE_DIALOGUE),
            5 => ::std::option::Option::Some(DMMJGAAJOJP::ROGUE_BUFF_SOURCE_TYPE_BONUS),
            6 => ::std::option::Option::Some(DMMJGAAJOJP::ROGUE_BUFF_SOURCE_TYPE_MAZE_SKILL),
            7 => ::std::option::Option::Some(DMMJGAAJOJP::ROGUE_BUFF_SOURCE_TYPE_SHOP),
            8 => ::std::option::Option::Some(DMMJGAAJOJP::ROGUE_BUFF_SOURCE_TYPE_LEVEL_MECHANISM),
            9 => ::std::option::Option::Some(DMMJGAAJOJP::ROGUE_BUFF_SOURCE_TYPE_ENDLESS_LEVEL_START),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<DMMJGAAJOJP> {
        match str {
            "ROGUE_BUFF_SOURCE_TYPE_NONE" => ::std::option::Option::Some(DMMJGAAJOJP::ROGUE_BUFF_SOURCE_TYPE_NONE),
            "ROGUE_BUFF_SOURCE_TYPE_SELECT" => ::std::option::Option::Some(DMMJGAAJOJP::ROGUE_BUFF_SOURCE_TYPE_SELECT),
            "ROGUE_BUFF_SOURCE_TYPE_ENHANCE" => ::std::option::Option::Some(DMMJGAAJOJP::ROGUE_BUFF_SOURCE_TYPE_ENHANCE),
            "ROGUE_BUFF_SOURCE_TYPE_MIRACLE" => ::std::option::Option::Some(DMMJGAAJOJP::ROGUE_BUFF_SOURCE_TYPE_MIRACLE),
            "ROGUE_BUFF_SOURCE_TYPE_DIALOGUE" => ::std::option::Option::Some(DMMJGAAJOJP::ROGUE_BUFF_SOURCE_TYPE_DIALOGUE),
            "ROGUE_BUFF_SOURCE_TYPE_BONUS" => ::std::option::Option::Some(DMMJGAAJOJP::ROGUE_BUFF_SOURCE_TYPE_BONUS),
            "ROGUE_BUFF_SOURCE_TYPE_MAZE_SKILL" => ::std::option::Option::Some(DMMJGAAJOJP::ROGUE_BUFF_SOURCE_TYPE_MAZE_SKILL),
            "ROGUE_BUFF_SOURCE_TYPE_SHOP" => ::std::option::Option::Some(DMMJGAAJOJP::ROGUE_BUFF_SOURCE_TYPE_SHOP),
            "ROGUE_BUFF_SOURCE_TYPE_LEVEL_MECHANISM" => ::std::option::Option::Some(DMMJGAAJOJP::ROGUE_BUFF_SOURCE_TYPE_LEVEL_MECHANISM),
            "ROGUE_BUFF_SOURCE_TYPE_ENDLESS_LEVEL_START" => ::std::option::Option::Some(DMMJGAAJOJP::ROGUE_BUFF_SOURCE_TYPE_ENDLESS_LEVEL_START),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [DMMJGAAJOJP] = &[
        DMMJGAAJOJP::ROGUE_BUFF_SOURCE_TYPE_NONE,
        DMMJGAAJOJP::ROGUE_BUFF_SOURCE_TYPE_SELECT,
        DMMJGAAJOJP::ROGUE_BUFF_SOURCE_TYPE_ENHANCE,
        DMMJGAAJOJP::ROGUE_BUFF_SOURCE_TYPE_MIRACLE,
        DMMJGAAJOJP::ROGUE_BUFF_SOURCE_TYPE_DIALOGUE,
        DMMJGAAJOJP::ROGUE_BUFF_SOURCE_TYPE_BONUS,
        DMMJGAAJOJP::ROGUE_BUFF_SOURCE_TYPE_MAZE_SKILL,
        DMMJGAAJOJP::ROGUE_BUFF_SOURCE_TYPE_SHOP,
        DMMJGAAJOJP::ROGUE_BUFF_SOURCE_TYPE_LEVEL_MECHANISM,
        DMMJGAAJOJP::ROGUE_BUFF_SOURCE_TYPE_ENDLESS_LEVEL_START,
    ];
}

impl ::protobuf::EnumFull for DMMJGAAJOJP {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("DMMJGAAJOJP").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for DMMJGAAJOJP {
    fn default() -> Self {
        DMMJGAAJOJP::ROGUE_BUFF_SOURCE_TYPE_NONE
    }
}

impl DMMJGAAJOJP {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<DMMJGAAJOJP>("DMMJGAAJOJP")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11DMMJGAAJOJP.proto*\x84\x03\n\x0bDMMJGAAJOJP\x12\x1f\n\x1bROGUE_BUF\
    F_SOURCE_TYPE_NONE\x10\0\x12!\n\x1dROGUE_BUFF_SOURCE_TYPE_SELECT\x10\x01\
    \x12\"\n\x1eROGUE_BUFF_SOURCE_TYPE_ENHANCE\x10\x02\x12\"\n\x1eROGUE_BUFF\
    _SOURCE_TYPE_MIRACLE\x10\x03\x12#\n\x1fROGUE_BUFF_SOURCE_TYPE_DIALOGUE\
    \x10\x04\x12\x20\n\x1cROGUE_BUFF_SOURCE_TYPE_BONUS\x10\x05\x12%\n!ROGUE_\
    BUFF_SOURCE_TYPE_MAZE_SKILL\x10\x06\x12\x1f\n\x1bROGUE_BUFF_SOURCE_TYPE_\
    SHOP\x10\x07\x12*\n&ROGUE_BUFF_SOURCE_TYPE_LEVEL_MECHANISM\x10\x08\x12.\
    \n*ROGUE_BUFF_SOURCE_TYPE_ENDLESS_LEVEL_START\x10\tb\x06proto3\
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
            enums.push(DMMJGAAJOJP::generated_enum_descriptor_data());
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
