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

//! Generated file from `CmdRedDotType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdRedDotType)
pub enum CmdRedDotType {
    // @@protoc_insertion_point(enum_value:CmdRedDotType.CmdRedDotTypeNone)
    CmdRedDotTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdRedDotType.CmdGetSingleRedDotParamGroupCsReq)
    CmdGetSingleRedDotParamGroupCsReq = 5976,
    // @@protoc_insertion_point(enum_value:CmdRedDotType.CmdUpdateRedDotDataScRsp)
    CmdUpdateRedDotDataScRsp = 5940,
    // @@protoc_insertion_point(enum_value:CmdRedDotType.CmdGetSingleRedDotParamGroupScRsp)
    CmdGetSingleRedDotParamGroupScRsp = 5948,
    // @@protoc_insertion_point(enum_value:CmdRedDotType.CmdGetAllRedDotDataScRsp)
    CmdGetAllRedDotDataScRsp = 5932,
    // @@protoc_insertion_point(enum_value:CmdRedDotType.CmdUpdateRedDotDataCsReq)
    CmdUpdateRedDotDataCsReq = 5931,
    // @@protoc_insertion_point(enum_value:CmdRedDotType.CmdGetAllRedDotDataCsReq)
    CmdGetAllRedDotDataCsReq = 5995,
}

impl ::protobuf::Enum for CmdRedDotType {
    const NAME: &'static str = "CmdRedDotType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdRedDotType> {
        match value {
            0 => ::std::option::Option::Some(CmdRedDotType::CmdRedDotTypeNone),
            5976 => ::std::option::Option::Some(CmdRedDotType::CmdGetSingleRedDotParamGroupCsReq),
            5940 => ::std::option::Option::Some(CmdRedDotType::CmdUpdateRedDotDataScRsp),
            5948 => ::std::option::Option::Some(CmdRedDotType::CmdGetSingleRedDotParamGroupScRsp),
            5932 => ::std::option::Option::Some(CmdRedDotType::CmdGetAllRedDotDataScRsp),
            5931 => ::std::option::Option::Some(CmdRedDotType::CmdUpdateRedDotDataCsReq),
            5995 => ::std::option::Option::Some(CmdRedDotType::CmdGetAllRedDotDataCsReq),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdRedDotType> {
        match str {
            "CmdRedDotTypeNone" => ::std::option::Option::Some(CmdRedDotType::CmdRedDotTypeNone),
            "CmdGetSingleRedDotParamGroupCsReq" => ::std::option::Option::Some(CmdRedDotType::CmdGetSingleRedDotParamGroupCsReq),
            "CmdUpdateRedDotDataScRsp" => ::std::option::Option::Some(CmdRedDotType::CmdUpdateRedDotDataScRsp),
            "CmdGetSingleRedDotParamGroupScRsp" => ::std::option::Option::Some(CmdRedDotType::CmdGetSingleRedDotParamGroupScRsp),
            "CmdGetAllRedDotDataScRsp" => ::std::option::Option::Some(CmdRedDotType::CmdGetAllRedDotDataScRsp),
            "CmdUpdateRedDotDataCsReq" => ::std::option::Option::Some(CmdRedDotType::CmdUpdateRedDotDataCsReq),
            "CmdGetAllRedDotDataCsReq" => ::std::option::Option::Some(CmdRedDotType::CmdGetAllRedDotDataCsReq),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdRedDotType] = &[
        CmdRedDotType::CmdRedDotTypeNone,
        CmdRedDotType::CmdGetSingleRedDotParamGroupCsReq,
        CmdRedDotType::CmdUpdateRedDotDataScRsp,
        CmdRedDotType::CmdGetSingleRedDotParamGroupScRsp,
        CmdRedDotType::CmdGetAllRedDotDataScRsp,
        CmdRedDotType::CmdUpdateRedDotDataCsReq,
        CmdRedDotType::CmdGetAllRedDotDataCsReq,
    ];
}

impl ::protobuf::EnumFull for CmdRedDotType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdRedDotType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdRedDotType::CmdRedDotTypeNone => 0,
            CmdRedDotType::CmdGetSingleRedDotParamGroupCsReq => 1,
            CmdRedDotType::CmdUpdateRedDotDataScRsp => 2,
            CmdRedDotType::CmdGetSingleRedDotParamGroupScRsp => 3,
            CmdRedDotType::CmdGetAllRedDotDataScRsp => 4,
            CmdRedDotType::CmdUpdateRedDotDataCsReq => 5,
            CmdRedDotType::CmdGetAllRedDotDataCsReq => 6,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdRedDotType {
    fn default() -> Self {
        CmdRedDotType::CmdRedDotTypeNone
    }
}

impl CmdRedDotType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdRedDotType>("CmdRedDotType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x13CmdRedDotType.proto*\xf2\x01\n\rCmdRedDotType\x12\x15\n\x11CmdRedD\
    otTypeNone\x10\0\x12&\n!CmdGetSingleRedDotParamGroupCsReq\x10\xd8.\x12\
    \x1d\n\x18CmdUpdateRedDotDataScRsp\x10\xb4.\x12&\n!CmdGetSingleRedDotPar\
    amGroupScRsp\x10\xbc.\x12\x1d\n\x18CmdGetAllRedDotDataScRsp\x10\xac.\x12\
    \x1d\n\x18CmdUpdateRedDotDataCsReq\x10\xab.\x12\x1d\n\x18CmdGetAllRedDot\
    DataCsReq\x10\xeb.b\x06proto3\
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
            enums.push(CmdRedDotType::generated_enum_descriptor_data());
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
