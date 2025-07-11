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

//! Generated file from `Match3FinishReason.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:Match3FinishReason)
pub enum Match3FinishReason {
    // @@protoc_insertion_point(enum_value:Match3FinishReason.MATCH3_FINISH_REASON_DEFAULT)
    MATCH3_FINISH_REASON_DEFAULT = 0,
    // @@protoc_insertion_point(enum_value:Match3FinishReason.MATCH3_FINISH_REASON_LEAVE)
    MATCH3_FINISH_REASON_LEAVE = 1,
    // @@protoc_insertion_point(enum_value:Match3FinishReason.MATCH3_FINISH_REASON_DIE)
    MATCH3_FINISH_REASON_DIE = 2,
    // @@protoc_insertion_point(enum_value:Match3FinishReason.MATCH3_FINISH_REASON_GAMEEND)
    MATCH3_FINISH_REASON_GAMEEND = 3,
    // @@protoc_insertion_point(enum_value:Match3FinishReason.MATCH3_FINISH_REASON_KICKOUT)
    MATCH3_FINISH_REASON_KICKOUT = 4,
    // @@protoc_insertion_point(enum_value:Match3FinishReason.MATCH3_FINISH_REASON_WIN)
    MATCH3_FINISH_REASON_WIN = 5,
    // @@protoc_insertion_point(enum_value:Match3FinishReason.MATCH3_FINISH_REASON_LOSE)
    MATCH3_FINISH_REASON_LOSE = 6,
    // @@protoc_insertion_point(enum_value:Match3FinishReason.MATCH3_FINISH_REASON_TIE)
    MATCH3_FINISH_REASON_TIE = 7,
}

impl ::protobuf::Enum for Match3FinishReason {
    const NAME: &'static str = "Match3FinishReason";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Match3FinishReason> {
        match value {
            0 => ::std::option::Option::Some(Match3FinishReason::MATCH3_FINISH_REASON_DEFAULT),
            1 => ::std::option::Option::Some(Match3FinishReason::MATCH3_FINISH_REASON_LEAVE),
            2 => ::std::option::Option::Some(Match3FinishReason::MATCH3_FINISH_REASON_DIE),
            3 => ::std::option::Option::Some(Match3FinishReason::MATCH3_FINISH_REASON_GAMEEND),
            4 => ::std::option::Option::Some(Match3FinishReason::MATCH3_FINISH_REASON_KICKOUT),
            5 => ::std::option::Option::Some(Match3FinishReason::MATCH3_FINISH_REASON_WIN),
            6 => ::std::option::Option::Some(Match3FinishReason::MATCH3_FINISH_REASON_LOSE),
            7 => ::std::option::Option::Some(Match3FinishReason::MATCH3_FINISH_REASON_TIE),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<Match3FinishReason> {
        match str {
            "MATCH3_FINISH_REASON_DEFAULT" => ::std::option::Option::Some(Match3FinishReason::MATCH3_FINISH_REASON_DEFAULT),
            "MATCH3_FINISH_REASON_LEAVE" => ::std::option::Option::Some(Match3FinishReason::MATCH3_FINISH_REASON_LEAVE),
            "MATCH3_FINISH_REASON_DIE" => ::std::option::Option::Some(Match3FinishReason::MATCH3_FINISH_REASON_DIE),
            "MATCH3_FINISH_REASON_GAMEEND" => ::std::option::Option::Some(Match3FinishReason::MATCH3_FINISH_REASON_GAMEEND),
            "MATCH3_FINISH_REASON_KICKOUT" => ::std::option::Option::Some(Match3FinishReason::MATCH3_FINISH_REASON_KICKOUT),
            "MATCH3_FINISH_REASON_WIN" => ::std::option::Option::Some(Match3FinishReason::MATCH3_FINISH_REASON_WIN),
            "MATCH3_FINISH_REASON_LOSE" => ::std::option::Option::Some(Match3FinishReason::MATCH3_FINISH_REASON_LOSE),
            "MATCH3_FINISH_REASON_TIE" => ::std::option::Option::Some(Match3FinishReason::MATCH3_FINISH_REASON_TIE),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [Match3FinishReason] = &[
        Match3FinishReason::MATCH3_FINISH_REASON_DEFAULT,
        Match3FinishReason::MATCH3_FINISH_REASON_LEAVE,
        Match3FinishReason::MATCH3_FINISH_REASON_DIE,
        Match3FinishReason::MATCH3_FINISH_REASON_GAMEEND,
        Match3FinishReason::MATCH3_FINISH_REASON_KICKOUT,
        Match3FinishReason::MATCH3_FINISH_REASON_WIN,
        Match3FinishReason::MATCH3_FINISH_REASON_LOSE,
        Match3FinishReason::MATCH3_FINISH_REASON_TIE,
    ];
}

impl ::protobuf::EnumFull for Match3FinishReason {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("Match3FinishReason").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for Match3FinishReason {
    fn default() -> Self {
        Match3FinishReason::MATCH3_FINISH_REASON_DEFAULT
    }
}

impl Match3FinishReason {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<Match3FinishReason>("Match3FinishReason")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18Match3FinishReason.proto*\x93\x02\n\x12Match3FinishReason\x12\x20\
    \n\x1cMATCH3_FINISH_REASON_DEFAULT\x10\0\x12\x1e\n\x1aMATCH3_FINISH_REAS\
    ON_LEAVE\x10\x01\x12\x1c\n\x18MATCH3_FINISH_REASON_DIE\x10\x02\x12\x20\n\
    \x1cMATCH3_FINISH_REASON_GAMEEND\x10\x03\x12\x20\n\x1cMATCH3_FINISH_REAS\
    ON_KICKOUT\x10\x04\x12\x1c\n\x18MATCH3_FINISH_REASON_WIN\x10\x05\x12\x1d\
    \n\x19MATCH3_FINISH_REASON_LOSE\x10\x06\x12\x1c\n\x18MATCH3_FINISH_REASO\
    N_TIE\x10\x07b\x06proto3\
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
            enums.push(Match3FinishReason::generated_enum_descriptor_data());
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
