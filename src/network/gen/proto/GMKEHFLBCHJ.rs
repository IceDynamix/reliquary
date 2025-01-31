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

//! Generated file from `GMKEHFLBCHJ.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:GMKEHFLBCHJ)
pub enum GMKEHFLBCHJ {
    // @@protoc_insertion_point(enum_value:GMKEHFLBCHJ.LEFT)
    LEFT = 0,
    // @@protoc_insertion_point(enum_value:GMKEHFLBCHJ.RIGHT)
    RIGHT = 1,
    // @@protoc_insertion_point(enum_value:GMKEHFLBCHJ.UP)
    UP = 2,
    // @@protoc_insertion_point(enum_value:GMKEHFLBCHJ.DOWN)
    DOWN = 3,
    // @@protoc_insertion_point(enum_value:GMKEHFLBCHJ.LEFT_UP)
    LEFT_UP = 4,
    // @@protoc_insertion_point(enum_value:GMKEHFLBCHJ.LEFT_DOWN)
    LEFT_DOWN = 5,
    // @@protoc_insertion_point(enum_value:GMKEHFLBCHJ.RIGHT_UP)
    RIGHT_UP = 6,
    // @@protoc_insertion_point(enum_value:GMKEHFLBCHJ.RIGHT_DOWN)
    RIGHT_DOWN = 7,
}

impl ::protobuf::Enum for GMKEHFLBCHJ {
    const NAME: &'static str = "GMKEHFLBCHJ";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<GMKEHFLBCHJ> {
        match value {
            0 => ::std::option::Option::Some(GMKEHFLBCHJ::LEFT),
            1 => ::std::option::Option::Some(GMKEHFLBCHJ::RIGHT),
            2 => ::std::option::Option::Some(GMKEHFLBCHJ::UP),
            3 => ::std::option::Option::Some(GMKEHFLBCHJ::DOWN),
            4 => ::std::option::Option::Some(GMKEHFLBCHJ::LEFT_UP),
            5 => ::std::option::Option::Some(GMKEHFLBCHJ::LEFT_DOWN),
            6 => ::std::option::Option::Some(GMKEHFLBCHJ::RIGHT_UP),
            7 => ::std::option::Option::Some(GMKEHFLBCHJ::RIGHT_DOWN),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<GMKEHFLBCHJ> {
        match str {
            "LEFT" => ::std::option::Option::Some(GMKEHFLBCHJ::LEFT),
            "RIGHT" => ::std::option::Option::Some(GMKEHFLBCHJ::RIGHT),
            "UP" => ::std::option::Option::Some(GMKEHFLBCHJ::UP),
            "DOWN" => ::std::option::Option::Some(GMKEHFLBCHJ::DOWN),
            "LEFT_UP" => ::std::option::Option::Some(GMKEHFLBCHJ::LEFT_UP),
            "LEFT_DOWN" => ::std::option::Option::Some(GMKEHFLBCHJ::LEFT_DOWN),
            "RIGHT_UP" => ::std::option::Option::Some(GMKEHFLBCHJ::RIGHT_UP),
            "RIGHT_DOWN" => ::std::option::Option::Some(GMKEHFLBCHJ::RIGHT_DOWN),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [GMKEHFLBCHJ] = &[
        GMKEHFLBCHJ::LEFT,
        GMKEHFLBCHJ::RIGHT,
        GMKEHFLBCHJ::UP,
        GMKEHFLBCHJ::DOWN,
        GMKEHFLBCHJ::LEFT_UP,
        GMKEHFLBCHJ::LEFT_DOWN,
        GMKEHFLBCHJ::RIGHT_UP,
        GMKEHFLBCHJ::RIGHT_DOWN,
    ];
}

impl ::protobuf::EnumFull for GMKEHFLBCHJ {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("GMKEHFLBCHJ").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for GMKEHFLBCHJ {
    fn default() -> Self {
        GMKEHFLBCHJ::LEFT
    }
}

impl GMKEHFLBCHJ {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<GMKEHFLBCHJ>("GMKEHFLBCHJ")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11GMKEHFLBCHJ.proto*n\n\x0bGMKEHFLBCHJ\x12\x08\n\x04LEFT\x10\0\x12\t\
    \n\x05RIGHT\x10\x01\x12\x06\n\x02UP\x10\x02\x12\x08\n\x04DOWN\x10\x03\
    \x12\x0b\n\x07LEFT_UP\x10\x04\x12\r\n\tLEFT_DOWN\x10\x05\x12\x0c\n\x08RI\
    GHT_UP\x10\x06\x12\x0e\n\nRIGHT_DOWN\x10\x07b\x06proto3\
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
            enums.push(GMKEHFLBCHJ::generated_enum_descriptor_data());
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
