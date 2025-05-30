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

//! Generated file from `DGFCBOFAOIA.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:DGFCBOFAOIA)
pub enum DGFCBOFAOIA {
    // @@protoc_insertion_point(enum_value:DGFCBOFAOIA.MATCH3_STATE_IDLE)
    MATCH3_STATE_IDLE = 0,
    // @@protoc_insertion_point(enum_value:DGFCBOFAOIA.MATCH3_STATE_START)
    MATCH3_STATE_START = 1,
    // @@protoc_insertion_point(enum_value:DGFCBOFAOIA.MATCH3_STATE_MATCH)
    MATCH3_STATE_MATCH = 2,
    // @@protoc_insertion_point(enum_value:DGFCBOFAOIA.MATCH3_STATE_GAME)
    MATCH3_STATE_GAME = 3,
    // @@protoc_insertion_point(enum_value:DGFCBOFAOIA.MATCH3_STATE_HALFTIME)
    MATCH3_STATE_HALFTIME = 4,
    // @@protoc_insertion_point(enum_value:DGFCBOFAOIA.MATCH3_STATE_OVER)
    MATCH3_STATE_OVER = 5,
}

impl ::protobuf::Enum for DGFCBOFAOIA {
    const NAME: &'static str = "DGFCBOFAOIA";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DGFCBOFAOIA> {
        match value {
            0 => ::std::option::Option::Some(DGFCBOFAOIA::MATCH3_STATE_IDLE),
            1 => ::std::option::Option::Some(DGFCBOFAOIA::MATCH3_STATE_START),
            2 => ::std::option::Option::Some(DGFCBOFAOIA::MATCH3_STATE_MATCH),
            3 => ::std::option::Option::Some(DGFCBOFAOIA::MATCH3_STATE_GAME),
            4 => ::std::option::Option::Some(DGFCBOFAOIA::MATCH3_STATE_HALFTIME),
            5 => ::std::option::Option::Some(DGFCBOFAOIA::MATCH3_STATE_OVER),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<DGFCBOFAOIA> {
        match str {
            "MATCH3_STATE_IDLE" => ::std::option::Option::Some(DGFCBOFAOIA::MATCH3_STATE_IDLE),
            "MATCH3_STATE_START" => ::std::option::Option::Some(DGFCBOFAOIA::MATCH3_STATE_START),
            "MATCH3_STATE_MATCH" => ::std::option::Option::Some(DGFCBOFAOIA::MATCH3_STATE_MATCH),
            "MATCH3_STATE_GAME" => ::std::option::Option::Some(DGFCBOFAOIA::MATCH3_STATE_GAME),
            "MATCH3_STATE_HALFTIME" => ::std::option::Option::Some(DGFCBOFAOIA::MATCH3_STATE_HALFTIME),
            "MATCH3_STATE_OVER" => ::std::option::Option::Some(DGFCBOFAOIA::MATCH3_STATE_OVER),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [DGFCBOFAOIA] = &[
        DGFCBOFAOIA::MATCH3_STATE_IDLE,
        DGFCBOFAOIA::MATCH3_STATE_START,
        DGFCBOFAOIA::MATCH3_STATE_MATCH,
        DGFCBOFAOIA::MATCH3_STATE_GAME,
        DGFCBOFAOIA::MATCH3_STATE_HALFTIME,
        DGFCBOFAOIA::MATCH3_STATE_OVER,
    ];
}

impl ::protobuf::EnumFull for DGFCBOFAOIA {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("DGFCBOFAOIA").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for DGFCBOFAOIA {
    fn default() -> Self {
        DGFCBOFAOIA::MATCH3_STATE_IDLE
    }
}

impl DGFCBOFAOIA {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<DGFCBOFAOIA>("DGFCBOFAOIA")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11DGFCBOFAOIA.proto*\x9d\x01\n\x0bDGFCBOFAOIA\x12\x15\n\x11MATCH3_ST\
    ATE_IDLE\x10\0\x12\x16\n\x12MATCH3_STATE_START\x10\x01\x12\x16\n\x12MATC\
    H3_STATE_MATCH\x10\x02\x12\x15\n\x11MATCH3_STATE_GAME\x10\x03\x12\x19\n\
    \x15MATCH3_STATE_HALFTIME\x10\x04\x12\x15\n\x11MATCH3_STATE_OVER\x10\x05\
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
            enums.push(DGFCBOFAOIA::generated_enum_descriptor_data());
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
