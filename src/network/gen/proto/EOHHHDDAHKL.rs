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

//! Generated file from `EOHHHDDAHKL.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:EOHHHDDAHKL)
pub enum EOHHHDDAHKL {
    // @@protoc_insertion_point(enum_value:EOHHHDDAHKL.ROGUE_MIRACLE_SOURCE_TYPE_NONE)
    ROGUE_MIRACLE_SOURCE_TYPE_NONE = 0,
    // @@protoc_insertion_point(enum_value:EOHHHDDAHKL.ROGUE_MIRACLE_SOURCE_TYPE_SELECT)
    ROGUE_MIRACLE_SOURCE_TYPE_SELECT = 1,
    // @@protoc_insertion_point(enum_value:EOHHHDDAHKL.ROGUE_MIRACLE_SOURCE_TYPE_DIALOGUE)
    ROGUE_MIRACLE_SOURCE_TYPE_DIALOGUE = 2,
    // @@protoc_insertion_point(enum_value:EOHHHDDAHKL.ROGUE_MIRACLE_SOURCE_TYPE_BONUS)
    ROGUE_MIRACLE_SOURCE_TYPE_BONUS = 3,
    // @@protoc_insertion_point(enum_value:EOHHHDDAHKL.ROGUE_MIRACLE_SOURCE_TYPE_USE)
    ROGUE_MIRACLE_SOURCE_TYPE_USE = 4,
    // @@protoc_insertion_point(enum_value:EOHHHDDAHKL.ROGUE_MIRACLE_SOURCE_TYPE_RESET)
    ROGUE_MIRACLE_SOURCE_TYPE_RESET = 5,
    // @@protoc_insertion_point(enum_value:EOHHHDDAHKL.ROGUE_MIRACLE_SOURCE_TYPE_REPLACE)
    ROGUE_MIRACLE_SOURCE_TYPE_REPLACE = 6,
    // @@protoc_insertion_point(enum_value:EOHHHDDAHKL.ROGUE_MIRACLE_SOURCE_TYPE_TRADE)
    ROGUE_MIRACLE_SOURCE_TYPE_TRADE = 7,
    // @@protoc_insertion_point(enum_value:EOHHHDDAHKL.ROGUE_MIRACLE_SOURCE_TYPE_GET)
    ROGUE_MIRACLE_SOURCE_TYPE_GET = 8,
    // @@protoc_insertion_point(enum_value:EOHHHDDAHKL.ROGUE_MIRACLE_SOURCE_TYPE_SHOP)
    ROGUE_MIRACLE_SOURCE_TYPE_SHOP = 9,
    // @@protoc_insertion_point(enum_value:EOHHHDDAHKL.ROGUE_MIRACLE_SOURCE_TYPE_MAZE_SKILL)
    ROGUE_MIRACLE_SOURCE_TYPE_MAZE_SKILL = 10,
    // @@protoc_insertion_point(enum_value:EOHHHDDAHKL.ROGUE_MIRACLE_SOURCE_TYPE_LEVEL_MECHANISM)
    ROGUE_MIRACLE_SOURCE_TYPE_LEVEL_MECHANISM = 11,
    // @@protoc_insertion_point(enum_value:EOHHHDDAHKL.ROGUE_MIRACLE_SOURCE_TYPE_ENDLESS_LEVEL_START)
    ROGUE_MIRACLE_SOURCE_TYPE_ENDLESS_LEVEL_START = 12,
}

impl ::protobuf::Enum for EOHHHDDAHKL {
    const NAME: &'static str = "EOHHHDDAHKL";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EOHHHDDAHKL> {
        match value {
            0 => ::std::option::Option::Some(EOHHHDDAHKL::ROGUE_MIRACLE_SOURCE_TYPE_NONE),
            1 => ::std::option::Option::Some(EOHHHDDAHKL::ROGUE_MIRACLE_SOURCE_TYPE_SELECT),
            2 => ::std::option::Option::Some(EOHHHDDAHKL::ROGUE_MIRACLE_SOURCE_TYPE_DIALOGUE),
            3 => ::std::option::Option::Some(EOHHHDDAHKL::ROGUE_MIRACLE_SOURCE_TYPE_BONUS),
            4 => ::std::option::Option::Some(EOHHHDDAHKL::ROGUE_MIRACLE_SOURCE_TYPE_USE),
            5 => ::std::option::Option::Some(EOHHHDDAHKL::ROGUE_MIRACLE_SOURCE_TYPE_RESET),
            6 => ::std::option::Option::Some(EOHHHDDAHKL::ROGUE_MIRACLE_SOURCE_TYPE_REPLACE),
            7 => ::std::option::Option::Some(EOHHHDDAHKL::ROGUE_MIRACLE_SOURCE_TYPE_TRADE),
            8 => ::std::option::Option::Some(EOHHHDDAHKL::ROGUE_MIRACLE_SOURCE_TYPE_GET),
            9 => ::std::option::Option::Some(EOHHHDDAHKL::ROGUE_MIRACLE_SOURCE_TYPE_SHOP),
            10 => ::std::option::Option::Some(EOHHHDDAHKL::ROGUE_MIRACLE_SOURCE_TYPE_MAZE_SKILL),
            11 => ::std::option::Option::Some(EOHHHDDAHKL::ROGUE_MIRACLE_SOURCE_TYPE_LEVEL_MECHANISM),
            12 => ::std::option::Option::Some(EOHHHDDAHKL::ROGUE_MIRACLE_SOURCE_TYPE_ENDLESS_LEVEL_START),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<EOHHHDDAHKL> {
        match str {
            "ROGUE_MIRACLE_SOURCE_TYPE_NONE" => ::std::option::Option::Some(EOHHHDDAHKL::ROGUE_MIRACLE_SOURCE_TYPE_NONE),
            "ROGUE_MIRACLE_SOURCE_TYPE_SELECT" => ::std::option::Option::Some(EOHHHDDAHKL::ROGUE_MIRACLE_SOURCE_TYPE_SELECT),
            "ROGUE_MIRACLE_SOURCE_TYPE_DIALOGUE" => ::std::option::Option::Some(EOHHHDDAHKL::ROGUE_MIRACLE_SOURCE_TYPE_DIALOGUE),
            "ROGUE_MIRACLE_SOURCE_TYPE_BONUS" => ::std::option::Option::Some(EOHHHDDAHKL::ROGUE_MIRACLE_SOURCE_TYPE_BONUS),
            "ROGUE_MIRACLE_SOURCE_TYPE_USE" => ::std::option::Option::Some(EOHHHDDAHKL::ROGUE_MIRACLE_SOURCE_TYPE_USE),
            "ROGUE_MIRACLE_SOURCE_TYPE_RESET" => ::std::option::Option::Some(EOHHHDDAHKL::ROGUE_MIRACLE_SOURCE_TYPE_RESET),
            "ROGUE_MIRACLE_SOURCE_TYPE_REPLACE" => ::std::option::Option::Some(EOHHHDDAHKL::ROGUE_MIRACLE_SOURCE_TYPE_REPLACE),
            "ROGUE_MIRACLE_SOURCE_TYPE_TRADE" => ::std::option::Option::Some(EOHHHDDAHKL::ROGUE_MIRACLE_SOURCE_TYPE_TRADE),
            "ROGUE_MIRACLE_SOURCE_TYPE_GET" => ::std::option::Option::Some(EOHHHDDAHKL::ROGUE_MIRACLE_SOURCE_TYPE_GET),
            "ROGUE_MIRACLE_SOURCE_TYPE_SHOP" => ::std::option::Option::Some(EOHHHDDAHKL::ROGUE_MIRACLE_SOURCE_TYPE_SHOP),
            "ROGUE_MIRACLE_SOURCE_TYPE_MAZE_SKILL" => ::std::option::Option::Some(EOHHHDDAHKL::ROGUE_MIRACLE_SOURCE_TYPE_MAZE_SKILL),
            "ROGUE_MIRACLE_SOURCE_TYPE_LEVEL_MECHANISM" => ::std::option::Option::Some(EOHHHDDAHKL::ROGUE_MIRACLE_SOURCE_TYPE_LEVEL_MECHANISM),
            "ROGUE_MIRACLE_SOURCE_TYPE_ENDLESS_LEVEL_START" => ::std::option::Option::Some(EOHHHDDAHKL::ROGUE_MIRACLE_SOURCE_TYPE_ENDLESS_LEVEL_START),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [EOHHHDDAHKL] = &[
        EOHHHDDAHKL::ROGUE_MIRACLE_SOURCE_TYPE_NONE,
        EOHHHDDAHKL::ROGUE_MIRACLE_SOURCE_TYPE_SELECT,
        EOHHHDDAHKL::ROGUE_MIRACLE_SOURCE_TYPE_DIALOGUE,
        EOHHHDDAHKL::ROGUE_MIRACLE_SOURCE_TYPE_BONUS,
        EOHHHDDAHKL::ROGUE_MIRACLE_SOURCE_TYPE_USE,
        EOHHHDDAHKL::ROGUE_MIRACLE_SOURCE_TYPE_RESET,
        EOHHHDDAHKL::ROGUE_MIRACLE_SOURCE_TYPE_REPLACE,
        EOHHHDDAHKL::ROGUE_MIRACLE_SOURCE_TYPE_TRADE,
        EOHHHDDAHKL::ROGUE_MIRACLE_SOURCE_TYPE_GET,
        EOHHHDDAHKL::ROGUE_MIRACLE_SOURCE_TYPE_SHOP,
        EOHHHDDAHKL::ROGUE_MIRACLE_SOURCE_TYPE_MAZE_SKILL,
        EOHHHDDAHKL::ROGUE_MIRACLE_SOURCE_TYPE_LEVEL_MECHANISM,
        EOHHHDDAHKL::ROGUE_MIRACLE_SOURCE_TYPE_ENDLESS_LEVEL_START,
    ];
}

impl ::protobuf::EnumFull for EOHHHDDAHKL {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("EOHHHDDAHKL").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for EOHHHDDAHKL {
    fn default() -> Self {
        EOHHHDDAHKL::ROGUE_MIRACLE_SOURCE_TYPE_NONE
    }
}

impl EOHHHDDAHKL {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<EOHHHDDAHKL>("EOHHHDDAHKL")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11EOHHHDDAHKL.proto*\x8b\x04\n\x0bEOHHHDDAHKL\x12\"\n\x1eROGUE_MIRAC\
    LE_SOURCE_TYPE_NONE\x10\0\x12$\n\x20ROGUE_MIRACLE_SOURCE_TYPE_SELECT\x10\
    \x01\x12&\n\"ROGUE_MIRACLE_SOURCE_TYPE_DIALOGUE\x10\x02\x12#\n\x1fROGUE_\
    MIRACLE_SOURCE_TYPE_BONUS\x10\x03\x12!\n\x1dROGUE_MIRACLE_SOURCE_TYPE_US\
    E\x10\x04\x12#\n\x1fROGUE_MIRACLE_SOURCE_TYPE_RESET\x10\x05\x12%\n!ROGUE\
    _MIRACLE_SOURCE_TYPE_REPLACE\x10\x06\x12#\n\x1fROGUE_MIRACLE_SOURCE_TYPE\
    _TRADE\x10\x07\x12!\n\x1dROGUE_MIRACLE_SOURCE_TYPE_GET\x10\x08\x12\"\n\
    \x1eROGUE_MIRACLE_SOURCE_TYPE_SHOP\x10\t\x12(\n$ROGUE_MIRACLE_SOURCE_TYP\
    E_MAZE_SKILL\x10\n\x12-\n)ROGUE_MIRACLE_SOURCE_TYPE_LEVEL_MECHANISM\x10\
    \x0b\x121\n-ROGUE_MIRACLE_SOURCE_TYPE_ENDLESS_LEVEL_START\x10\x0cb\x06pr\
    oto3\
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
            enums.push(EOHHHDDAHKL::generated_enum_descriptor_data());
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
