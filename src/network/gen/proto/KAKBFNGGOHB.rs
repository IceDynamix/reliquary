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

//! Generated file from `KAKBFNGGOHB.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:KAKBFNGGOHB)
pub enum KAKBFNGGOHB {
    // @@protoc_insertion_point(enum_value:KAKBFNGGOHB.KICK_SQUEEZED)
    KICK_SQUEEZED = 0,
    // @@protoc_insertion_point(enum_value:KAKBFNGGOHB.KICK_BLACK)
    KICK_BLACK = 1,
    // @@protoc_insertion_point(enum_value:KAKBFNGGOHB.KICK_CHANGE_PWD)
    KICK_CHANGE_PWD = 2,
    // @@protoc_insertion_point(enum_value:KAKBFNGGOHB.KICK_LOGIN_WHITE_TIMEOUT)
    KICK_LOGIN_WHITE_TIMEOUT = 3,
    // @@protoc_insertion_point(enum_value:KAKBFNGGOHB.KICK_ACE_ANTI_CHEATER)
    KICK_ACE_ANTI_CHEATER = 4,
    // @@protoc_insertion_point(enum_value:KAKBFNGGOHB.KICK_BY_GM)
    KICK_BY_GM = 5,
}

impl ::protobuf::Enum for KAKBFNGGOHB {
    const NAME: &'static str = "KAKBFNGGOHB";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<KAKBFNGGOHB> {
        match value {
            0 => ::std::option::Option::Some(KAKBFNGGOHB::KICK_SQUEEZED),
            1 => ::std::option::Option::Some(KAKBFNGGOHB::KICK_BLACK),
            2 => ::std::option::Option::Some(KAKBFNGGOHB::KICK_CHANGE_PWD),
            3 => ::std::option::Option::Some(KAKBFNGGOHB::KICK_LOGIN_WHITE_TIMEOUT),
            4 => ::std::option::Option::Some(KAKBFNGGOHB::KICK_ACE_ANTI_CHEATER),
            5 => ::std::option::Option::Some(KAKBFNGGOHB::KICK_BY_GM),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<KAKBFNGGOHB> {
        match str {
            "KICK_SQUEEZED" => ::std::option::Option::Some(KAKBFNGGOHB::KICK_SQUEEZED),
            "KICK_BLACK" => ::std::option::Option::Some(KAKBFNGGOHB::KICK_BLACK),
            "KICK_CHANGE_PWD" => ::std::option::Option::Some(KAKBFNGGOHB::KICK_CHANGE_PWD),
            "KICK_LOGIN_WHITE_TIMEOUT" => ::std::option::Option::Some(KAKBFNGGOHB::KICK_LOGIN_WHITE_TIMEOUT),
            "KICK_ACE_ANTI_CHEATER" => ::std::option::Option::Some(KAKBFNGGOHB::KICK_ACE_ANTI_CHEATER),
            "KICK_BY_GM" => ::std::option::Option::Some(KAKBFNGGOHB::KICK_BY_GM),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [KAKBFNGGOHB] = &[
        KAKBFNGGOHB::KICK_SQUEEZED,
        KAKBFNGGOHB::KICK_BLACK,
        KAKBFNGGOHB::KICK_CHANGE_PWD,
        KAKBFNGGOHB::KICK_LOGIN_WHITE_TIMEOUT,
        KAKBFNGGOHB::KICK_ACE_ANTI_CHEATER,
        KAKBFNGGOHB::KICK_BY_GM,
    ];
}

impl ::protobuf::EnumFull for KAKBFNGGOHB {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("KAKBFNGGOHB").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for KAKBFNGGOHB {
    fn default() -> Self {
        KAKBFNGGOHB::KICK_SQUEEZED
    }
}

impl KAKBFNGGOHB {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<KAKBFNGGOHB>("KAKBFNGGOHB")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11KAKBFNGGOHB.proto*\x8e\x01\n\x0bKAKBFNGGOHB\x12\x11\n\rKICK_SQUEEZ\
    ED\x10\0\x12\x0e\n\nKICK_BLACK\x10\x01\x12\x13\n\x0fKICK_CHANGE_PWD\x10\
    \x02\x12\x1c\n\x18KICK_LOGIN_WHITE_TIMEOUT\x10\x03\x12\x19\n\x15KICK_ACE\
    _ANTI_CHEATER\x10\x04\x12\x0e\n\nKICK_BY_GM\x10\x05b\x06proto3\
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
            enums.push(KAKBFNGGOHB::generated_enum_descriptor_data());
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
