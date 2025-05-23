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

//! Generated file from `CmdRogueEndlessType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdRogueEndlessType)
pub enum CmdRogueEndlessType {
    // @@protoc_insertion_point(enum_value:CmdRogueEndlessType.CmdRogueEndlessNone)
    CmdRogueEndlessNone = 0,
    // @@protoc_insertion_point(enum_value:CmdRogueEndlessType.CmdTakeRogueEndlessActivityAllBonusRewardScRsp)
    CmdTakeRogueEndlessActivityAllBonusRewardScRsp = 6010,
    // @@protoc_insertion_point(enum_value:CmdRogueEndlessType.CmdGetRogueEndlessActivityDataCsReq)
    CmdGetRogueEndlessActivityDataCsReq = 6002,
    // @@protoc_insertion_point(enum_value:CmdRogueEndlessType.CmdTakeRogueEndlessActivityPointRewardScRsp)
    CmdTakeRogueEndlessActivityPointRewardScRsp = 6006,
    // @@protoc_insertion_point(enum_value:CmdRogueEndlessType.CmdEnterRogueEndlessActivityStageCsReq)
    CmdEnterRogueEndlessActivityStageCsReq = 6004,
    // @@protoc_insertion_point(enum_value:CmdRogueEndlessType.CmdGetRogueEndlessActivityDataScRsp)
    CmdGetRogueEndlessActivityDataScRsp = 6005,
    // @@protoc_insertion_point(enum_value:CmdRogueEndlessType.CmdRogueEndlessActivityBattleEndScNotify)
    CmdRogueEndlessActivityBattleEndScNotify = 6001,
    // @@protoc_insertion_point(enum_value:CmdRogueEndlessType.CmdTakeRogueEndlessActivityAllBonusRewardCsReq)
    CmdTakeRogueEndlessActivityAllBonusRewardCsReq = 6003,
    // @@protoc_insertion_point(enum_value:CmdRogueEndlessType.CmdTakeRogueEndlessActivityPointRewardCsReq)
    CmdTakeRogueEndlessActivityPointRewardCsReq = 6009,
    // @@protoc_insertion_point(enum_value:CmdRogueEndlessType.CmdEnterRogueEndlessActivityStageScRsp)
    CmdEnterRogueEndlessActivityStageScRsp = 6007,
}

impl ::protobuf::Enum for CmdRogueEndlessType {
    const NAME: &'static str = "CmdRogueEndlessType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdRogueEndlessType> {
        match value {
            0 => ::std::option::Option::Some(CmdRogueEndlessType::CmdRogueEndlessNone),
            6010 => ::std::option::Option::Some(CmdRogueEndlessType::CmdTakeRogueEndlessActivityAllBonusRewardScRsp),
            6002 => ::std::option::Option::Some(CmdRogueEndlessType::CmdGetRogueEndlessActivityDataCsReq),
            6006 => ::std::option::Option::Some(CmdRogueEndlessType::CmdTakeRogueEndlessActivityPointRewardScRsp),
            6004 => ::std::option::Option::Some(CmdRogueEndlessType::CmdEnterRogueEndlessActivityStageCsReq),
            6005 => ::std::option::Option::Some(CmdRogueEndlessType::CmdGetRogueEndlessActivityDataScRsp),
            6001 => ::std::option::Option::Some(CmdRogueEndlessType::CmdRogueEndlessActivityBattleEndScNotify),
            6003 => ::std::option::Option::Some(CmdRogueEndlessType::CmdTakeRogueEndlessActivityAllBonusRewardCsReq),
            6009 => ::std::option::Option::Some(CmdRogueEndlessType::CmdTakeRogueEndlessActivityPointRewardCsReq),
            6007 => ::std::option::Option::Some(CmdRogueEndlessType::CmdEnterRogueEndlessActivityStageScRsp),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdRogueEndlessType> {
        match str {
            "CmdRogueEndlessNone" => ::std::option::Option::Some(CmdRogueEndlessType::CmdRogueEndlessNone),
            "CmdTakeRogueEndlessActivityAllBonusRewardScRsp" => ::std::option::Option::Some(CmdRogueEndlessType::CmdTakeRogueEndlessActivityAllBonusRewardScRsp),
            "CmdGetRogueEndlessActivityDataCsReq" => ::std::option::Option::Some(CmdRogueEndlessType::CmdGetRogueEndlessActivityDataCsReq),
            "CmdTakeRogueEndlessActivityPointRewardScRsp" => ::std::option::Option::Some(CmdRogueEndlessType::CmdTakeRogueEndlessActivityPointRewardScRsp),
            "CmdEnterRogueEndlessActivityStageCsReq" => ::std::option::Option::Some(CmdRogueEndlessType::CmdEnterRogueEndlessActivityStageCsReq),
            "CmdGetRogueEndlessActivityDataScRsp" => ::std::option::Option::Some(CmdRogueEndlessType::CmdGetRogueEndlessActivityDataScRsp),
            "CmdRogueEndlessActivityBattleEndScNotify" => ::std::option::Option::Some(CmdRogueEndlessType::CmdRogueEndlessActivityBattleEndScNotify),
            "CmdTakeRogueEndlessActivityAllBonusRewardCsReq" => ::std::option::Option::Some(CmdRogueEndlessType::CmdTakeRogueEndlessActivityAllBonusRewardCsReq),
            "CmdTakeRogueEndlessActivityPointRewardCsReq" => ::std::option::Option::Some(CmdRogueEndlessType::CmdTakeRogueEndlessActivityPointRewardCsReq),
            "CmdEnterRogueEndlessActivityStageScRsp" => ::std::option::Option::Some(CmdRogueEndlessType::CmdEnterRogueEndlessActivityStageScRsp),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdRogueEndlessType] = &[
        CmdRogueEndlessType::CmdRogueEndlessNone,
        CmdRogueEndlessType::CmdTakeRogueEndlessActivityAllBonusRewardScRsp,
        CmdRogueEndlessType::CmdGetRogueEndlessActivityDataCsReq,
        CmdRogueEndlessType::CmdTakeRogueEndlessActivityPointRewardScRsp,
        CmdRogueEndlessType::CmdEnterRogueEndlessActivityStageCsReq,
        CmdRogueEndlessType::CmdGetRogueEndlessActivityDataScRsp,
        CmdRogueEndlessType::CmdRogueEndlessActivityBattleEndScNotify,
        CmdRogueEndlessType::CmdTakeRogueEndlessActivityAllBonusRewardCsReq,
        CmdRogueEndlessType::CmdTakeRogueEndlessActivityPointRewardCsReq,
        CmdRogueEndlessType::CmdEnterRogueEndlessActivityStageScRsp,
    ];
}

impl ::protobuf::EnumFull for CmdRogueEndlessType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdRogueEndlessType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdRogueEndlessType::CmdRogueEndlessNone => 0,
            CmdRogueEndlessType::CmdTakeRogueEndlessActivityAllBonusRewardScRsp => 1,
            CmdRogueEndlessType::CmdGetRogueEndlessActivityDataCsReq => 2,
            CmdRogueEndlessType::CmdTakeRogueEndlessActivityPointRewardScRsp => 3,
            CmdRogueEndlessType::CmdEnterRogueEndlessActivityStageCsReq => 4,
            CmdRogueEndlessType::CmdGetRogueEndlessActivityDataScRsp => 5,
            CmdRogueEndlessType::CmdRogueEndlessActivityBattleEndScNotify => 6,
            CmdRogueEndlessType::CmdTakeRogueEndlessActivityAllBonusRewardCsReq => 7,
            CmdRogueEndlessType::CmdTakeRogueEndlessActivityPointRewardCsReq => 8,
            CmdRogueEndlessType::CmdEnterRogueEndlessActivityStageScRsp => 9,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdRogueEndlessType {
    fn default() -> Self {
        CmdRogueEndlessType::CmdRogueEndlessNone
    }
}

impl CmdRogueEndlessType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdRogueEndlessType>("CmdRogueEndlessType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x19CmdRogueEndlessType.proto*\xd9\x03\n\x13CmdRogueEndlessType\x12\
    \x17\n\x13CmdRogueEndlessNone\x10\0\x123\n.CmdTakeRogueEndlessActivityAl\
    lBonusRewardScRsp\x10\xfa.\x12(\n#CmdGetRogueEndlessActivityDataCsReq\
    \x10\xf2.\x120\n+CmdTakeRogueEndlessActivityPointRewardScRsp\x10\xf6.\
    \x12+\n&CmdEnterRogueEndlessActivityStageCsReq\x10\xf4.\x12(\n#CmdGetRog\
    ueEndlessActivityDataScRsp\x10\xf5.\x12-\n(CmdRogueEndlessActivityBattle\
    EndScNotify\x10\xf1.\x123\n.CmdTakeRogueEndlessActivityAllBonusRewardCsR\
    eq\x10\xf3.\x120\n+CmdTakeRogueEndlessActivityPointRewardCsReq\x10\xf9.\
    \x12+\n&CmdEnterRogueEndlessActivityStageScRsp\x10\xf7.b\x06proto3\
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
            enums.push(CmdRogueEndlessType::generated_enum_descriptor_data());
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
