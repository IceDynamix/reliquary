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

//! Generated file from `KFHLBKCCACO.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:KFHLBKCCACO)
pub enum KFHLBKCCACO {
    // @@protoc_insertion_point(enum_value:KFHLBKCCACO.CHESS_ROGUE_ACCOUNT_BY_NONE)
    CHESS_ROGUE_ACCOUNT_BY_NONE = 0,
    // @@protoc_insertion_point(enum_value:KFHLBKCCACO.CHESS_ROGUE_ACCOUNT_BY_NORMAL_FINISH)
    CHESS_ROGUE_ACCOUNT_BY_NORMAL_FINISH = 1,
    // @@protoc_insertion_point(enum_value:KFHLBKCCACO.CHESS_ROGUE_ACCOUNT_BY_NORMAL_QUIT)
    CHESS_ROGUE_ACCOUNT_BY_NORMAL_QUIT = 2,
    // @@protoc_insertion_point(enum_value:KFHLBKCCACO.CHESS_ROGUE_ACCOUNT_BY_DIALOG)
    CHESS_ROGUE_ACCOUNT_BY_DIALOG = 3,
    // @@protoc_insertion_point(enum_value:KFHLBKCCACO.CHESS_ROGUE_ACCOUNT_BY_FAILED)
    CHESS_ROGUE_ACCOUNT_BY_FAILED = 4,
    // @@protoc_insertion_point(enum_value:KFHLBKCCACO.CHESS_ROGUE_ACCOUNT_BY_CUSTOM_OP)
    CHESS_ROGUE_ACCOUNT_BY_CUSTOM_OP = 5,
}

impl ::protobuf::Enum for KFHLBKCCACO {
    const NAME: &'static str = "KFHLBKCCACO";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<KFHLBKCCACO> {
        match value {
            0 => ::std::option::Option::Some(KFHLBKCCACO::CHESS_ROGUE_ACCOUNT_BY_NONE),
            1 => ::std::option::Option::Some(KFHLBKCCACO::CHESS_ROGUE_ACCOUNT_BY_NORMAL_FINISH),
            2 => ::std::option::Option::Some(KFHLBKCCACO::CHESS_ROGUE_ACCOUNT_BY_NORMAL_QUIT),
            3 => ::std::option::Option::Some(KFHLBKCCACO::CHESS_ROGUE_ACCOUNT_BY_DIALOG),
            4 => ::std::option::Option::Some(KFHLBKCCACO::CHESS_ROGUE_ACCOUNT_BY_FAILED),
            5 => ::std::option::Option::Some(KFHLBKCCACO::CHESS_ROGUE_ACCOUNT_BY_CUSTOM_OP),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<KFHLBKCCACO> {
        match str {
            "CHESS_ROGUE_ACCOUNT_BY_NONE" => ::std::option::Option::Some(KFHLBKCCACO::CHESS_ROGUE_ACCOUNT_BY_NONE),
            "CHESS_ROGUE_ACCOUNT_BY_NORMAL_FINISH" => ::std::option::Option::Some(KFHLBKCCACO::CHESS_ROGUE_ACCOUNT_BY_NORMAL_FINISH),
            "CHESS_ROGUE_ACCOUNT_BY_NORMAL_QUIT" => ::std::option::Option::Some(KFHLBKCCACO::CHESS_ROGUE_ACCOUNT_BY_NORMAL_QUIT),
            "CHESS_ROGUE_ACCOUNT_BY_DIALOG" => ::std::option::Option::Some(KFHLBKCCACO::CHESS_ROGUE_ACCOUNT_BY_DIALOG),
            "CHESS_ROGUE_ACCOUNT_BY_FAILED" => ::std::option::Option::Some(KFHLBKCCACO::CHESS_ROGUE_ACCOUNT_BY_FAILED),
            "CHESS_ROGUE_ACCOUNT_BY_CUSTOM_OP" => ::std::option::Option::Some(KFHLBKCCACO::CHESS_ROGUE_ACCOUNT_BY_CUSTOM_OP),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [KFHLBKCCACO] = &[
        KFHLBKCCACO::CHESS_ROGUE_ACCOUNT_BY_NONE,
        KFHLBKCCACO::CHESS_ROGUE_ACCOUNT_BY_NORMAL_FINISH,
        KFHLBKCCACO::CHESS_ROGUE_ACCOUNT_BY_NORMAL_QUIT,
        KFHLBKCCACO::CHESS_ROGUE_ACCOUNT_BY_DIALOG,
        KFHLBKCCACO::CHESS_ROGUE_ACCOUNT_BY_FAILED,
        KFHLBKCCACO::CHESS_ROGUE_ACCOUNT_BY_CUSTOM_OP,
    ];
}

impl ::protobuf::EnumFull for KFHLBKCCACO {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("KFHLBKCCACO").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for KFHLBKCCACO {
    fn default() -> Self {
        KFHLBKCCACO::CHESS_ROGUE_ACCOUNT_BY_NONE
    }
}

impl KFHLBKCCACO {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<KFHLBKCCACO>("KFHLBKCCACO")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11KFHLBKCCACO.proto*\xec\x01\n\x0bKFHLBKCCACO\x12\x1f\n\x1bCHESS_ROG\
    UE_ACCOUNT_BY_NONE\x10\0\x12(\n$CHESS_ROGUE_ACCOUNT_BY_NORMAL_FINISH\x10\
    \x01\x12&\n\"CHESS_ROGUE_ACCOUNT_BY_NORMAL_QUIT\x10\x02\x12!\n\x1dCHESS_\
    ROGUE_ACCOUNT_BY_DIALOG\x10\x03\x12!\n\x1dCHESS_ROGUE_ACCOUNT_BY_FAILED\
    \x10\x04\x12$\n\x20CHESS_ROGUE_ACCOUNT_BY_CUSTOM_OP\x10\x05b\x06proto3\
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
            enums.push(KFHLBKCCACO::generated_enum_descriptor_data());
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
