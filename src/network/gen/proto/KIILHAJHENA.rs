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

//! Generated file from `KIILHAJHENA.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:KIILHAJHENA)
pub enum KIILHAJHENA {
    // @@protoc_insertion_point(enum_value:KIILHAJHENA.BATTLE_TARGET_TYPE_NONE)
    BATTLE_TARGET_TYPE_NONE = 0,
    // @@protoc_insertion_point(enum_value:KIILHAJHENA.BATTLE_TARGET_TYPE_SCORE)
    BATTLE_TARGET_TYPE_SCORE = 1,
    // @@protoc_insertion_point(enum_value:KIILHAJHENA.BATTLE_TARGET_TYPE_ACHIEVEMENT)
    BATTLE_TARGET_TYPE_ACHIEVEMENT = 2,
    // @@protoc_insertion_point(enum_value:KIILHAJHENA.BATTLE_TARGET_TYPE_RAID)
    BATTLE_TARGET_TYPE_RAID = 3,
    // @@protoc_insertion_point(enum_value:KIILHAJHENA.BATTLE_TARGET_TYPE_CHALLENGE_SCORE)
    BATTLE_TARGET_TYPE_CHALLENGE_SCORE = 4,
    // @@protoc_insertion_point(enum_value:KIILHAJHENA.BATTLE_TARGET_TYPE_COMMON)
    BATTLE_TARGET_TYPE_COMMON = 5,
    // @@protoc_insertion_point(enum_value:KIILHAJHENA.BATTLE_TARGET_TYPE_CLIENT_ACHIEVEMENT)
    BATTLE_TARGET_TYPE_CLIENT_ACHIEVEMENT = 6,
}

impl ::protobuf::Enum for KIILHAJHENA {
    const NAME: &'static str = "KIILHAJHENA";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<KIILHAJHENA> {
        match value {
            0 => ::std::option::Option::Some(KIILHAJHENA::BATTLE_TARGET_TYPE_NONE),
            1 => ::std::option::Option::Some(KIILHAJHENA::BATTLE_TARGET_TYPE_SCORE),
            2 => ::std::option::Option::Some(KIILHAJHENA::BATTLE_TARGET_TYPE_ACHIEVEMENT),
            3 => ::std::option::Option::Some(KIILHAJHENA::BATTLE_TARGET_TYPE_RAID),
            4 => ::std::option::Option::Some(KIILHAJHENA::BATTLE_TARGET_TYPE_CHALLENGE_SCORE),
            5 => ::std::option::Option::Some(KIILHAJHENA::BATTLE_TARGET_TYPE_COMMON),
            6 => ::std::option::Option::Some(KIILHAJHENA::BATTLE_TARGET_TYPE_CLIENT_ACHIEVEMENT),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<KIILHAJHENA> {
        match str {
            "BATTLE_TARGET_TYPE_NONE" => ::std::option::Option::Some(KIILHAJHENA::BATTLE_TARGET_TYPE_NONE),
            "BATTLE_TARGET_TYPE_SCORE" => ::std::option::Option::Some(KIILHAJHENA::BATTLE_TARGET_TYPE_SCORE),
            "BATTLE_TARGET_TYPE_ACHIEVEMENT" => ::std::option::Option::Some(KIILHAJHENA::BATTLE_TARGET_TYPE_ACHIEVEMENT),
            "BATTLE_TARGET_TYPE_RAID" => ::std::option::Option::Some(KIILHAJHENA::BATTLE_TARGET_TYPE_RAID),
            "BATTLE_TARGET_TYPE_CHALLENGE_SCORE" => ::std::option::Option::Some(KIILHAJHENA::BATTLE_TARGET_TYPE_CHALLENGE_SCORE),
            "BATTLE_TARGET_TYPE_COMMON" => ::std::option::Option::Some(KIILHAJHENA::BATTLE_TARGET_TYPE_COMMON),
            "BATTLE_TARGET_TYPE_CLIENT_ACHIEVEMENT" => ::std::option::Option::Some(KIILHAJHENA::BATTLE_TARGET_TYPE_CLIENT_ACHIEVEMENT),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [KIILHAJHENA] = &[
        KIILHAJHENA::BATTLE_TARGET_TYPE_NONE,
        KIILHAJHENA::BATTLE_TARGET_TYPE_SCORE,
        KIILHAJHENA::BATTLE_TARGET_TYPE_ACHIEVEMENT,
        KIILHAJHENA::BATTLE_TARGET_TYPE_RAID,
        KIILHAJHENA::BATTLE_TARGET_TYPE_CHALLENGE_SCORE,
        KIILHAJHENA::BATTLE_TARGET_TYPE_COMMON,
        KIILHAJHENA::BATTLE_TARGET_TYPE_CLIENT_ACHIEVEMENT,
    ];
}

impl ::protobuf::EnumFull for KIILHAJHENA {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("KIILHAJHENA").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for KIILHAJHENA {
    fn default() -> Self {
        KIILHAJHENA::BATTLE_TARGET_TYPE_NONE
    }
}

impl KIILHAJHENA {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<KIILHAJHENA>("KIILHAJHENA")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11KIILHAJHENA.proto*\xfb\x01\n\x0bKIILHAJHENA\x12\x1b\n\x17BATTLE_TA\
    RGET_TYPE_NONE\x10\0\x12\x1c\n\x18BATTLE_TARGET_TYPE_SCORE\x10\x01\x12\"\
    \n\x1eBATTLE_TARGET_TYPE_ACHIEVEMENT\x10\x02\x12\x1b\n\x17BATTLE_TARGET_\
    TYPE_RAID\x10\x03\x12&\n\"BATTLE_TARGET_TYPE_CHALLENGE_SCORE\x10\x04\x12\
    \x1d\n\x19BATTLE_TARGET_TYPE_COMMON\x10\x05\x12)\n%BATTLE_TARGET_TYPE_CL\
    IENT_ACHIEVEMENT\x10\x06b\x06proto3\
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
            enums.push(KIILHAJHENA::generated_enum_descriptor_data());
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