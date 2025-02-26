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

//! Generated file from `CmdExpeditionType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdExpeditionType)
pub enum CmdExpeditionType {
    // @@protoc_insertion_point(enum_value:CmdExpeditionType.CmdExpeditionTypeNone)
    CmdExpeditionTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdExpeditionType.CmdGetExpeditionDataCsReq)
    CmdGetExpeditionDataCsReq = 2501,
    // @@protoc_insertion_point(enum_value:CmdExpeditionType.CmdAcceptMultipleExpeditionCsReq)
    CmdAcceptMultipleExpeditionCsReq = 2580,
    // @@protoc_insertion_point(enum_value:CmdExpeditionType.CmdTakeExpeditionRewardCsReq)
    CmdTakeExpeditionRewardCsReq = 2597,
    // @@protoc_insertion_point(enum_value:CmdExpeditionType.CmdTakeActivityExpeditionRewardScRsp)
    CmdTakeActivityExpeditionRewardScRsp = 2547,
    // @@protoc_insertion_point(enum_value:CmdExpeditionType.CmdTakeExpeditionRewardScRsp)
    CmdTakeExpeditionRewardScRsp = 2576,
    // @@protoc_insertion_point(enum_value:CmdExpeditionType.CmdTakeMultipleActivityExpeditionRewardCsReq)
    CmdTakeMultipleActivityExpeditionRewardCsReq = 2512,
    // @@protoc_insertion_point(enum_value:CmdExpeditionType.CmdTakeMultipleExpeditionRewardCsReq)
    CmdTakeMultipleExpeditionRewardCsReq = 2596,
    // @@protoc_insertion_point(enum_value:CmdExpeditionType.CmdGetExpeditionDataScRsp)
    CmdGetExpeditionDataScRsp = 2568,
    // @@protoc_insertion_point(enum_value:CmdExpeditionType.CmdAcceptActivityExpeditionScRsp)
    CmdAcceptActivityExpeditionScRsp = 2505,
    // @@protoc_insertion_point(enum_value:CmdExpeditionType.CmdCancelActivityExpeditionScRsp)
    CmdCancelActivityExpeditionScRsp = 2549,
    // @@protoc_insertion_point(enum_value:CmdExpeditionType.CmdAcceptExpeditionCsReq)
    CmdAcceptExpeditionCsReq = 2558,
    // @@protoc_insertion_point(enum_value:CmdExpeditionType.CmdCancelExpeditionScRsp)
    CmdCancelExpeditionScRsp = 2556,
    // @@protoc_insertion_point(enum_value:CmdExpeditionType.CmdAcceptExpeditionScRsp)
    CmdAcceptExpeditionScRsp = 2524,
    // @@protoc_insertion_point(enum_value:CmdExpeditionType.CmdTakeMultipleActivityExpeditionRewardScRsp)
    CmdTakeMultipleActivityExpeditionRewardScRsp = 2517,
    // @@protoc_insertion_point(enum_value:CmdExpeditionType.CmdTakeActivityExpeditionRewardCsReq)
    CmdTakeActivityExpeditionRewardCsReq = 2522,
    // @@protoc_insertion_point(enum_value:CmdExpeditionType.CmdAcceptMultipleExpeditionScRsp)
    CmdAcceptMultipleExpeditionScRsp = 2539,
    // @@protoc_insertion_point(enum_value:CmdExpeditionType.CmdExpeditionDataChangeScNotify)
    CmdExpeditionDataChangeScNotify = 2528,
    // @@protoc_insertion_point(enum_value:CmdExpeditionType.CmdCancelExpeditionCsReq)
    CmdCancelExpeditionCsReq = 2530,
    // @@protoc_insertion_point(enum_value:CmdExpeditionType.CmdCancelActivityExpeditionCsReq)
    CmdCancelActivityExpeditionCsReq = 2514,
    // @@protoc_insertion_point(enum_value:CmdExpeditionType.CmdAcceptActivityExpeditionCsReq)
    CmdAcceptActivityExpeditionCsReq = 2511,
    // @@protoc_insertion_point(enum_value:CmdExpeditionType.CmdTakeMultipleExpeditionRewardScRsp)
    CmdTakeMultipleExpeditionRewardScRsp = 2523,
}

impl ::protobuf::Enum for CmdExpeditionType {
    const NAME: &'static str = "CmdExpeditionType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdExpeditionType> {
        match value {
            0 => ::std::option::Option::Some(CmdExpeditionType::CmdExpeditionTypeNone),
            2501 => ::std::option::Option::Some(CmdExpeditionType::CmdGetExpeditionDataCsReq),
            2580 => ::std::option::Option::Some(CmdExpeditionType::CmdAcceptMultipleExpeditionCsReq),
            2597 => ::std::option::Option::Some(CmdExpeditionType::CmdTakeExpeditionRewardCsReq),
            2547 => ::std::option::Option::Some(CmdExpeditionType::CmdTakeActivityExpeditionRewardScRsp),
            2576 => ::std::option::Option::Some(CmdExpeditionType::CmdTakeExpeditionRewardScRsp),
            2512 => ::std::option::Option::Some(CmdExpeditionType::CmdTakeMultipleActivityExpeditionRewardCsReq),
            2596 => ::std::option::Option::Some(CmdExpeditionType::CmdTakeMultipleExpeditionRewardCsReq),
            2568 => ::std::option::Option::Some(CmdExpeditionType::CmdGetExpeditionDataScRsp),
            2505 => ::std::option::Option::Some(CmdExpeditionType::CmdAcceptActivityExpeditionScRsp),
            2549 => ::std::option::Option::Some(CmdExpeditionType::CmdCancelActivityExpeditionScRsp),
            2558 => ::std::option::Option::Some(CmdExpeditionType::CmdAcceptExpeditionCsReq),
            2556 => ::std::option::Option::Some(CmdExpeditionType::CmdCancelExpeditionScRsp),
            2524 => ::std::option::Option::Some(CmdExpeditionType::CmdAcceptExpeditionScRsp),
            2517 => ::std::option::Option::Some(CmdExpeditionType::CmdTakeMultipleActivityExpeditionRewardScRsp),
            2522 => ::std::option::Option::Some(CmdExpeditionType::CmdTakeActivityExpeditionRewardCsReq),
            2539 => ::std::option::Option::Some(CmdExpeditionType::CmdAcceptMultipleExpeditionScRsp),
            2528 => ::std::option::Option::Some(CmdExpeditionType::CmdExpeditionDataChangeScNotify),
            2530 => ::std::option::Option::Some(CmdExpeditionType::CmdCancelExpeditionCsReq),
            2514 => ::std::option::Option::Some(CmdExpeditionType::CmdCancelActivityExpeditionCsReq),
            2511 => ::std::option::Option::Some(CmdExpeditionType::CmdAcceptActivityExpeditionCsReq),
            2523 => ::std::option::Option::Some(CmdExpeditionType::CmdTakeMultipleExpeditionRewardScRsp),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdExpeditionType> {
        match str {
            "CmdExpeditionTypeNone" => ::std::option::Option::Some(CmdExpeditionType::CmdExpeditionTypeNone),
            "CmdGetExpeditionDataCsReq" => ::std::option::Option::Some(CmdExpeditionType::CmdGetExpeditionDataCsReq),
            "CmdAcceptMultipleExpeditionCsReq" => ::std::option::Option::Some(CmdExpeditionType::CmdAcceptMultipleExpeditionCsReq),
            "CmdTakeExpeditionRewardCsReq" => ::std::option::Option::Some(CmdExpeditionType::CmdTakeExpeditionRewardCsReq),
            "CmdTakeActivityExpeditionRewardScRsp" => ::std::option::Option::Some(CmdExpeditionType::CmdTakeActivityExpeditionRewardScRsp),
            "CmdTakeExpeditionRewardScRsp" => ::std::option::Option::Some(CmdExpeditionType::CmdTakeExpeditionRewardScRsp),
            "CmdTakeMultipleActivityExpeditionRewardCsReq" => ::std::option::Option::Some(CmdExpeditionType::CmdTakeMultipleActivityExpeditionRewardCsReq),
            "CmdTakeMultipleExpeditionRewardCsReq" => ::std::option::Option::Some(CmdExpeditionType::CmdTakeMultipleExpeditionRewardCsReq),
            "CmdGetExpeditionDataScRsp" => ::std::option::Option::Some(CmdExpeditionType::CmdGetExpeditionDataScRsp),
            "CmdAcceptActivityExpeditionScRsp" => ::std::option::Option::Some(CmdExpeditionType::CmdAcceptActivityExpeditionScRsp),
            "CmdCancelActivityExpeditionScRsp" => ::std::option::Option::Some(CmdExpeditionType::CmdCancelActivityExpeditionScRsp),
            "CmdAcceptExpeditionCsReq" => ::std::option::Option::Some(CmdExpeditionType::CmdAcceptExpeditionCsReq),
            "CmdCancelExpeditionScRsp" => ::std::option::Option::Some(CmdExpeditionType::CmdCancelExpeditionScRsp),
            "CmdAcceptExpeditionScRsp" => ::std::option::Option::Some(CmdExpeditionType::CmdAcceptExpeditionScRsp),
            "CmdTakeMultipleActivityExpeditionRewardScRsp" => ::std::option::Option::Some(CmdExpeditionType::CmdTakeMultipleActivityExpeditionRewardScRsp),
            "CmdTakeActivityExpeditionRewardCsReq" => ::std::option::Option::Some(CmdExpeditionType::CmdTakeActivityExpeditionRewardCsReq),
            "CmdAcceptMultipleExpeditionScRsp" => ::std::option::Option::Some(CmdExpeditionType::CmdAcceptMultipleExpeditionScRsp),
            "CmdExpeditionDataChangeScNotify" => ::std::option::Option::Some(CmdExpeditionType::CmdExpeditionDataChangeScNotify),
            "CmdCancelExpeditionCsReq" => ::std::option::Option::Some(CmdExpeditionType::CmdCancelExpeditionCsReq),
            "CmdCancelActivityExpeditionCsReq" => ::std::option::Option::Some(CmdExpeditionType::CmdCancelActivityExpeditionCsReq),
            "CmdAcceptActivityExpeditionCsReq" => ::std::option::Option::Some(CmdExpeditionType::CmdAcceptActivityExpeditionCsReq),
            "CmdTakeMultipleExpeditionRewardScRsp" => ::std::option::Option::Some(CmdExpeditionType::CmdTakeMultipleExpeditionRewardScRsp),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdExpeditionType] = &[
        CmdExpeditionType::CmdExpeditionTypeNone,
        CmdExpeditionType::CmdGetExpeditionDataCsReq,
        CmdExpeditionType::CmdAcceptMultipleExpeditionCsReq,
        CmdExpeditionType::CmdTakeExpeditionRewardCsReq,
        CmdExpeditionType::CmdTakeActivityExpeditionRewardScRsp,
        CmdExpeditionType::CmdTakeExpeditionRewardScRsp,
        CmdExpeditionType::CmdTakeMultipleActivityExpeditionRewardCsReq,
        CmdExpeditionType::CmdTakeMultipleExpeditionRewardCsReq,
        CmdExpeditionType::CmdGetExpeditionDataScRsp,
        CmdExpeditionType::CmdAcceptActivityExpeditionScRsp,
        CmdExpeditionType::CmdCancelActivityExpeditionScRsp,
        CmdExpeditionType::CmdAcceptExpeditionCsReq,
        CmdExpeditionType::CmdCancelExpeditionScRsp,
        CmdExpeditionType::CmdAcceptExpeditionScRsp,
        CmdExpeditionType::CmdTakeMultipleActivityExpeditionRewardScRsp,
        CmdExpeditionType::CmdTakeActivityExpeditionRewardCsReq,
        CmdExpeditionType::CmdAcceptMultipleExpeditionScRsp,
        CmdExpeditionType::CmdExpeditionDataChangeScNotify,
        CmdExpeditionType::CmdCancelExpeditionCsReq,
        CmdExpeditionType::CmdCancelActivityExpeditionCsReq,
        CmdExpeditionType::CmdAcceptActivityExpeditionCsReq,
        CmdExpeditionType::CmdTakeMultipleExpeditionRewardScRsp,
    ];
}

impl ::protobuf::EnumFull for CmdExpeditionType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdExpeditionType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdExpeditionType::CmdExpeditionTypeNone => 0,
            CmdExpeditionType::CmdGetExpeditionDataCsReq => 1,
            CmdExpeditionType::CmdAcceptMultipleExpeditionCsReq => 2,
            CmdExpeditionType::CmdTakeExpeditionRewardCsReq => 3,
            CmdExpeditionType::CmdTakeActivityExpeditionRewardScRsp => 4,
            CmdExpeditionType::CmdTakeExpeditionRewardScRsp => 5,
            CmdExpeditionType::CmdTakeMultipleActivityExpeditionRewardCsReq => 6,
            CmdExpeditionType::CmdTakeMultipleExpeditionRewardCsReq => 7,
            CmdExpeditionType::CmdGetExpeditionDataScRsp => 8,
            CmdExpeditionType::CmdAcceptActivityExpeditionScRsp => 9,
            CmdExpeditionType::CmdCancelActivityExpeditionScRsp => 10,
            CmdExpeditionType::CmdAcceptExpeditionCsReq => 11,
            CmdExpeditionType::CmdCancelExpeditionScRsp => 12,
            CmdExpeditionType::CmdAcceptExpeditionScRsp => 13,
            CmdExpeditionType::CmdTakeMultipleActivityExpeditionRewardScRsp => 14,
            CmdExpeditionType::CmdTakeActivityExpeditionRewardCsReq => 15,
            CmdExpeditionType::CmdAcceptMultipleExpeditionScRsp => 16,
            CmdExpeditionType::CmdExpeditionDataChangeScNotify => 17,
            CmdExpeditionType::CmdCancelExpeditionCsReq => 18,
            CmdExpeditionType::CmdCancelActivityExpeditionCsReq => 19,
            CmdExpeditionType::CmdAcceptActivityExpeditionCsReq => 20,
            CmdExpeditionType::CmdTakeMultipleExpeditionRewardScRsp => 21,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdExpeditionType {
    fn default() -> Self {
        CmdExpeditionType::CmdExpeditionTypeNone
    }
}

impl CmdExpeditionType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdExpeditionType>("CmdExpeditionType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17CmdExpeditionType.proto*\xd2\x06\n\x11CmdExpeditionType\x12\x19\n\
    \x15CmdExpeditionTypeNone\x10\0\x12\x1e\n\x19CmdGetExpeditionDataCsReq\
    \x10\xc5\x13\x12%\n\x20CmdAcceptMultipleExpeditionCsReq\x10\x94\x14\x12!\
    \n\x1cCmdTakeExpeditionRewardCsReq\x10\xa5\x14\x12)\n$CmdTakeActivityExp\
    editionRewardScRsp\x10\xf3\x13\x12!\n\x1cCmdTakeExpeditionRewardScRsp\
    \x10\x90\x14\x121\n,CmdTakeMultipleActivityExpeditionRewardCsReq\x10\xd0\
    \x13\x12)\n$CmdTakeMultipleExpeditionRewardCsReq\x10\xa4\x14\x12\x1e\n\
    \x19CmdGetExpeditionDataScRsp\x10\x88\x14\x12%\n\x20CmdAcceptActivityExp\
    editionScRsp\x10\xc9\x13\x12%\n\x20CmdCancelActivityExpeditionScRsp\x10\
    \xf5\x13\x12\x1d\n\x18CmdAcceptExpeditionCsReq\x10\xfe\x13\x12\x1d\n\x18\
    CmdCancelExpeditionScRsp\x10\xfc\x13\x12\x1d\n\x18CmdAcceptExpeditionScR\
    sp\x10\xdc\x13\x121\n,CmdTakeMultipleActivityExpeditionRewardScRsp\x10\
    \xd5\x13\x12)\n$CmdTakeActivityExpeditionRewardCsReq\x10\xda\x13\x12%\n\
    \x20CmdAcceptMultipleExpeditionScRsp\x10\xeb\x13\x12$\n\x1fCmdExpedition\
    DataChangeScNotify\x10\xe0\x13\x12\x1d\n\x18CmdCancelExpeditionCsReq\x10\
    \xe2\x13\x12%\n\x20CmdCancelActivityExpeditionCsReq\x10\xd2\x13\x12%\n\
    \x20CmdAcceptActivityExpeditionCsReq\x10\xcf\x13\x12)\n$CmdTakeMultipleE\
    xpeditionRewardScRsp\x10\xdb\x13b\x06proto3\
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
            enums.push(CmdExpeditionType::generated_enum_descriptor_data());
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
