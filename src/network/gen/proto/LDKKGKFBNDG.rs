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

//! Generated file from `LDKKGKFBNDG.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:LDKKGKFBNDG)
pub enum LDKKGKFBNDG {
    // @@protoc_insertion_point(enum_value:LDKKGKFBNDG.ADD_AVATAR_SRC_NONE)
    ADD_AVATAR_SRC_NONE = 0,
    // @@protoc_insertion_point(enum_value:LDKKGKFBNDG.ADD_AVATAR_SRC_GACHA)
    ADD_AVATAR_SRC_GACHA = 1,
    // @@protoc_insertion_point(enum_value:LDKKGKFBNDG.ADD_AVATAR_SRC_ROGUE)
    ADD_AVATAR_SRC_ROGUE = 2,
}

impl ::protobuf::Enum for LDKKGKFBNDG {
    const NAME: &'static str = "LDKKGKFBNDG";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<LDKKGKFBNDG> {
        match value {
            0 => ::std::option::Option::Some(LDKKGKFBNDG::ADD_AVATAR_SRC_NONE),
            1 => ::std::option::Option::Some(LDKKGKFBNDG::ADD_AVATAR_SRC_GACHA),
            2 => ::std::option::Option::Some(LDKKGKFBNDG::ADD_AVATAR_SRC_ROGUE),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<LDKKGKFBNDG> {
        match str {
            "ADD_AVATAR_SRC_NONE" => ::std::option::Option::Some(LDKKGKFBNDG::ADD_AVATAR_SRC_NONE),
            "ADD_AVATAR_SRC_GACHA" => ::std::option::Option::Some(LDKKGKFBNDG::ADD_AVATAR_SRC_GACHA),
            "ADD_AVATAR_SRC_ROGUE" => ::std::option::Option::Some(LDKKGKFBNDG::ADD_AVATAR_SRC_ROGUE),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [LDKKGKFBNDG] = &[
        LDKKGKFBNDG::ADD_AVATAR_SRC_NONE,
        LDKKGKFBNDG::ADD_AVATAR_SRC_GACHA,
        LDKKGKFBNDG::ADD_AVATAR_SRC_ROGUE,
    ];
}

impl ::protobuf::EnumFull for LDKKGKFBNDG {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("LDKKGKFBNDG").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for LDKKGKFBNDG {
    fn default() -> Self {
        LDKKGKFBNDG::ADD_AVATAR_SRC_NONE
    }
}

impl LDKKGKFBNDG {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<LDKKGKFBNDG>("LDKKGKFBNDG")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11LDKKGKFBNDG.proto*Z\n\x0bLDKKGKFBNDG\x12\x17\n\x13ADD_AVATAR_SRC_N\
    ONE\x10\0\x12\x18\n\x14ADD_AVATAR_SRC_GACHA\x10\x01\x12\x18\n\x14ADD_AVA\
    TAR_SRC_ROGUE\x10\x02b\x06proto3\
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
            enums.push(LDKKGKFBNDG::generated_enum_descriptor_data());
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
