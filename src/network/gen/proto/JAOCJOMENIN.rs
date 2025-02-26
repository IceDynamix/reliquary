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

//! Generated file from `JAOCJOMENIN.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:JAOCJOMENIN)
pub enum JAOCJOMENIN {
    // @@protoc_insertion_point(enum_value:JAOCJOMENIN.ROGUE_ADVENTURE_ROOM_STATUS_NONE)
    ROGUE_ADVENTURE_ROOM_STATUS_NONE = 0,
    // @@protoc_insertion_point(enum_value:JAOCJOMENIN.ROGUE_ADVENTURE_ROOM_STATUS_PREPARE)
    ROGUE_ADVENTURE_ROOM_STATUS_PREPARE = 1,
    // @@protoc_insertion_point(enum_value:JAOCJOMENIN.ROGUE_ADVENTURE_ROOM_STATUS_STARTED)
    ROGUE_ADVENTURE_ROOM_STATUS_STARTED = 2,
    // @@protoc_insertion_point(enum_value:JAOCJOMENIN.ROGUE_ADVENTURE_ROOM_STATUS_STOPPED)
    ROGUE_ADVENTURE_ROOM_STATUS_STOPPED = 3,
}

impl ::protobuf::Enum for JAOCJOMENIN {
    const NAME: &'static str = "JAOCJOMENIN";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<JAOCJOMENIN> {
        match value {
            0 => ::std::option::Option::Some(JAOCJOMENIN::ROGUE_ADVENTURE_ROOM_STATUS_NONE),
            1 => ::std::option::Option::Some(JAOCJOMENIN::ROGUE_ADVENTURE_ROOM_STATUS_PREPARE),
            2 => ::std::option::Option::Some(JAOCJOMENIN::ROGUE_ADVENTURE_ROOM_STATUS_STARTED),
            3 => ::std::option::Option::Some(JAOCJOMENIN::ROGUE_ADVENTURE_ROOM_STATUS_STOPPED),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<JAOCJOMENIN> {
        match str {
            "ROGUE_ADVENTURE_ROOM_STATUS_NONE" => ::std::option::Option::Some(JAOCJOMENIN::ROGUE_ADVENTURE_ROOM_STATUS_NONE),
            "ROGUE_ADVENTURE_ROOM_STATUS_PREPARE" => ::std::option::Option::Some(JAOCJOMENIN::ROGUE_ADVENTURE_ROOM_STATUS_PREPARE),
            "ROGUE_ADVENTURE_ROOM_STATUS_STARTED" => ::std::option::Option::Some(JAOCJOMENIN::ROGUE_ADVENTURE_ROOM_STATUS_STARTED),
            "ROGUE_ADVENTURE_ROOM_STATUS_STOPPED" => ::std::option::Option::Some(JAOCJOMENIN::ROGUE_ADVENTURE_ROOM_STATUS_STOPPED),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [JAOCJOMENIN] = &[
        JAOCJOMENIN::ROGUE_ADVENTURE_ROOM_STATUS_NONE,
        JAOCJOMENIN::ROGUE_ADVENTURE_ROOM_STATUS_PREPARE,
        JAOCJOMENIN::ROGUE_ADVENTURE_ROOM_STATUS_STARTED,
        JAOCJOMENIN::ROGUE_ADVENTURE_ROOM_STATUS_STOPPED,
    ];
}

impl ::protobuf::EnumFull for JAOCJOMENIN {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("JAOCJOMENIN").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for JAOCJOMENIN {
    fn default() -> Self {
        JAOCJOMENIN::ROGUE_ADVENTURE_ROOM_STATUS_NONE
    }
}

impl JAOCJOMENIN {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<JAOCJOMENIN>("JAOCJOMENIN")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11JAOCJOMENIN.proto*\xae\x01\n\x0bJAOCJOMENIN\x12$\n\x20ROGUE_ADVENT\
    URE_ROOM_STATUS_NONE\x10\0\x12'\n#ROGUE_ADVENTURE_ROOM_STATUS_PREPARE\
    \x10\x01\x12'\n#ROGUE_ADVENTURE_ROOM_STATUS_STARTED\x10\x02\x12'\n#ROGUE\
    _ADVENTURE_ROOM_STATUS_STOPPED\x10\x03b\x06proto3\
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
            enums.push(JAOCJOMENIN::generated_enum_descriptor_data());
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
