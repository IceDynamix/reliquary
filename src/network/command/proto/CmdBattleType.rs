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

//! Generated file from `CmdBattleType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdBattleType)
pub enum CmdBattleType {
    // @@protoc_insertion_point(enum_value:CmdBattleType.CmdBattleTypeNone)
    CmdBattleTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdBattleType.CmdPVEBattleResultCsReq)
    CmdPVEBattleResultCsReq = 195,
    // @@protoc_insertion_point(enum_value:CmdBattleType.CmdPVEBattleResultScRsp)
    CmdPVEBattleResultScRsp = 132,
    // @@protoc_insertion_point(enum_value:CmdBattleType.CmdSyncClientResVersionCsReq)
    CmdSyncClientResVersionCsReq = 171,
    // @@protoc_insertion_point(enum_value:CmdBattleType.CmdBattleLogReportScRsp)
    CmdBattleLogReportScRsp = 172,
    // @@protoc_insertion_point(enum_value:CmdBattleType.CmdQuitBattleScRsp)
    CmdQuitBattleScRsp = 140,
    // @@protoc_insertion_point(enum_value:CmdBattleType.CmdBattleLogReportCsReq)
    CmdBattleLogReportCsReq = 156,
    // @@protoc_insertion_point(enum_value:CmdBattleType.CmdQuitBattleScNotify)
    CmdQuitBattleScNotify = 122,
    // @@protoc_insertion_point(enum_value:CmdBattleType.CmdGetCurBattleInfoScRsp)
    CmdGetCurBattleInfoScRsp = 148,
    // @@protoc_insertion_point(enum_value:CmdBattleType.CmdGetCurBattleInfoCsReq)
    CmdGetCurBattleInfoCsReq = 176,
    // @@protoc_insertion_point(enum_value:CmdBattleType.CmdReBattleAfterBattleLoseCsNotify)
    CmdReBattleAfterBattleLoseCsNotify = 116,
    // @@protoc_insertion_point(enum_value:CmdBattleType.CmdQuitBattleCsReq)
    CmdQuitBattleCsReq = 131,
    // @@protoc_insertion_point(enum_value:CmdBattleType.CmdServerSimulateBattleFinishScNotify)
    CmdServerSimulateBattleFinishScNotify = 185,
    // @@protoc_insertion_point(enum_value:CmdBattleType.CmdRebattleByClientCsNotify)
    CmdRebattleByClientCsNotify = 146,
    // @@protoc_insertion_point(enum_value:CmdBattleType.CmdSyncClientResVersionScRsp)
    CmdSyncClientResVersionScRsp = 152,
}

impl ::protobuf::Enum for CmdBattleType {
    const NAME: &'static str = "CmdBattleType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdBattleType> {
        match value {
            0 => ::std::option::Option::Some(CmdBattleType::CmdBattleTypeNone),
            195 => ::std::option::Option::Some(CmdBattleType::CmdPVEBattleResultCsReq),
            132 => ::std::option::Option::Some(CmdBattleType::CmdPVEBattleResultScRsp),
            171 => ::std::option::Option::Some(CmdBattleType::CmdSyncClientResVersionCsReq),
            172 => ::std::option::Option::Some(CmdBattleType::CmdBattleLogReportScRsp),
            140 => ::std::option::Option::Some(CmdBattleType::CmdQuitBattleScRsp),
            156 => ::std::option::Option::Some(CmdBattleType::CmdBattleLogReportCsReq),
            122 => ::std::option::Option::Some(CmdBattleType::CmdQuitBattleScNotify),
            148 => ::std::option::Option::Some(CmdBattleType::CmdGetCurBattleInfoScRsp),
            176 => ::std::option::Option::Some(CmdBattleType::CmdGetCurBattleInfoCsReq),
            116 => ::std::option::Option::Some(CmdBattleType::CmdReBattleAfterBattleLoseCsNotify),
            131 => ::std::option::Option::Some(CmdBattleType::CmdQuitBattleCsReq),
            185 => ::std::option::Option::Some(CmdBattleType::CmdServerSimulateBattleFinishScNotify),
            146 => ::std::option::Option::Some(CmdBattleType::CmdRebattleByClientCsNotify),
            152 => ::std::option::Option::Some(CmdBattleType::CmdSyncClientResVersionScRsp),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdBattleType> {
        match str {
            "CmdBattleTypeNone" => ::std::option::Option::Some(CmdBattleType::CmdBattleTypeNone),
            "CmdPVEBattleResultCsReq" => ::std::option::Option::Some(CmdBattleType::CmdPVEBattleResultCsReq),
            "CmdPVEBattleResultScRsp" => ::std::option::Option::Some(CmdBattleType::CmdPVEBattleResultScRsp),
            "CmdSyncClientResVersionCsReq" => ::std::option::Option::Some(CmdBattleType::CmdSyncClientResVersionCsReq),
            "CmdBattleLogReportScRsp" => ::std::option::Option::Some(CmdBattleType::CmdBattleLogReportScRsp),
            "CmdQuitBattleScRsp" => ::std::option::Option::Some(CmdBattleType::CmdQuitBattleScRsp),
            "CmdBattleLogReportCsReq" => ::std::option::Option::Some(CmdBattleType::CmdBattleLogReportCsReq),
            "CmdQuitBattleScNotify" => ::std::option::Option::Some(CmdBattleType::CmdQuitBattleScNotify),
            "CmdGetCurBattleInfoScRsp" => ::std::option::Option::Some(CmdBattleType::CmdGetCurBattleInfoScRsp),
            "CmdGetCurBattleInfoCsReq" => ::std::option::Option::Some(CmdBattleType::CmdGetCurBattleInfoCsReq),
            "CmdReBattleAfterBattleLoseCsNotify" => ::std::option::Option::Some(CmdBattleType::CmdReBattleAfterBattleLoseCsNotify),
            "CmdQuitBattleCsReq" => ::std::option::Option::Some(CmdBattleType::CmdQuitBattleCsReq),
            "CmdServerSimulateBattleFinishScNotify" => ::std::option::Option::Some(CmdBattleType::CmdServerSimulateBattleFinishScNotify),
            "CmdRebattleByClientCsNotify" => ::std::option::Option::Some(CmdBattleType::CmdRebattleByClientCsNotify),
            "CmdSyncClientResVersionScRsp" => ::std::option::Option::Some(CmdBattleType::CmdSyncClientResVersionScRsp),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdBattleType] = &[
        CmdBattleType::CmdBattleTypeNone,
        CmdBattleType::CmdPVEBattleResultCsReq,
        CmdBattleType::CmdPVEBattleResultScRsp,
        CmdBattleType::CmdSyncClientResVersionCsReq,
        CmdBattleType::CmdBattleLogReportScRsp,
        CmdBattleType::CmdQuitBattleScRsp,
        CmdBattleType::CmdBattleLogReportCsReq,
        CmdBattleType::CmdQuitBattleScNotify,
        CmdBattleType::CmdGetCurBattleInfoScRsp,
        CmdBattleType::CmdGetCurBattleInfoCsReq,
        CmdBattleType::CmdReBattleAfterBattleLoseCsNotify,
        CmdBattleType::CmdQuitBattleCsReq,
        CmdBattleType::CmdServerSimulateBattleFinishScNotify,
        CmdBattleType::CmdRebattleByClientCsNotify,
        CmdBattleType::CmdSyncClientResVersionScRsp,
    ];
}

impl ::protobuf::EnumFull for CmdBattleType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdBattleType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdBattleType::CmdBattleTypeNone => 0,
            CmdBattleType::CmdPVEBattleResultCsReq => 1,
            CmdBattleType::CmdPVEBattleResultScRsp => 2,
            CmdBattleType::CmdSyncClientResVersionCsReq => 3,
            CmdBattleType::CmdBattleLogReportScRsp => 4,
            CmdBattleType::CmdQuitBattleScRsp => 5,
            CmdBattleType::CmdBattleLogReportCsReq => 6,
            CmdBattleType::CmdQuitBattleScNotify => 7,
            CmdBattleType::CmdGetCurBattleInfoScRsp => 8,
            CmdBattleType::CmdGetCurBattleInfoCsReq => 9,
            CmdBattleType::CmdReBattleAfterBattleLoseCsNotify => 10,
            CmdBattleType::CmdQuitBattleCsReq => 11,
            CmdBattleType::CmdServerSimulateBattleFinishScNotify => 12,
            CmdBattleType::CmdRebattleByClientCsNotify => 13,
            CmdBattleType::CmdSyncClientResVersionScRsp => 14,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdBattleType {
    fn default() -> Self {
        CmdBattleType::CmdBattleTypeNone
    }
}

impl CmdBattleType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdBattleType>("CmdBattleType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x13CmdBattleType.proto*\xe5\x03\n\rCmdBattleType\x12\x15\n\x11CmdBatt\
    leTypeNone\x10\0\x12\x1c\n\x17CmdPVEBattleResultCsReq\x10\xc3\x01\x12\
    \x1c\n\x17CmdPVEBattleResultScRsp\x10\x84\x01\x12!\n\x1cCmdSyncClientRes\
    VersionCsReq\x10\xab\x01\x12\x1c\n\x17CmdBattleLogReportScRsp\x10\xac\
    \x01\x12\x17\n\x12CmdQuitBattleScRsp\x10\x8c\x01\x12\x1c\n\x17CmdBattleL\
    ogReportCsReq\x10\x9c\x01\x12\x19\n\x15CmdQuitBattleScNotify\x10z\x12\
    \x1d\n\x18CmdGetCurBattleInfoScRsp\x10\x94\x01\x12\x1d\n\x18CmdGetCurBat\
    tleInfoCsReq\x10\xb0\x01\x12&\n\"CmdReBattleAfterBattleLoseCsNotify\x10t\
    \x12\x17\n\x12CmdQuitBattleCsReq\x10\x83\x01\x12*\n%CmdServerSimulateBat\
    tleFinishScNotify\x10\xb9\x01\x12\x20\n\x1bCmdRebattleByClientCsNotify\
    \x10\x92\x01\x12!\n\x1cCmdSyncClientResVersionScRsp\x10\x98\x01b\x06prot\
    o3\
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
            enums.push(CmdBattleType::generated_enum_descriptor_data());
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
