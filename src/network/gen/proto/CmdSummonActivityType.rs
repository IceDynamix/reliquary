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

//! Generated file from `CmdSummonActivityType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdSummonActivityType)
pub enum CmdSummonActivityType {
    // @@protoc_insertion_point(enum_value:CmdSummonActivityType.CmdSummonActivityTypeNone)
    CmdSummonActivityTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdSummonActivityType.CmdEnterSummonActivityStageCsReq)
    CmdEnterSummonActivityStageCsReq = 7561,
    // @@protoc_insertion_point(enum_value:CmdSummonActivityType.CmdGetSummonActivityDataCsReq)
    CmdGetSummonActivityDataCsReq = 7569,
    // @@protoc_insertion_point(enum_value:CmdSummonActivityType.CmdSummonActivityBattleEndScNotify)
    CmdSummonActivityBattleEndScNotify = 7565,
    // @@protoc_insertion_point(enum_value:CmdSummonActivityType.CmdEnterSummonActivityStageScRsp)
    CmdEnterSummonActivityStageScRsp = 7566,
    // @@protoc_insertion_point(enum_value:CmdSummonActivityType.CmdGetSummonActivityDataScRsp)
    CmdGetSummonActivityDataScRsp = 7568,
}

impl ::protobuf::Enum for CmdSummonActivityType {
    const NAME: &'static str = "CmdSummonActivityType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdSummonActivityType> {
        match value {
            0 => ::std::option::Option::Some(CmdSummonActivityType::CmdSummonActivityTypeNone),
            7561 => ::std::option::Option::Some(CmdSummonActivityType::CmdEnterSummonActivityStageCsReq),
            7569 => ::std::option::Option::Some(CmdSummonActivityType::CmdGetSummonActivityDataCsReq),
            7565 => ::std::option::Option::Some(CmdSummonActivityType::CmdSummonActivityBattleEndScNotify),
            7566 => ::std::option::Option::Some(CmdSummonActivityType::CmdEnterSummonActivityStageScRsp),
            7568 => ::std::option::Option::Some(CmdSummonActivityType::CmdGetSummonActivityDataScRsp),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdSummonActivityType> {
        match str {
            "CmdSummonActivityTypeNone" => ::std::option::Option::Some(CmdSummonActivityType::CmdSummonActivityTypeNone),
            "CmdEnterSummonActivityStageCsReq" => ::std::option::Option::Some(CmdSummonActivityType::CmdEnterSummonActivityStageCsReq),
            "CmdGetSummonActivityDataCsReq" => ::std::option::Option::Some(CmdSummonActivityType::CmdGetSummonActivityDataCsReq),
            "CmdSummonActivityBattleEndScNotify" => ::std::option::Option::Some(CmdSummonActivityType::CmdSummonActivityBattleEndScNotify),
            "CmdEnterSummonActivityStageScRsp" => ::std::option::Option::Some(CmdSummonActivityType::CmdEnterSummonActivityStageScRsp),
            "CmdGetSummonActivityDataScRsp" => ::std::option::Option::Some(CmdSummonActivityType::CmdGetSummonActivityDataScRsp),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdSummonActivityType] = &[
        CmdSummonActivityType::CmdSummonActivityTypeNone,
        CmdSummonActivityType::CmdEnterSummonActivityStageCsReq,
        CmdSummonActivityType::CmdGetSummonActivityDataCsReq,
        CmdSummonActivityType::CmdSummonActivityBattleEndScNotify,
        CmdSummonActivityType::CmdEnterSummonActivityStageScRsp,
        CmdSummonActivityType::CmdGetSummonActivityDataScRsp,
    ];
}

impl ::protobuf::EnumFull for CmdSummonActivityType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdSummonActivityType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdSummonActivityType::CmdSummonActivityTypeNone => 0,
            CmdSummonActivityType::CmdEnterSummonActivityStageCsReq => 1,
            CmdSummonActivityType::CmdGetSummonActivityDataCsReq => 2,
            CmdSummonActivityType::CmdSummonActivityBattleEndScNotify => 3,
            CmdSummonActivityType::CmdEnterSummonActivityStageScRsp => 4,
            CmdSummonActivityType::CmdGetSummonActivityDataScRsp => 5,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdSummonActivityType {
    fn default() -> Self {
        CmdSummonActivityType::CmdSummonActivityTypeNone
    }
}

impl CmdSummonActivityType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdSummonActivityType>("CmdSummonActivityType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bCmdSummonActivityType.proto*\xf5\x01\n\x15CmdSummonActivityType\
    \x12\x1d\n\x19CmdSummonActivityTypeNone\x10\0\x12%\n\x20CmdEnterSummonAc\
    tivityStageCsReq\x10\x89;\x12\"\n\x1dCmdGetSummonActivityDataCsReq\x10\
    \x91;\x12'\n\"CmdSummonActivityBattleEndScNotify\x10\x8d;\x12%\n\x20CmdE\
    nterSummonActivityStageScRsp\x10\x8e;\x12\"\n\x1dCmdGetSummonActivityDat\
    aScRsp\x10\x90;b\x06proto3\
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
            enums.push(CmdSummonActivityType::generated_enum_descriptor_data());
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
