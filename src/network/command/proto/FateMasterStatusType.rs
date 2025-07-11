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

//! Generated file from `FateMasterStatusType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:FateMasterStatusType)
pub enum FateMasterStatusType {
    // @@protoc_insertion_point(enum_value:FateMasterStatusType.FATE_MASTER_STATUS_TYPE_NONE)
    FATE_MASTER_STATUS_TYPE_NONE = 0,
    // @@protoc_insertion_point(enum_value:FateMasterStatusType.FATE_MASTER_STATUS_TYPE_ALIVE)
    FATE_MASTER_STATUS_TYPE_ALIVE = 1,
    // @@protoc_insertion_point(enum_value:FateMasterStatusType.FATE_MASTER_STATUS_TYPE_DEAD)
    FATE_MASTER_STATUS_TYPE_DEAD = 2,
    // @@protoc_insertion_point(enum_value:FateMasterStatusType.FATE_MASTER_STATUS_TYPE_BANNED)
    FATE_MASTER_STATUS_TYPE_BANNED = 3,
}

impl ::protobuf::Enum for FateMasterStatusType {
    const NAME: &'static str = "FateMasterStatusType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<FateMasterStatusType> {
        match value {
            0 => ::std::option::Option::Some(FateMasterStatusType::FATE_MASTER_STATUS_TYPE_NONE),
            1 => ::std::option::Option::Some(FateMasterStatusType::FATE_MASTER_STATUS_TYPE_ALIVE),
            2 => ::std::option::Option::Some(FateMasterStatusType::FATE_MASTER_STATUS_TYPE_DEAD),
            3 => ::std::option::Option::Some(FateMasterStatusType::FATE_MASTER_STATUS_TYPE_BANNED),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<FateMasterStatusType> {
        match str {
            "FATE_MASTER_STATUS_TYPE_NONE" => ::std::option::Option::Some(FateMasterStatusType::FATE_MASTER_STATUS_TYPE_NONE),
            "FATE_MASTER_STATUS_TYPE_ALIVE" => ::std::option::Option::Some(FateMasterStatusType::FATE_MASTER_STATUS_TYPE_ALIVE),
            "FATE_MASTER_STATUS_TYPE_DEAD" => ::std::option::Option::Some(FateMasterStatusType::FATE_MASTER_STATUS_TYPE_DEAD),
            "FATE_MASTER_STATUS_TYPE_BANNED" => ::std::option::Option::Some(FateMasterStatusType::FATE_MASTER_STATUS_TYPE_BANNED),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [FateMasterStatusType] = &[
        FateMasterStatusType::FATE_MASTER_STATUS_TYPE_NONE,
        FateMasterStatusType::FATE_MASTER_STATUS_TYPE_ALIVE,
        FateMasterStatusType::FATE_MASTER_STATUS_TYPE_DEAD,
        FateMasterStatusType::FATE_MASTER_STATUS_TYPE_BANNED,
    ];
}

impl ::protobuf::EnumFull for FateMasterStatusType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("FateMasterStatusType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for FateMasterStatusType {
    fn default() -> Self {
        FateMasterStatusType::FATE_MASTER_STATUS_TYPE_NONE
    }
}

impl FateMasterStatusType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<FateMasterStatusType>("FateMasterStatusType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1aFateMasterStatusType.proto*\xa1\x01\n\x14FateMasterStatusType\x12\
    \x20\n\x1cFATE_MASTER_STATUS_TYPE_NONE\x10\0\x12!\n\x1dFATE_MASTER_STATU\
    S_TYPE_ALIVE\x10\x01\x12\x20\n\x1cFATE_MASTER_STATUS_TYPE_DEAD\x10\x02\
    \x12\"\n\x1eFATE_MASTER_STATUS_TYPE_BANNED\x10\x03b\x06proto3\
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
            enums.push(FateMasterStatusType::generated_enum_descriptor_data());
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
