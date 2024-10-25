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

//! Generated file from `CmdRndOptionType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdRndOptionType)
pub enum CmdRndOptionType {
    // @@protoc_insertion_point(enum_value:CmdRndOptionType.CmdRndOptionTypeNone)
    CmdRndOptionTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdRndOptionType.CmdDailyFirstMeetPamCsReq)
    CmdDailyFirstMeetPamCsReq = 3483,
    // @@protoc_insertion_point(enum_value:CmdRndOptionType.CmdGetRndOptionCsReq)
    CmdGetRndOptionCsReq = 3498,
    // @@protoc_insertion_point(enum_value:CmdRndOptionType.CmdGetRndOptionScRsp)
    CmdGetRndOptionScRsp = 3471,
    // @@protoc_insertion_point(enum_value:CmdRndOptionType.CmdDailyFirstMeetPamScRsp)
    CmdDailyFirstMeetPamScRsp = 3442,
}

impl ::protobuf::Enum for CmdRndOptionType {
    const NAME: &'static str = "CmdRndOptionType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdRndOptionType> {
        match value {
            0 => ::std::option::Option::Some(CmdRndOptionType::CmdRndOptionTypeNone),
            3483 => ::std::option::Option::Some(CmdRndOptionType::CmdDailyFirstMeetPamCsReq),
            3498 => ::std::option::Option::Some(CmdRndOptionType::CmdGetRndOptionCsReq),
            3471 => ::std::option::Option::Some(CmdRndOptionType::CmdGetRndOptionScRsp),
            3442 => ::std::option::Option::Some(CmdRndOptionType::CmdDailyFirstMeetPamScRsp),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdRndOptionType> {
        match str {
            "CmdRndOptionTypeNone" => ::std::option::Option::Some(CmdRndOptionType::CmdRndOptionTypeNone),
            "CmdDailyFirstMeetPamCsReq" => ::std::option::Option::Some(CmdRndOptionType::CmdDailyFirstMeetPamCsReq),
            "CmdGetRndOptionCsReq" => ::std::option::Option::Some(CmdRndOptionType::CmdGetRndOptionCsReq),
            "CmdGetRndOptionScRsp" => ::std::option::Option::Some(CmdRndOptionType::CmdGetRndOptionScRsp),
            "CmdDailyFirstMeetPamScRsp" => ::std::option::Option::Some(CmdRndOptionType::CmdDailyFirstMeetPamScRsp),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdRndOptionType] = &[
        CmdRndOptionType::CmdRndOptionTypeNone,
        CmdRndOptionType::CmdDailyFirstMeetPamCsReq,
        CmdRndOptionType::CmdGetRndOptionCsReq,
        CmdRndOptionType::CmdGetRndOptionScRsp,
        CmdRndOptionType::CmdDailyFirstMeetPamScRsp,
    ];
}

impl ::protobuf::EnumFull for CmdRndOptionType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdRndOptionType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdRndOptionType::CmdRndOptionTypeNone => 0,
            CmdRndOptionType::CmdDailyFirstMeetPamCsReq => 1,
            CmdRndOptionType::CmdGetRndOptionCsReq => 2,
            CmdRndOptionType::CmdGetRndOptionScRsp => 3,
            CmdRndOptionType::CmdDailyFirstMeetPamScRsp => 4,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdRndOptionType {
    fn default() -> Self {
        CmdRndOptionType::CmdRndOptionTypeNone
    }
}

impl CmdRndOptionType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdRndOptionType>("CmdRndOptionType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x16CmdRndOptionType.proto*\xa2\x01\n\x10CmdRndOptionType\x12\x18\n\
    \x14CmdRndOptionTypeNone\x10\0\x12\x1e\n\x19CmdDailyFirstMeetPamCsReq\
    \x10\x9b\x1b\x12\x19\n\x14CmdGetRndOptionCsReq\x10\xaa\x1b\x12\x19\n\x14\
    CmdGetRndOptionScRsp\x10\x8f\x1b\x12\x1e\n\x19CmdDailyFirstMeetPamScRsp\
    \x10\xf2\x1ab\x06proto3\
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
            enums.push(CmdRndOptionType::generated_enum_descriptor_data());
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
