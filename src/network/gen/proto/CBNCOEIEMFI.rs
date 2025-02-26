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

//! Generated file from `CBNCOEIEMFI.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CBNCOEIEMFI)
pub enum CBNCOEIEMFI {
    // @@protoc_insertion_point(enum_value:CBNCOEIEMFI.CHESS_ROGUE_AEON_TYPE_NONE)
    CHESS_ROGUE_AEON_TYPE_NONE = 0,
    // @@protoc_insertion_point(enum_value:CBNCOEIEMFI.CHESS_ROGUE_AEON_TYPE_KNIGHT)
    CHESS_ROGUE_AEON_TYPE_KNIGHT = 1,
    // @@protoc_insertion_point(enum_value:CBNCOEIEMFI.CHESS_ROGUE_AEON_TYPE_MEMORY)
    CHESS_ROGUE_AEON_TYPE_MEMORY = 2,
    // @@protoc_insertion_point(enum_value:CBNCOEIEMFI.CHESS_ROGUE_AEON_TYPE_WARLOCK)
    CHESS_ROGUE_AEON_TYPE_WARLOCK = 3,
    // @@protoc_insertion_point(enum_value:CBNCOEIEMFI.CHESS_ROGUE_AEON_TYPE_PRIEST)
    CHESS_ROGUE_AEON_TYPE_PRIEST = 4,
    // @@protoc_insertion_point(enum_value:CBNCOEIEMFI.CHESS_ROGUE_AEON_TYPE_ROGUE)
    CHESS_ROGUE_AEON_TYPE_ROGUE = 5,
    // @@protoc_insertion_point(enum_value:CBNCOEIEMFI.CHESS_ROGUE_AEON_TYPE_WARRIOR)
    CHESS_ROGUE_AEON_TYPE_WARRIOR = 6,
    // @@protoc_insertion_point(enum_value:CBNCOEIEMFI.CHESS_ROGUE_AEON_TYPE_HAPPY)
    CHESS_ROGUE_AEON_TYPE_HAPPY = 7,
    // @@protoc_insertion_point(enum_value:CBNCOEIEMFI.CHESS_ROGUE_AEON_TYPE_BREED)
    CHESS_ROGUE_AEON_TYPE_BREED = 8,
}

impl ::protobuf::Enum for CBNCOEIEMFI {
    const NAME: &'static str = "CBNCOEIEMFI";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CBNCOEIEMFI> {
        match value {
            0 => ::std::option::Option::Some(CBNCOEIEMFI::CHESS_ROGUE_AEON_TYPE_NONE),
            1 => ::std::option::Option::Some(CBNCOEIEMFI::CHESS_ROGUE_AEON_TYPE_KNIGHT),
            2 => ::std::option::Option::Some(CBNCOEIEMFI::CHESS_ROGUE_AEON_TYPE_MEMORY),
            3 => ::std::option::Option::Some(CBNCOEIEMFI::CHESS_ROGUE_AEON_TYPE_WARLOCK),
            4 => ::std::option::Option::Some(CBNCOEIEMFI::CHESS_ROGUE_AEON_TYPE_PRIEST),
            5 => ::std::option::Option::Some(CBNCOEIEMFI::CHESS_ROGUE_AEON_TYPE_ROGUE),
            6 => ::std::option::Option::Some(CBNCOEIEMFI::CHESS_ROGUE_AEON_TYPE_WARRIOR),
            7 => ::std::option::Option::Some(CBNCOEIEMFI::CHESS_ROGUE_AEON_TYPE_HAPPY),
            8 => ::std::option::Option::Some(CBNCOEIEMFI::CHESS_ROGUE_AEON_TYPE_BREED),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CBNCOEIEMFI> {
        match str {
            "CHESS_ROGUE_AEON_TYPE_NONE" => ::std::option::Option::Some(CBNCOEIEMFI::CHESS_ROGUE_AEON_TYPE_NONE),
            "CHESS_ROGUE_AEON_TYPE_KNIGHT" => ::std::option::Option::Some(CBNCOEIEMFI::CHESS_ROGUE_AEON_TYPE_KNIGHT),
            "CHESS_ROGUE_AEON_TYPE_MEMORY" => ::std::option::Option::Some(CBNCOEIEMFI::CHESS_ROGUE_AEON_TYPE_MEMORY),
            "CHESS_ROGUE_AEON_TYPE_WARLOCK" => ::std::option::Option::Some(CBNCOEIEMFI::CHESS_ROGUE_AEON_TYPE_WARLOCK),
            "CHESS_ROGUE_AEON_TYPE_PRIEST" => ::std::option::Option::Some(CBNCOEIEMFI::CHESS_ROGUE_AEON_TYPE_PRIEST),
            "CHESS_ROGUE_AEON_TYPE_ROGUE" => ::std::option::Option::Some(CBNCOEIEMFI::CHESS_ROGUE_AEON_TYPE_ROGUE),
            "CHESS_ROGUE_AEON_TYPE_WARRIOR" => ::std::option::Option::Some(CBNCOEIEMFI::CHESS_ROGUE_AEON_TYPE_WARRIOR),
            "CHESS_ROGUE_AEON_TYPE_HAPPY" => ::std::option::Option::Some(CBNCOEIEMFI::CHESS_ROGUE_AEON_TYPE_HAPPY),
            "CHESS_ROGUE_AEON_TYPE_BREED" => ::std::option::Option::Some(CBNCOEIEMFI::CHESS_ROGUE_AEON_TYPE_BREED),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CBNCOEIEMFI] = &[
        CBNCOEIEMFI::CHESS_ROGUE_AEON_TYPE_NONE,
        CBNCOEIEMFI::CHESS_ROGUE_AEON_TYPE_KNIGHT,
        CBNCOEIEMFI::CHESS_ROGUE_AEON_TYPE_MEMORY,
        CBNCOEIEMFI::CHESS_ROGUE_AEON_TYPE_WARLOCK,
        CBNCOEIEMFI::CHESS_ROGUE_AEON_TYPE_PRIEST,
        CBNCOEIEMFI::CHESS_ROGUE_AEON_TYPE_ROGUE,
        CBNCOEIEMFI::CHESS_ROGUE_AEON_TYPE_WARRIOR,
        CBNCOEIEMFI::CHESS_ROGUE_AEON_TYPE_HAPPY,
        CBNCOEIEMFI::CHESS_ROGUE_AEON_TYPE_BREED,
    ];
}

impl ::protobuf::EnumFull for CBNCOEIEMFI {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CBNCOEIEMFI").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CBNCOEIEMFI {
    fn default() -> Self {
        CBNCOEIEMFI::CHESS_ROGUE_AEON_TYPE_NONE
    }
}

impl CBNCOEIEMFI {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CBNCOEIEMFI>("CBNCOEIEMFI")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11CBNCOEIEMFI.proto*\xbc\x02\n\x0bCBNCOEIEMFI\x12\x1e\n\x1aCHESS_ROG\
    UE_AEON_TYPE_NONE\x10\0\x12\x20\n\x1cCHESS_ROGUE_AEON_TYPE_KNIGHT\x10\
    \x01\x12\x20\n\x1cCHESS_ROGUE_AEON_TYPE_MEMORY\x10\x02\x12!\n\x1dCHESS_R\
    OGUE_AEON_TYPE_WARLOCK\x10\x03\x12\x20\n\x1cCHESS_ROGUE_AEON_TYPE_PRIEST\
    \x10\x04\x12\x1f\n\x1bCHESS_ROGUE_AEON_TYPE_ROGUE\x10\x05\x12!\n\x1dCHES\
    S_ROGUE_AEON_TYPE_WARRIOR\x10\x06\x12\x1f\n\x1bCHESS_ROGUE_AEON_TYPE_HAP\
    PY\x10\x07\x12\x1f\n\x1bCHESS_ROGUE_AEON_TYPE_BREED\x10\x08b\x06proto3\
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
            enums.push(CBNCOEIEMFI::generated_enum_descriptor_data());
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
