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

//! Generated file from `LJGIADHJLHP.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:LJGIADHJLHP)
pub enum LJGIADHJLHP {
    // @@protoc_insertion_point(enum_value:LJGIADHJLHP.EVOLVE_BATTLE_RESULT_NONE)
    EVOLVE_BATTLE_RESULT_NONE = 0,
    // @@protoc_insertion_point(enum_value:LJGIADHJLHP.EVOLVE_BATTLE_RESULT_WIN)
    EVOLVE_BATTLE_RESULT_WIN = 1,
    // @@protoc_insertion_point(enum_value:LJGIADHJLHP.EVOLVE_BATTLE_RESULT_ALL_AVATAR_DEAD)
    EVOLVE_BATTLE_RESULT_ALL_AVATAR_DEAD = 2,
    // @@protoc_insertion_point(enum_value:LJGIADHJLHP.EVOLVE_BATTLE_RESULT_NO_DEAD_LINE)
    EVOLVE_BATTLE_RESULT_NO_DEAD_LINE = 3,
    // @@protoc_insertion_point(enum_value:LJGIADHJLHP.EVOLVE_BATTLE_RESULT_QUIT)
    EVOLVE_BATTLE_RESULT_QUIT = 4,
}

impl ::protobuf::Enum for LJGIADHJLHP {
    const NAME: &'static str = "LJGIADHJLHP";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<LJGIADHJLHP> {
        match value {
            0 => ::std::option::Option::Some(LJGIADHJLHP::EVOLVE_BATTLE_RESULT_NONE),
            1 => ::std::option::Option::Some(LJGIADHJLHP::EVOLVE_BATTLE_RESULT_WIN),
            2 => ::std::option::Option::Some(LJGIADHJLHP::EVOLVE_BATTLE_RESULT_ALL_AVATAR_DEAD),
            3 => ::std::option::Option::Some(LJGIADHJLHP::EVOLVE_BATTLE_RESULT_NO_DEAD_LINE),
            4 => ::std::option::Option::Some(LJGIADHJLHP::EVOLVE_BATTLE_RESULT_QUIT),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<LJGIADHJLHP> {
        match str {
            "EVOLVE_BATTLE_RESULT_NONE" => ::std::option::Option::Some(LJGIADHJLHP::EVOLVE_BATTLE_RESULT_NONE),
            "EVOLVE_BATTLE_RESULT_WIN" => ::std::option::Option::Some(LJGIADHJLHP::EVOLVE_BATTLE_RESULT_WIN),
            "EVOLVE_BATTLE_RESULT_ALL_AVATAR_DEAD" => ::std::option::Option::Some(LJGIADHJLHP::EVOLVE_BATTLE_RESULT_ALL_AVATAR_DEAD),
            "EVOLVE_BATTLE_RESULT_NO_DEAD_LINE" => ::std::option::Option::Some(LJGIADHJLHP::EVOLVE_BATTLE_RESULT_NO_DEAD_LINE),
            "EVOLVE_BATTLE_RESULT_QUIT" => ::std::option::Option::Some(LJGIADHJLHP::EVOLVE_BATTLE_RESULT_QUIT),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [LJGIADHJLHP] = &[
        LJGIADHJLHP::EVOLVE_BATTLE_RESULT_NONE,
        LJGIADHJLHP::EVOLVE_BATTLE_RESULT_WIN,
        LJGIADHJLHP::EVOLVE_BATTLE_RESULT_ALL_AVATAR_DEAD,
        LJGIADHJLHP::EVOLVE_BATTLE_RESULT_NO_DEAD_LINE,
        LJGIADHJLHP::EVOLVE_BATTLE_RESULT_QUIT,
    ];
}

impl ::protobuf::EnumFull for LJGIADHJLHP {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("LJGIADHJLHP").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for LJGIADHJLHP {
    fn default() -> Self {
        LJGIADHJLHP::EVOLVE_BATTLE_RESULT_NONE
    }
}

impl LJGIADHJLHP {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<LJGIADHJLHP>("LJGIADHJLHP")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11LJGIADHJLHP.proto*\xba\x01\n\x0bLJGIADHJLHP\x12\x1d\n\x19EVOLVE_BA\
    TTLE_RESULT_NONE\x10\0\x12\x1c\n\x18EVOLVE_BATTLE_RESULT_WIN\x10\x01\x12\
    (\n$EVOLVE_BATTLE_RESULT_ALL_AVATAR_DEAD\x10\x02\x12%\n!EVOLVE_BATTLE_RE\
    SULT_NO_DEAD_LINE\x10\x03\x12\x1d\n\x19EVOLVE_BATTLE_RESULT_QUIT\x10\x04\
    b\x06proto3\
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
            enums.push(LJGIADHJLHP::generated_enum_descriptor_data());
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