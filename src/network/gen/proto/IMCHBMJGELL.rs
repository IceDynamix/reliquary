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

//! Generated file from `IMCHBMJGELL.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:IMCHBMJGELL)
pub enum IMCHBMJGELL {
    // @@protoc_insertion_point(enum_value:IMCHBMJGELL.NO_KICK)
    NO_KICK = 0,
    // @@protoc_insertion_point(enum_value:IMCHBMJGELL.FORCE_KICK)
    FORCE_KICK = 1,
    // @@protoc_insertion_point(enum_value:IMCHBMJGELL.IDLE_KICK)
    IDLE_KICK = 2,
    // @@protoc_insertion_point(enum_value:IMCHBMJGELL.SILENCE)
    SILENCE = 3,
}

impl ::protobuf::Enum for IMCHBMJGELL {
    const NAME: &'static str = "IMCHBMJGELL";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<IMCHBMJGELL> {
        match value {
            0 => ::std::option::Option::Some(IMCHBMJGELL::NO_KICK),
            1 => ::std::option::Option::Some(IMCHBMJGELL::FORCE_KICK),
            2 => ::std::option::Option::Some(IMCHBMJGELL::IDLE_KICK),
            3 => ::std::option::Option::Some(IMCHBMJGELL::SILENCE),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<IMCHBMJGELL> {
        match str {
            "NO_KICK" => ::std::option::Option::Some(IMCHBMJGELL::NO_KICK),
            "FORCE_KICK" => ::std::option::Option::Some(IMCHBMJGELL::FORCE_KICK),
            "IDLE_KICK" => ::std::option::Option::Some(IMCHBMJGELL::IDLE_KICK),
            "SILENCE" => ::std::option::Option::Some(IMCHBMJGELL::SILENCE),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [IMCHBMJGELL] = &[
        IMCHBMJGELL::NO_KICK,
        IMCHBMJGELL::FORCE_KICK,
        IMCHBMJGELL::IDLE_KICK,
        IMCHBMJGELL::SILENCE,
    ];
}

impl ::protobuf::EnumFull for IMCHBMJGELL {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("IMCHBMJGELL").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for IMCHBMJGELL {
    fn default() -> Self {
        IMCHBMJGELL::NO_KICK
    }
}

impl IMCHBMJGELL {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<IMCHBMJGELL>("IMCHBMJGELL")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11IMCHBMJGELL.proto*F\n\x0bIMCHBMJGELL\x12\x0b\n\x07NO_KICK\x10\0\
    \x12\x0e\n\nFORCE_KICK\x10\x01\x12\r\n\tIDLE_KICK\x10\x02\x12\x0b\n\x07S\
    ILENCE\x10\x03b\x06proto3\
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
            enums.push(IMCHBMJGELL::generated_enum_descriptor_data());
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
