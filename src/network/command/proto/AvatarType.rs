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

//! Generated file from `AvatarType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:AvatarType)
pub enum AvatarType {
    // @@protoc_insertion_point(enum_value:AvatarType.AVATAR_TYPE_NONE)
    AVATAR_TYPE_NONE = 0,
    // @@protoc_insertion_point(enum_value:AvatarType.AVATAR_TRIAL_TYPE)
    AVATAR_TRIAL_TYPE = 1,
    // @@protoc_insertion_point(enum_value:AvatarType.AVATAR_LIMIT_TYPE)
    AVATAR_LIMIT_TYPE = 2,
    // @@protoc_insertion_point(enum_value:AvatarType.AVATAR_FORMAL_TYPE)
    AVATAR_FORMAL_TYPE = 3,
    // @@protoc_insertion_point(enum_value:AvatarType.AVATAR_ASSIST_TYPE)
    AVATAR_ASSIST_TYPE = 4,
    // @@protoc_insertion_point(enum_value:AvatarType.AVATAR_AETHER_DIVIDE_TYPE)
    AVATAR_AETHER_DIVIDE_TYPE = 5,
    // @@protoc_insertion_point(enum_value:AvatarType.AVATAR_UPGRADE_AVAILABLE_TYPE)
    AVATAR_UPGRADE_AVAILABLE_TYPE = 6,
}

impl ::protobuf::Enum for AvatarType {
    const NAME: &'static str = "AvatarType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<AvatarType> {
        match value {
            0 => ::std::option::Option::Some(AvatarType::AVATAR_TYPE_NONE),
            1 => ::std::option::Option::Some(AvatarType::AVATAR_TRIAL_TYPE),
            2 => ::std::option::Option::Some(AvatarType::AVATAR_LIMIT_TYPE),
            3 => ::std::option::Option::Some(AvatarType::AVATAR_FORMAL_TYPE),
            4 => ::std::option::Option::Some(AvatarType::AVATAR_ASSIST_TYPE),
            5 => ::std::option::Option::Some(AvatarType::AVATAR_AETHER_DIVIDE_TYPE),
            6 => ::std::option::Option::Some(AvatarType::AVATAR_UPGRADE_AVAILABLE_TYPE),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<AvatarType> {
        match str {
            "AVATAR_TYPE_NONE" => ::std::option::Option::Some(AvatarType::AVATAR_TYPE_NONE),
            "AVATAR_TRIAL_TYPE" => ::std::option::Option::Some(AvatarType::AVATAR_TRIAL_TYPE),
            "AVATAR_LIMIT_TYPE" => ::std::option::Option::Some(AvatarType::AVATAR_LIMIT_TYPE),
            "AVATAR_FORMAL_TYPE" => ::std::option::Option::Some(AvatarType::AVATAR_FORMAL_TYPE),
            "AVATAR_ASSIST_TYPE" => ::std::option::Option::Some(AvatarType::AVATAR_ASSIST_TYPE),
            "AVATAR_AETHER_DIVIDE_TYPE" => ::std::option::Option::Some(AvatarType::AVATAR_AETHER_DIVIDE_TYPE),
            "AVATAR_UPGRADE_AVAILABLE_TYPE" => ::std::option::Option::Some(AvatarType::AVATAR_UPGRADE_AVAILABLE_TYPE),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [AvatarType] = &[
        AvatarType::AVATAR_TYPE_NONE,
        AvatarType::AVATAR_TRIAL_TYPE,
        AvatarType::AVATAR_LIMIT_TYPE,
        AvatarType::AVATAR_FORMAL_TYPE,
        AvatarType::AVATAR_ASSIST_TYPE,
        AvatarType::AVATAR_AETHER_DIVIDE_TYPE,
        AvatarType::AVATAR_UPGRADE_AVAILABLE_TYPE,
    ];
}

impl ::protobuf::EnumFull for AvatarType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("AvatarType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for AvatarType {
    fn default() -> Self {
        AvatarType::AVATAR_TYPE_NONE
    }
}

impl AvatarType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<AvatarType>("AvatarType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x10AvatarType.proto*\xc2\x01\n\nAvatarType\x12\x14\n\x10AVATAR_TYPE_N\
    ONE\x10\0\x12\x15\n\x11AVATAR_TRIAL_TYPE\x10\x01\x12\x15\n\x11AVATAR_LIM\
    IT_TYPE\x10\x02\x12\x16\n\x12AVATAR_FORMAL_TYPE\x10\x03\x12\x16\n\x12AVA\
    TAR_ASSIST_TYPE\x10\x04\x12\x1d\n\x19AVATAR_AETHER_DIVIDE_TYPE\x10\x05\
    \x12!\n\x1dAVATAR_UPGRADE_AVAILABLE_TYPE\x10\x06b\x06proto3\
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
            enums.push(AvatarType::generated_enum_descriptor_data());
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
