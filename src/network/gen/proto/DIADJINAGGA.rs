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

//! Generated file from `DIADJINAGGA.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:DIADJINAGGA)
pub enum DIADJINAGGA {
    // @@protoc_insertion_point(enum_value:DIADJINAGGA.SWORD_TRAIN_GAME_SOURCE_TYPE_NONE)
    SWORD_TRAIN_GAME_SOURCE_TYPE_NONE = 0,
    // @@protoc_insertion_point(enum_value:DIADJINAGGA.SWORD_TRAIN_GAME_SOURCE_TYPE_TURN_SETTLE)
    SWORD_TRAIN_GAME_SOURCE_TYPE_TURN_SETTLE = 1,
    // @@protoc_insertion_point(enum_value:DIADJINAGGA.SWORD_TRAIN_GAME_SOURCE_TYPE_STATUS_UPGRADE)
    SWORD_TRAIN_GAME_SOURCE_TYPE_STATUS_UPGRADE = 2,
    // @@protoc_insertion_point(enum_value:DIADJINAGGA.SWORD_TRAIN_GAME_SOURCE_TYPE_ACTION)
    SWORD_TRAIN_GAME_SOURCE_TYPE_ACTION = 3,
    // @@protoc_insertion_point(enum_value:DIADJINAGGA.SWORD_TRAIN_GAME_SOURCE_TYPE_ACTION_HINT)
    SWORD_TRAIN_GAME_SOURCE_TYPE_ACTION_HINT = 4,
    // @@protoc_insertion_point(enum_value:DIADJINAGGA.SWORD_TRAIN_GAME_SOURCE_TYPE_STORY)
    SWORD_TRAIN_GAME_SOURCE_TYPE_STORY = 5,
    // @@protoc_insertion_point(enum_value:DIADJINAGGA.SWORD_TRAIN_GAME_SOURCE_TYPE_EXAM_BONUS)
    SWORD_TRAIN_GAME_SOURCE_TYPE_EXAM_BONUS = 6,
    // @@protoc_insertion_point(enum_value:DIADJINAGGA.SWORD_TRAIN_GAME_SOURCE_TYPE_DIALOGUE)
    SWORD_TRAIN_GAME_SOURCE_TYPE_DIALOGUE = 7,
}

impl ::protobuf::Enum for DIADJINAGGA {
    const NAME: &'static str = "DIADJINAGGA";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DIADJINAGGA> {
        match value {
            0 => ::std::option::Option::Some(DIADJINAGGA::SWORD_TRAIN_GAME_SOURCE_TYPE_NONE),
            1 => ::std::option::Option::Some(DIADJINAGGA::SWORD_TRAIN_GAME_SOURCE_TYPE_TURN_SETTLE),
            2 => ::std::option::Option::Some(DIADJINAGGA::SWORD_TRAIN_GAME_SOURCE_TYPE_STATUS_UPGRADE),
            3 => ::std::option::Option::Some(DIADJINAGGA::SWORD_TRAIN_GAME_SOURCE_TYPE_ACTION),
            4 => ::std::option::Option::Some(DIADJINAGGA::SWORD_TRAIN_GAME_SOURCE_TYPE_ACTION_HINT),
            5 => ::std::option::Option::Some(DIADJINAGGA::SWORD_TRAIN_GAME_SOURCE_TYPE_STORY),
            6 => ::std::option::Option::Some(DIADJINAGGA::SWORD_TRAIN_GAME_SOURCE_TYPE_EXAM_BONUS),
            7 => ::std::option::Option::Some(DIADJINAGGA::SWORD_TRAIN_GAME_SOURCE_TYPE_DIALOGUE),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<DIADJINAGGA> {
        match str {
            "SWORD_TRAIN_GAME_SOURCE_TYPE_NONE" => ::std::option::Option::Some(DIADJINAGGA::SWORD_TRAIN_GAME_SOURCE_TYPE_NONE),
            "SWORD_TRAIN_GAME_SOURCE_TYPE_TURN_SETTLE" => ::std::option::Option::Some(DIADJINAGGA::SWORD_TRAIN_GAME_SOURCE_TYPE_TURN_SETTLE),
            "SWORD_TRAIN_GAME_SOURCE_TYPE_STATUS_UPGRADE" => ::std::option::Option::Some(DIADJINAGGA::SWORD_TRAIN_GAME_SOURCE_TYPE_STATUS_UPGRADE),
            "SWORD_TRAIN_GAME_SOURCE_TYPE_ACTION" => ::std::option::Option::Some(DIADJINAGGA::SWORD_TRAIN_GAME_SOURCE_TYPE_ACTION),
            "SWORD_TRAIN_GAME_SOURCE_TYPE_ACTION_HINT" => ::std::option::Option::Some(DIADJINAGGA::SWORD_TRAIN_GAME_SOURCE_TYPE_ACTION_HINT),
            "SWORD_TRAIN_GAME_SOURCE_TYPE_STORY" => ::std::option::Option::Some(DIADJINAGGA::SWORD_TRAIN_GAME_SOURCE_TYPE_STORY),
            "SWORD_TRAIN_GAME_SOURCE_TYPE_EXAM_BONUS" => ::std::option::Option::Some(DIADJINAGGA::SWORD_TRAIN_GAME_SOURCE_TYPE_EXAM_BONUS),
            "SWORD_TRAIN_GAME_SOURCE_TYPE_DIALOGUE" => ::std::option::Option::Some(DIADJINAGGA::SWORD_TRAIN_GAME_SOURCE_TYPE_DIALOGUE),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [DIADJINAGGA] = &[
        DIADJINAGGA::SWORD_TRAIN_GAME_SOURCE_TYPE_NONE,
        DIADJINAGGA::SWORD_TRAIN_GAME_SOURCE_TYPE_TURN_SETTLE,
        DIADJINAGGA::SWORD_TRAIN_GAME_SOURCE_TYPE_STATUS_UPGRADE,
        DIADJINAGGA::SWORD_TRAIN_GAME_SOURCE_TYPE_ACTION,
        DIADJINAGGA::SWORD_TRAIN_GAME_SOURCE_TYPE_ACTION_HINT,
        DIADJINAGGA::SWORD_TRAIN_GAME_SOURCE_TYPE_STORY,
        DIADJINAGGA::SWORD_TRAIN_GAME_SOURCE_TYPE_EXAM_BONUS,
        DIADJINAGGA::SWORD_TRAIN_GAME_SOURCE_TYPE_DIALOGUE,
    ];
}

impl ::protobuf::EnumFull for DIADJINAGGA {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("DIADJINAGGA").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for DIADJINAGGA {
    fn default() -> Self {
        DIADJINAGGA::SWORD_TRAIN_GAME_SOURCE_TYPE_NONE
    }
}

impl DIADJINAGGA {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<DIADJINAGGA>("DIADJINAGGA")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11DIADJINAGGA.proto*\xea\x02\n\x0bDIADJINAGGA\x12%\n!SWORD_TRAIN_GAM\
    E_SOURCE_TYPE_NONE\x10\0\x12,\n(SWORD_TRAIN_GAME_SOURCE_TYPE_TURN_SETTLE\
    \x10\x01\x12/\n+SWORD_TRAIN_GAME_SOURCE_TYPE_STATUS_UPGRADE\x10\x02\x12'\
    \n#SWORD_TRAIN_GAME_SOURCE_TYPE_ACTION\x10\x03\x12,\n(SWORD_TRAIN_GAME_S\
    OURCE_TYPE_ACTION_HINT\x10\x04\x12&\n\"SWORD_TRAIN_GAME_SOURCE_TYPE_STOR\
    Y\x10\x05\x12+\n'SWORD_TRAIN_GAME_SOURCE_TYPE_EXAM_BONUS\x10\x06\x12)\n%\
    SWORD_TRAIN_GAME_SOURCE_TYPE_DIALOGUE\x10\x07b\x06proto3\
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
            enums.push(DIADJINAGGA::generated_enum_descriptor_data());
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
