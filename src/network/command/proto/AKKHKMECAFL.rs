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

//! Generated file from `AKKHKMECAFL.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:AKKHKMECAFL)
pub enum AKKHKMECAFL {
    // @@protoc_insertion_point(enum_value:AKKHKMECAFL.ROGUE_TOURN_SETTLE_REASON_NONE)
    ROGUE_TOURN_SETTLE_REASON_NONE = 0,
    // @@protoc_insertion_point(enum_value:AKKHKMECAFL.ROGUE_TOURN_SETTLE_REASON_WIN)
    ROGUE_TOURN_SETTLE_REASON_WIN = 1,
    // @@protoc_insertion_point(enum_value:AKKHKMECAFL.ROGUE_TOURN_SETTLE_REASON_FAIL)
    ROGUE_TOURN_SETTLE_REASON_FAIL = 2,
    // @@protoc_insertion_point(enum_value:AKKHKMECAFL.ROGUE_TOURN_SETTLE_REASON_INTERRUPT)
    ROGUE_TOURN_SETTLE_REASON_INTERRUPT = 3,
}

impl ::protobuf::Enum for AKKHKMECAFL {
    const NAME: &'static str = "AKKHKMECAFL";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<AKKHKMECAFL> {
        match value {
            0 => ::std::option::Option::Some(AKKHKMECAFL::ROGUE_TOURN_SETTLE_REASON_NONE),
            1 => ::std::option::Option::Some(AKKHKMECAFL::ROGUE_TOURN_SETTLE_REASON_WIN),
            2 => ::std::option::Option::Some(AKKHKMECAFL::ROGUE_TOURN_SETTLE_REASON_FAIL),
            3 => ::std::option::Option::Some(AKKHKMECAFL::ROGUE_TOURN_SETTLE_REASON_INTERRUPT),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<AKKHKMECAFL> {
        match str {
            "ROGUE_TOURN_SETTLE_REASON_NONE" => ::std::option::Option::Some(AKKHKMECAFL::ROGUE_TOURN_SETTLE_REASON_NONE),
            "ROGUE_TOURN_SETTLE_REASON_WIN" => ::std::option::Option::Some(AKKHKMECAFL::ROGUE_TOURN_SETTLE_REASON_WIN),
            "ROGUE_TOURN_SETTLE_REASON_FAIL" => ::std::option::Option::Some(AKKHKMECAFL::ROGUE_TOURN_SETTLE_REASON_FAIL),
            "ROGUE_TOURN_SETTLE_REASON_INTERRUPT" => ::std::option::Option::Some(AKKHKMECAFL::ROGUE_TOURN_SETTLE_REASON_INTERRUPT),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [AKKHKMECAFL] = &[
        AKKHKMECAFL::ROGUE_TOURN_SETTLE_REASON_NONE,
        AKKHKMECAFL::ROGUE_TOURN_SETTLE_REASON_WIN,
        AKKHKMECAFL::ROGUE_TOURN_SETTLE_REASON_FAIL,
        AKKHKMECAFL::ROGUE_TOURN_SETTLE_REASON_INTERRUPT,
    ];
}

impl ::protobuf::EnumFull for AKKHKMECAFL {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("AKKHKMECAFL").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for AKKHKMECAFL {
    fn default() -> Self {
        AKKHKMECAFL::ROGUE_TOURN_SETTLE_REASON_NONE
    }
}

impl AKKHKMECAFL {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<AKKHKMECAFL>("AKKHKMECAFL")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11AKKHKMECAFL.proto*\xa1\x01\n\x0bAKKHKMECAFL\x12\"\n\x1eROGUE_TOURN\
    _SETTLE_REASON_NONE\x10\0\x12!\n\x1dROGUE_TOURN_SETTLE_REASON_WIN\x10\
    \x01\x12\"\n\x1eROGUE_TOURN_SETTLE_REASON_FAIL\x10\x02\x12'\n#ROGUE_TOUR\
    N_SETTLE_REASON_INTERRUPT\x10\x03b\x06proto3\
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
            enums.push(AKKHKMECAFL::generated_enum_descriptor_data());
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
