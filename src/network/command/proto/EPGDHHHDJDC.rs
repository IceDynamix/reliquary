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

//! Generated file from `EPGDHHHDJDC.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:EPGDHHHDJDC)
pub enum EPGDHHHDJDC {
    // @@protoc_insertion_point(enum_value:EPGDHHHDJDC.STATUS_CLOSE)
    STATUS_CLOSE = 0,
    // @@protoc_insertion_point(enum_value:EPGDHHHDJDC.STATUS_OPEN)
    STATUS_OPEN = 1,
}

impl ::protobuf::Enum for EPGDHHHDJDC {
    const NAME: &'static str = "EPGDHHHDJDC";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EPGDHHHDJDC> {
        match value {
            0 => ::std::option::Option::Some(EPGDHHHDJDC::STATUS_CLOSE),
            1 => ::std::option::Option::Some(EPGDHHHDJDC::STATUS_OPEN),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<EPGDHHHDJDC> {
        match str {
            "STATUS_CLOSE" => ::std::option::Option::Some(EPGDHHHDJDC::STATUS_CLOSE),
            "STATUS_OPEN" => ::std::option::Option::Some(EPGDHHHDJDC::STATUS_OPEN),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [EPGDHHHDJDC] = &[
        EPGDHHHDJDC::STATUS_CLOSE,
        EPGDHHHDJDC::STATUS_OPEN,
    ];
}

impl ::protobuf::EnumFull for EPGDHHHDJDC {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("EPGDHHHDJDC").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for EPGDHHHDJDC {
    fn default() -> Self {
        EPGDHHHDJDC::STATUS_CLOSE
    }
}

impl EPGDHHHDJDC {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<EPGDHHHDJDC>("EPGDHHHDJDC")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11EPGDHHHDJDC.proto*0\n\x0bEPGDHHHDJDC\x12\x10\n\x0cSTATUS_CLOSE\x10\
    \0\x12\x0f\n\x0bSTATUS_OPEN\x10\x01b\x06proto3\
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
            enums.push(EPGDHHHDJDC::generated_enum_descriptor_data());
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
