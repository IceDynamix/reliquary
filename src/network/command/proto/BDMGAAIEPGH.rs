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

//! Generated file from `BDMGAAIEPGH.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:BDMGAAIEPGH)
pub enum BDMGAAIEPGH {
    // @@protoc_insertion_point(enum_value:BDMGAAIEPGH.MatchRoomCharacter_None)
    MatchRoomCharacter_None = 0,
    // @@protoc_insertion_point(enum_value:BDMGAAIEPGH.MatchRoomCharacter_Leader)
    MatchRoomCharacter_Leader = 1,
    // @@protoc_insertion_point(enum_value:BDMGAAIEPGH.MatchRoomCharacter_Member)
    MatchRoomCharacter_Member = 2,
    // @@protoc_insertion_point(enum_value:BDMGAAIEPGH.MatchRoomCharacter_Watcher)
    MatchRoomCharacter_Watcher = 3,
}

impl ::protobuf::Enum for BDMGAAIEPGH {
    const NAME: &'static str = "BDMGAAIEPGH";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<BDMGAAIEPGH> {
        match value {
            0 => ::std::option::Option::Some(BDMGAAIEPGH::MatchRoomCharacter_None),
            1 => ::std::option::Option::Some(BDMGAAIEPGH::MatchRoomCharacter_Leader),
            2 => ::std::option::Option::Some(BDMGAAIEPGH::MatchRoomCharacter_Member),
            3 => ::std::option::Option::Some(BDMGAAIEPGH::MatchRoomCharacter_Watcher),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<BDMGAAIEPGH> {
        match str {
            "MatchRoomCharacter_None" => ::std::option::Option::Some(BDMGAAIEPGH::MatchRoomCharacter_None),
            "MatchRoomCharacter_Leader" => ::std::option::Option::Some(BDMGAAIEPGH::MatchRoomCharacter_Leader),
            "MatchRoomCharacter_Member" => ::std::option::Option::Some(BDMGAAIEPGH::MatchRoomCharacter_Member),
            "MatchRoomCharacter_Watcher" => ::std::option::Option::Some(BDMGAAIEPGH::MatchRoomCharacter_Watcher),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [BDMGAAIEPGH] = &[
        BDMGAAIEPGH::MatchRoomCharacter_None,
        BDMGAAIEPGH::MatchRoomCharacter_Leader,
        BDMGAAIEPGH::MatchRoomCharacter_Member,
        BDMGAAIEPGH::MatchRoomCharacter_Watcher,
    ];
}

impl ::protobuf::EnumFull for BDMGAAIEPGH {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("BDMGAAIEPGH").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for BDMGAAIEPGH {
    fn default() -> Self {
        BDMGAAIEPGH::MatchRoomCharacter_None
    }
}

impl BDMGAAIEPGH {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<BDMGAAIEPGH>("BDMGAAIEPGH")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11BDMGAAIEPGH.proto*\x88\x01\n\x0bBDMGAAIEPGH\x12\x1b\n\x17MatchRoom\
    Character_None\x10\0\x12\x1d\n\x19MatchRoomCharacter_Leader\x10\x01\x12\
    \x1d\n\x19MatchRoomCharacter_Member\x10\x02\x12\x1e\n\x1aMatchRoomCharac\
    ter_Watcher\x10\x03b\x06proto3\
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
            enums.push(BDMGAAIEPGH::generated_enum_descriptor_data());
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
