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

//! Generated file from `IHPJCPDHOBK.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:IHPJCPDHOBK)
pub enum IHPJCPDHOBK {
    // @@protoc_insertion_point(enum_value:IHPJCPDHOBK.MISSION_SYNC_RECORD_NONE)
    MISSION_SYNC_RECORD_NONE = 0,
    // @@protoc_insertion_point(enum_value:IHPJCPDHOBK.MISSION_SYNC_RECORD_MAIN_MISSION_ACCEPT)
    MISSION_SYNC_RECORD_MAIN_MISSION_ACCEPT = 1,
    // @@protoc_insertion_point(enum_value:IHPJCPDHOBK.MISSION_SYNC_RECORD_MAIN_MISSION_START)
    MISSION_SYNC_RECORD_MAIN_MISSION_START = 2,
    // @@protoc_insertion_point(enum_value:IHPJCPDHOBK.MISSION_SYNC_RECORD_MAIN_MISSION_FINISH)
    MISSION_SYNC_RECORD_MAIN_MISSION_FINISH = 3,
    // @@protoc_insertion_point(enum_value:IHPJCPDHOBK.MISSION_SYNC_RECORD_MAIN_MISSION_DELETE)
    MISSION_SYNC_RECORD_MAIN_MISSION_DELETE = 4,
    // @@protoc_insertion_point(enum_value:IHPJCPDHOBK.MISSION_SYNC_RECORD_MISSION_ACCEPT)
    MISSION_SYNC_RECORD_MISSION_ACCEPT = 11,
    // @@protoc_insertion_point(enum_value:IHPJCPDHOBK.MISSION_SYNC_RECORD_MISSION_START)
    MISSION_SYNC_RECORD_MISSION_START = 12,
    // @@protoc_insertion_point(enum_value:IHPJCPDHOBK.MISSION_SYNC_RECORD_MISSION_FINISH)
    MISSION_SYNC_RECORD_MISSION_FINISH = 13,
    // @@protoc_insertion_point(enum_value:IHPJCPDHOBK.MISSION_SYNC_RECORD_MISSION_DELETE)
    MISSION_SYNC_RECORD_MISSION_DELETE = 14,
    // @@protoc_insertion_point(enum_value:IHPJCPDHOBK.MISSION_SYNC_RECORD_MISSION_PROGRESS)
    MISSION_SYNC_RECORD_MISSION_PROGRESS = 15,
}

impl ::protobuf::Enum for IHPJCPDHOBK {
    const NAME: &'static str = "IHPJCPDHOBK";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<IHPJCPDHOBK> {
        match value {
            0 => ::std::option::Option::Some(IHPJCPDHOBK::MISSION_SYNC_RECORD_NONE),
            1 => ::std::option::Option::Some(IHPJCPDHOBK::MISSION_SYNC_RECORD_MAIN_MISSION_ACCEPT),
            2 => ::std::option::Option::Some(IHPJCPDHOBK::MISSION_SYNC_RECORD_MAIN_MISSION_START),
            3 => ::std::option::Option::Some(IHPJCPDHOBK::MISSION_SYNC_RECORD_MAIN_MISSION_FINISH),
            4 => ::std::option::Option::Some(IHPJCPDHOBK::MISSION_SYNC_RECORD_MAIN_MISSION_DELETE),
            11 => ::std::option::Option::Some(IHPJCPDHOBK::MISSION_SYNC_RECORD_MISSION_ACCEPT),
            12 => ::std::option::Option::Some(IHPJCPDHOBK::MISSION_SYNC_RECORD_MISSION_START),
            13 => ::std::option::Option::Some(IHPJCPDHOBK::MISSION_SYNC_RECORD_MISSION_FINISH),
            14 => ::std::option::Option::Some(IHPJCPDHOBK::MISSION_SYNC_RECORD_MISSION_DELETE),
            15 => ::std::option::Option::Some(IHPJCPDHOBK::MISSION_SYNC_RECORD_MISSION_PROGRESS),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<IHPJCPDHOBK> {
        match str {
            "MISSION_SYNC_RECORD_NONE" => ::std::option::Option::Some(IHPJCPDHOBK::MISSION_SYNC_RECORD_NONE),
            "MISSION_SYNC_RECORD_MAIN_MISSION_ACCEPT" => ::std::option::Option::Some(IHPJCPDHOBK::MISSION_SYNC_RECORD_MAIN_MISSION_ACCEPT),
            "MISSION_SYNC_RECORD_MAIN_MISSION_START" => ::std::option::Option::Some(IHPJCPDHOBK::MISSION_SYNC_RECORD_MAIN_MISSION_START),
            "MISSION_SYNC_RECORD_MAIN_MISSION_FINISH" => ::std::option::Option::Some(IHPJCPDHOBK::MISSION_SYNC_RECORD_MAIN_MISSION_FINISH),
            "MISSION_SYNC_RECORD_MAIN_MISSION_DELETE" => ::std::option::Option::Some(IHPJCPDHOBK::MISSION_SYNC_RECORD_MAIN_MISSION_DELETE),
            "MISSION_SYNC_RECORD_MISSION_ACCEPT" => ::std::option::Option::Some(IHPJCPDHOBK::MISSION_SYNC_RECORD_MISSION_ACCEPT),
            "MISSION_SYNC_RECORD_MISSION_START" => ::std::option::Option::Some(IHPJCPDHOBK::MISSION_SYNC_RECORD_MISSION_START),
            "MISSION_SYNC_RECORD_MISSION_FINISH" => ::std::option::Option::Some(IHPJCPDHOBK::MISSION_SYNC_RECORD_MISSION_FINISH),
            "MISSION_SYNC_RECORD_MISSION_DELETE" => ::std::option::Option::Some(IHPJCPDHOBK::MISSION_SYNC_RECORD_MISSION_DELETE),
            "MISSION_SYNC_RECORD_MISSION_PROGRESS" => ::std::option::Option::Some(IHPJCPDHOBK::MISSION_SYNC_RECORD_MISSION_PROGRESS),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [IHPJCPDHOBK] = &[
        IHPJCPDHOBK::MISSION_SYNC_RECORD_NONE,
        IHPJCPDHOBK::MISSION_SYNC_RECORD_MAIN_MISSION_ACCEPT,
        IHPJCPDHOBK::MISSION_SYNC_RECORD_MAIN_MISSION_START,
        IHPJCPDHOBK::MISSION_SYNC_RECORD_MAIN_MISSION_FINISH,
        IHPJCPDHOBK::MISSION_SYNC_RECORD_MAIN_MISSION_DELETE,
        IHPJCPDHOBK::MISSION_SYNC_RECORD_MISSION_ACCEPT,
        IHPJCPDHOBK::MISSION_SYNC_RECORD_MISSION_START,
        IHPJCPDHOBK::MISSION_SYNC_RECORD_MISSION_FINISH,
        IHPJCPDHOBK::MISSION_SYNC_RECORD_MISSION_DELETE,
        IHPJCPDHOBK::MISSION_SYNC_RECORD_MISSION_PROGRESS,
    ];
}

impl ::protobuf::EnumFull for IHPJCPDHOBK {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("IHPJCPDHOBK").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            IHPJCPDHOBK::MISSION_SYNC_RECORD_NONE => 0,
            IHPJCPDHOBK::MISSION_SYNC_RECORD_MAIN_MISSION_ACCEPT => 1,
            IHPJCPDHOBK::MISSION_SYNC_RECORD_MAIN_MISSION_START => 2,
            IHPJCPDHOBK::MISSION_SYNC_RECORD_MAIN_MISSION_FINISH => 3,
            IHPJCPDHOBK::MISSION_SYNC_RECORD_MAIN_MISSION_DELETE => 4,
            IHPJCPDHOBK::MISSION_SYNC_RECORD_MISSION_ACCEPT => 5,
            IHPJCPDHOBK::MISSION_SYNC_RECORD_MISSION_START => 6,
            IHPJCPDHOBK::MISSION_SYNC_RECORD_MISSION_FINISH => 7,
            IHPJCPDHOBK::MISSION_SYNC_RECORD_MISSION_DELETE => 8,
            IHPJCPDHOBK::MISSION_SYNC_RECORD_MISSION_PROGRESS => 9,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for IHPJCPDHOBK {
    fn default() -> Self {
        IHPJCPDHOBK::MISSION_SYNC_RECORD_NONE
    }
}

impl IHPJCPDHOBK {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<IHPJCPDHOBK>("IHPJCPDHOBK")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11IHPJCPDHOBK.proto*\xa7\x03\n\x0bIHPJCPDHOBK\x12\x1c\n\x18MISSION_S\
    YNC_RECORD_NONE\x10\0\x12+\n'MISSION_SYNC_RECORD_MAIN_MISSION_ACCEPT\x10\
    \x01\x12*\n&MISSION_SYNC_RECORD_MAIN_MISSION_START\x10\x02\x12+\n'MISSIO\
    N_SYNC_RECORD_MAIN_MISSION_FINISH\x10\x03\x12+\n'MISSION_SYNC_RECORD_MAI\
    N_MISSION_DELETE\x10\x04\x12&\n\"MISSION_SYNC_RECORD_MISSION_ACCEPT\x10\
    \x0b\x12%\n!MISSION_SYNC_RECORD_MISSION_START\x10\x0c\x12&\n\"MISSION_SY\
    NC_RECORD_MISSION_FINISH\x10\r\x12&\n\"MISSION_SYNC_RECORD_MISSION_DELET\
    E\x10\x0e\x12(\n$MISSION_SYNC_RECORD_MISSION_PROGRESS\x10\x0fb\x06proto3\
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
            enums.push(IHPJCPDHOBK::generated_enum_descriptor_data());
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