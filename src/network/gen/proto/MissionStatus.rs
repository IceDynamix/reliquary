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

//! Generated file from `MissionStatus.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:MissionStatus)
pub enum MissionStatus {
    // @@protoc_insertion_point(enum_value:MissionStatus.MISSION_NONE)
    MISSION_NONE = 0,
    // @@protoc_insertion_point(enum_value:MissionStatus.MISSION_DOING)
    MISSION_DOING = 1,
    // @@protoc_insertion_point(enum_value:MissionStatus.MISSION_FINISH)
    MISSION_FINISH = 2,
    // @@protoc_insertion_point(enum_value:MissionStatus.MISSION_PREPARED)
    MISSION_PREPARED = 3,
}

impl ::protobuf::Enum for MissionStatus {
    const NAME: &'static str = "MissionStatus";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<MissionStatus> {
        match value {
            0 => ::std::option::Option::Some(MissionStatus::MISSION_NONE),
            1 => ::std::option::Option::Some(MissionStatus::MISSION_DOING),
            2 => ::std::option::Option::Some(MissionStatus::MISSION_FINISH),
            3 => ::std::option::Option::Some(MissionStatus::MISSION_PREPARED),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<MissionStatus> {
        match str {
            "MISSION_NONE" => ::std::option::Option::Some(MissionStatus::MISSION_NONE),
            "MISSION_DOING" => ::std::option::Option::Some(MissionStatus::MISSION_DOING),
            "MISSION_FINISH" => ::std::option::Option::Some(MissionStatus::MISSION_FINISH),
            "MISSION_PREPARED" => ::std::option::Option::Some(MissionStatus::MISSION_PREPARED),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [MissionStatus] = &[
        MissionStatus::MISSION_NONE,
        MissionStatus::MISSION_DOING,
        MissionStatus::MISSION_FINISH,
        MissionStatus::MISSION_PREPARED,
    ];
}

impl ::protobuf::EnumFull for MissionStatus {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("MissionStatus").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for MissionStatus {
    fn default() -> Self {
        MissionStatus::MISSION_NONE
    }
}

impl MissionStatus {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<MissionStatus>("MissionStatus")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x13MissionStatus.proto*^\n\rMissionStatus\x12\x10\n\x0cMISSION_NONE\
    \x10\0\x12\x11\n\rMISSION_DOING\x10\x01\x12\x12\n\x0eMISSION_FINISH\x10\
    \x02\x12\x14\n\x10MISSION_PREPARED\x10\x03b\x06proto3\
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
            enums.push(MissionStatus::generated_enum_descriptor_data());
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
