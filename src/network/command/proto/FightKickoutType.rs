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

//! Generated file from `FightKickoutType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:FightKickoutType)
pub enum FightKickoutType {
    // @@protoc_insertion_point(enum_value:FightKickoutType.FIGHT_KICKOUT_UNKNOWN)
    FIGHT_KICKOUT_UNKNOWN = 0,
    // @@protoc_insertion_point(enum_value:FightKickoutType.FIGHT_KICKOUT_BLACK)
    FIGHT_KICKOUT_BLACK = 1,
    // @@protoc_insertion_point(enum_value:FightKickoutType.FIGHT_KICKOUT_BY_GM)
    FIGHT_KICKOUT_BY_GM = 2,
    // @@protoc_insertion_point(enum_value:FightKickoutType.FIGHT_KICKOUT_TIMEOUT)
    FIGHT_KICKOUT_TIMEOUT = 3,
    // @@protoc_insertion_point(enum_value:FightKickoutType.FIGHT_KICKOUT_SESSION_RESET)
    FIGHT_KICKOUT_SESSION_RESET = 4,
}

impl ::protobuf::Enum for FightKickoutType {
    const NAME: &'static str = "FightKickoutType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<FightKickoutType> {
        match value {
            0 => ::std::option::Option::Some(FightKickoutType::FIGHT_KICKOUT_UNKNOWN),
            1 => ::std::option::Option::Some(FightKickoutType::FIGHT_KICKOUT_BLACK),
            2 => ::std::option::Option::Some(FightKickoutType::FIGHT_KICKOUT_BY_GM),
            3 => ::std::option::Option::Some(FightKickoutType::FIGHT_KICKOUT_TIMEOUT),
            4 => ::std::option::Option::Some(FightKickoutType::FIGHT_KICKOUT_SESSION_RESET),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<FightKickoutType> {
        match str {
            "FIGHT_KICKOUT_UNKNOWN" => ::std::option::Option::Some(FightKickoutType::FIGHT_KICKOUT_UNKNOWN),
            "FIGHT_KICKOUT_BLACK" => ::std::option::Option::Some(FightKickoutType::FIGHT_KICKOUT_BLACK),
            "FIGHT_KICKOUT_BY_GM" => ::std::option::Option::Some(FightKickoutType::FIGHT_KICKOUT_BY_GM),
            "FIGHT_KICKOUT_TIMEOUT" => ::std::option::Option::Some(FightKickoutType::FIGHT_KICKOUT_TIMEOUT),
            "FIGHT_KICKOUT_SESSION_RESET" => ::std::option::Option::Some(FightKickoutType::FIGHT_KICKOUT_SESSION_RESET),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [FightKickoutType] = &[
        FightKickoutType::FIGHT_KICKOUT_UNKNOWN,
        FightKickoutType::FIGHT_KICKOUT_BLACK,
        FightKickoutType::FIGHT_KICKOUT_BY_GM,
        FightKickoutType::FIGHT_KICKOUT_TIMEOUT,
        FightKickoutType::FIGHT_KICKOUT_SESSION_RESET,
    ];
}

impl ::protobuf::EnumFull for FightKickoutType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("FightKickoutType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for FightKickoutType {
    fn default() -> Self {
        FightKickoutType::FIGHT_KICKOUT_UNKNOWN
    }
}

impl FightKickoutType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<FightKickoutType>("FightKickoutType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x16FightKickoutType.proto*\x9b\x01\n\x10FightKickoutType\x12\x19\n\
    \x15FIGHT_KICKOUT_UNKNOWN\x10\0\x12\x17\n\x13FIGHT_KICKOUT_BLACK\x10\x01\
    \x12\x17\n\x13FIGHT_KICKOUT_BY_GM\x10\x02\x12\x19\n\x15FIGHT_KICKOUT_TIM\
    EOUT\x10\x03\x12\x1f\n\x1bFIGHT_KICKOUT_SESSION_RESET\x10\x04b\x06proto3\
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
            enums.push(FightKickoutType::generated_enum_descriptor_data());
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
