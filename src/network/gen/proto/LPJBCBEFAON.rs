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

//! Generated file from `LPJBCBEFAON.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:LPJBCBEFAON)
pub enum LPJBCBEFAON {
    // @@protoc_insertion_point(enum_value:LPJBCBEFAON.RAID_STATUS_NONE)
    RAID_STATUS_NONE = 0,
    // @@protoc_insertion_point(enum_value:LPJBCBEFAON.RAID_STATUS_DOING)
    RAID_STATUS_DOING = 1,
    // @@protoc_insertion_point(enum_value:LPJBCBEFAON.RAID_STATUS_FINISH)
    RAID_STATUS_FINISH = 2,
    // @@protoc_insertion_point(enum_value:LPJBCBEFAON.RAID_STATUS_FAILED)
    RAID_STATUS_FAILED = 3,
}

impl ::protobuf::Enum for LPJBCBEFAON {
    const NAME: &'static str = "LPJBCBEFAON";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<LPJBCBEFAON> {
        match value {
            0 => ::std::option::Option::Some(LPJBCBEFAON::RAID_STATUS_NONE),
            1 => ::std::option::Option::Some(LPJBCBEFAON::RAID_STATUS_DOING),
            2 => ::std::option::Option::Some(LPJBCBEFAON::RAID_STATUS_FINISH),
            3 => ::std::option::Option::Some(LPJBCBEFAON::RAID_STATUS_FAILED),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<LPJBCBEFAON> {
        match str {
            "RAID_STATUS_NONE" => ::std::option::Option::Some(LPJBCBEFAON::RAID_STATUS_NONE),
            "RAID_STATUS_DOING" => ::std::option::Option::Some(LPJBCBEFAON::RAID_STATUS_DOING),
            "RAID_STATUS_FINISH" => ::std::option::Option::Some(LPJBCBEFAON::RAID_STATUS_FINISH),
            "RAID_STATUS_FAILED" => ::std::option::Option::Some(LPJBCBEFAON::RAID_STATUS_FAILED),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [LPJBCBEFAON] = &[
        LPJBCBEFAON::RAID_STATUS_NONE,
        LPJBCBEFAON::RAID_STATUS_DOING,
        LPJBCBEFAON::RAID_STATUS_FINISH,
        LPJBCBEFAON::RAID_STATUS_FAILED,
    ];
}

impl ::protobuf::EnumFull for LPJBCBEFAON {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("LPJBCBEFAON").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for LPJBCBEFAON {
    fn default() -> Self {
        LPJBCBEFAON::RAID_STATUS_NONE
    }
}

impl LPJBCBEFAON {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<LPJBCBEFAON>("LPJBCBEFAON")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11LPJBCBEFAON.proto*j\n\x0bLPJBCBEFAON\x12\x14\n\x10RAID_STATUS_NONE\
    \x10\0\x12\x15\n\x11RAID_STATUS_DOING\x10\x01\x12\x16\n\x12RAID_STATUS_F\
    INISH\x10\x02\x12\x16\n\x12RAID_STATUS_FAILED\x10\x03b\x06proto3\
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
            enums.push(LPJBCBEFAON::generated_enum_descriptor_data());
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
