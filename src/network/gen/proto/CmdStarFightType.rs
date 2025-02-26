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

//! Generated file from `CmdStarFightType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdStarFightType)
pub enum CmdStarFightType {
    // @@protoc_insertion_point(enum_value:CmdStarFightType.CmdStarFightTypeNone)
    CmdStarFightTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdStarFightType.CmdStartStarFightLevelCsReq)
    CmdStartStarFightLevelCsReq = 7161,
    // @@protoc_insertion_point(enum_value:CmdStarFightType.CmdGetStarFightDataCsReq)
    CmdGetStarFightDataCsReq = 7169,
    // @@protoc_insertion_point(enum_value:CmdStarFightType.CmdStarFightDataChangeNotify)
    CmdStarFightDataChangeNotify = 7165,
    // @@protoc_insertion_point(enum_value:CmdStarFightType.CmdStartStarFightLevelScRsp)
    CmdStartStarFightLevelScRsp = 7166,
    // @@protoc_insertion_point(enum_value:CmdStarFightType.CmdGetStarFightDataScRsp)
    CmdGetStarFightDataScRsp = 7168,
}

impl ::protobuf::Enum for CmdStarFightType {
    const NAME: &'static str = "CmdStarFightType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdStarFightType> {
        match value {
            0 => ::std::option::Option::Some(CmdStarFightType::CmdStarFightTypeNone),
            7161 => ::std::option::Option::Some(CmdStarFightType::CmdStartStarFightLevelCsReq),
            7169 => ::std::option::Option::Some(CmdStarFightType::CmdGetStarFightDataCsReq),
            7165 => ::std::option::Option::Some(CmdStarFightType::CmdStarFightDataChangeNotify),
            7166 => ::std::option::Option::Some(CmdStarFightType::CmdStartStarFightLevelScRsp),
            7168 => ::std::option::Option::Some(CmdStarFightType::CmdGetStarFightDataScRsp),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdStarFightType> {
        match str {
            "CmdStarFightTypeNone" => ::std::option::Option::Some(CmdStarFightType::CmdStarFightTypeNone),
            "CmdStartStarFightLevelCsReq" => ::std::option::Option::Some(CmdStarFightType::CmdStartStarFightLevelCsReq),
            "CmdGetStarFightDataCsReq" => ::std::option::Option::Some(CmdStarFightType::CmdGetStarFightDataCsReq),
            "CmdStarFightDataChangeNotify" => ::std::option::Option::Some(CmdStarFightType::CmdStarFightDataChangeNotify),
            "CmdStartStarFightLevelScRsp" => ::std::option::Option::Some(CmdStarFightType::CmdStartStarFightLevelScRsp),
            "CmdGetStarFightDataScRsp" => ::std::option::Option::Some(CmdStarFightType::CmdGetStarFightDataScRsp),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdStarFightType] = &[
        CmdStarFightType::CmdStarFightTypeNone,
        CmdStarFightType::CmdStartStarFightLevelCsReq,
        CmdStarFightType::CmdGetStarFightDataCsReq,
        CmdStarFightType::CmdStarFightDataChangeNotify,
        CmdStarFightType::CmdStartStarFightLevelScRsp,
        CmdStarFightType::CmdGetStarFightDataScRsp,
    ];
}

impl ::protobuf::EnumFull for CmdStarFightType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdStarFightType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdStarFightType::CmdStarFightTypeNone => 0,
            CmdStarFightType::CmdStartStarFightLevelCsReq => 1,
            CmdStarFightType::CmdGetStarFightDataCsReq => 2,
            CmdStarFightType::CmdStarFightDataChangeNotify => 3,
            CmdStarFightType::CmdStartStarFightLevelScRsp => 4,
            CmdStarFightType::CmdGetStarFightDataScRsp => 5,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdStarFightType {
    fn default() -> Self {
        CmdStarFightType::CmdStarFightTypeNone
    }
}

impl CmdStarFightType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdStarFightType>("CmdStarFightType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x16CmdStarFightType.proto*\xd1\x01\n\x10CmdStarFightType\x12\x18\n\
    \x14CmdStarFightTypeNone\x10\0\x12\x20\n\x1bCmdStartStarFightLevelCsReq\
    \x10\xf97\x12\x1d\n\x18CmdGetStarFightDataCsReq\x10\x818\x12!\n\x1cCmdSt\
    arFightDataChangeNotify\x10\xfd7\x12\x20\n\x1bCmdStartStarFightLevelScRs\
    p\x10\xfe7\x12\x1d\n\x18CmdGetStarFightDataScRsp\x10\x808b\x06proto3\
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
            enums.push(CmdStarFightType::generated_enum_descriptor_data());
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
