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

//! Generated file from `CmdMissionType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdMissionType)
pub enum CmdMissionType {
    // @@protoc_insertion_point(enum_value:CmdMissionType.CmdMissionTypeNone)
    CmdMissionTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdMissionType.CmdFinishTalkMissionCsReq)
    CmdFinishTalkMissionCsReq = 1258,
    // @@protoc_insertion_point(enum_value:CmdMissionType.CmdFinishCosumeItemMissionCsReq)
    CmdFinishCosumeItemMissionCsReq = 1249,
    // @@protoc_insertion_point(enum_value:CmdMissionType.CmdGetMainMissionCustomValueScRsp)
    CmdGetMainMissionCustomValueScRsp = 1282,
    // @@protoc_insertion_point(enum_value:CmdMissionType.CmdStartFinishMainMissionScNotify)
    CmdStartFinishMainMissionScNotify = 1270,
    // @@protoc_insertion_point(enum_value:CmdMissionType.CmdMissionRewardScNotify)
    CmdMissionRewardScNotify = 1230,
    // @@protoc_insertion_point(enum_value:CmdMissionType.CmdGetMissionDataCsReq)
    CmdGetMissionDataCsReq = 1201,
    // @@protoc_insertion_point(enum_value:CmdMissionType.CmdUpdateTrackMainMissionIdCsReq)
    CmdUpdateTrackMainMissionIdCsReq = 1273,
    // @@protoc_insertion_point(enum_value:CmdMissionType.CmdFinishedMissionScNotify)
    CmdFinishedMissionScNotify = 1203,
    // @@protoc_insertion_point(enum_value:CmdMissionType.CmdGetMainMissionCustomValueCsReq)
    CmdGetMainMissionCustomValueCsReq = 1219,
    // @@protoc_insertion_point(enum_value:CmdMissionType.CmdFinishCosumeItemMissionScRsp)
    CmdFinishCosumeItemMissionScRsp = 1222,
    // @@protoc_insertion_point(enum_value:CmdMissionType.CmdFinishTalkMissionScRsp)
    CmdFinishTalkMissionScRsp = 1224,
    // @@protoc_insertion_point(enum_value:CmdMissionType.CmdGetMissionStatusScRsp)
    CmdGetMissionStatusScRsp = 1217,
    // @@protoc_insertion_point(enum_value:CmdMissionType.CmdStartFinishSubMissionScNotify)
    CmdStartFinishSubMissionScNotify = 1292,
    // @@protoc_insertion_point(enum_value:CmdMissionType.CmdGetMissionStatusCsReq)
    CmdGetMissionStatusCsReq = 1212,
    // @@protoc_insertion_point(enum_value:CmdMissionType.CmdSyncTaskScRsp)
    CmdSyncTaskScRsp = 1297,
    // @@protoc_insertion_point(enum_value:CmdMissionType.CmdAcceptMainMissionCsReq)
    CmdAcceptMainMissionCsReq = 1281,
    // @@protoc_insertion_point(enum_value:CmdMissionType.CmdTeleportToMissionResetPointCsReq)
    CmdTeleportToMissionResetPointCsReq = 1269,
    // @@protoc_insertion_point(enum_value:CmdMissionType.CmdSyncTaskCsReq)
    CmdSyncTaskCsReq = 1256,
    // @@protoc_insertion_point(enum_value:CmdMissionType.CmdGetMissionDataScRsp)
    CmdGetMissionDataScRsp = 1268,
    // @@protoc_insertion_point(enum_value:CmdMissionType.CmdMissionGroupWarnScNotify)
    CmdMissionGroupWarnScNotify = 1214,
    // @@protoc_insertion_point(enum_value:CmdMissionType.CmdMissionAcceptScNotify)
    CmdMissionAcceptScNotify = 1259,
    // @@protoc_insertion_point(enum_value:CmdMissionType.CmdTeleportToMissionResetPointScRsp)
    CmdTeleportToMissionResetPointScRsp = 1220,
    // @@protoc_insertion_point(enum_value:CmdMissionType.CmdAcceptMainMissionScRsp)
    CmdAcceptMainMissionScRsp = 1238,
    // @@protoc_insertion_point(enum_value:CmdMissionType.CmdSubMissionRewardScNotify)
    CmdSubMissionRewardScNotify = 1248,
    // @@protoc_insertion_point(enum_value:CmdMissionType.CmdUpdateTrackMainMissionIdScRsp)
    CmdUpdateTrackMainMissionIdScRsp = 1240,
}

impl ::protobuf::Enum for CmdMissionType {
    const NAME: &'static str = "CmdMissionType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdMissionType> {
        match value {
            0 => ::std::option::Option::Some(CmdMissionType::CmdMissionTypeNone),
            1258 => ::std::option::Option::Some(CmdMissionType::CmdFinishTalkMissionCsReq),
            1249 => ::std::option::Option::Some(CmdMissionType::CmdFinishCosumeItemMissionCsReq),
            1282 => ::std::option::Option::Some(CmdMissionType::CmdGetMainMissionCustomValueScRsp),
            1270 => ::std::option::Option::Some(CmdMissionType::CmdStartFinishMainMissionScNotify),
            1230 => ::std::option::Option::Some(CmdMissionType::CmdMissionRewardScNotify),
            1201 => ::std::option::Option::Some(CmdMissionType::CmdGetMissionDataCsReq),
            1273 => ::std::option::Option::Some(CmdMissionType::CmdUpdateTrackMainMissionIdCsReq),
            1203 => ::std::option::Option::Some(CmdMissionType::CmdFinishedMissionScNotify),
            1219 => ::std::option::Option::Some(CmdMissionType::CmdGetMainMissionCustomValueCsReq),
            1222 => ::std::option::Option::Some(CmdMissionType::CmdFinishCosumeItemMissionScRsp),
            1224 => ::std::option::Option::Some(CmdMissionType::CmdFinishTalkMissionScRsp),
            1217 => ::std::option::Option::Some(CmdMissionType::CmdGetMissionStatusScRsp),
            1292 => ::std::option::Option::Some(CmdMissionType::CmdStartFinishSubMissionScNotify),
            1212 => ::std::option::Option::Some(CmdMissionType::CmdGetMissionStatusCsReq),
            1297 => ::std::option::Option::Some(CmdMissionType::CmdSyncTaskScRsp),
            1281 => ::std::option::Option::Some(CmdMissionType::CmdAcceptMainMissionCsReq),
            1269 => ::std::option::Option::Some(CmdMissionType::CmdTeleportToMissionResetPointCsReq),
            1256 => ::std::option::Option::Some(CmdMissionType::CmdSyncTaskCsReq),
            1268 => ::std::option::Option::Some(CmdMissionType::CmdGetMissionDataScRsp),
            1214 => ::std::option::Option::Some(CmdMissionType::CmdMissionGroupWarnScNotify),
            1259 => ::std::option::Option::Some(CmdMissionType::CmdMissionAcceptScNotify),
            1220 => ::std::option::Option::Some(CmdMissionType::CmdTeleportToMissionResetPointScRsp),
            1238 => ::std::option::Option::Some(CmdMissionType::CmdAcceptMainMissionScRsp),
            1248 => ::std::option::Option::Some(CmdMissionType::CmdSubMissionRewardScNotify),
            1240 => ::std::option::Option::Some(CmdMissionType::CmdUpdateTrackMainMissionIdScRsp),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdMissionType> {
        match str {
            "CmdMissionTypeNone" => ::std::option::Option::Some(CmdMissionType::CmdMissionTypeNone),
            "CmdFinishTalkMissionCsReq" => ::std::option::Option::Some(CmdMissionType::CmdFinishTalkMissionCsReq),
            "CmdFinishCosumeItemMissionCsReq" => ::std::option::Option::Some(CmdMissionType::CmdFinishCosumeItemMissionCsReq),
            "CmdGetMainMissionCustomValueScRsp" => ::std::option::Option::Some(CmdMissionType::CmdGetMainMissionCustomValueScRsp),
            "CmdStartFinishMainMissionScNotify" => ::std::option::Option::Some(CmdMissionType::CmdStartFinishMainMissionScNotify),
            "CmdMissionRewardScNotify" => ::std::option::Option::Some(CmdMissionType::CmdMissionRewardScNotify),
            "CmdGetMissionDataCsReq" => ::std::option::Option::Some(CmdMissionType::CmdGetMissionDataCsReq),
            "CmdUpdateTrackMainMissionIdCsReq" => ::std::option::Option::Some(CmdMissionType::CmdUpdateTrackMainMissionIdCsReq),
            "CmdFinishedMissionScNotify" => ::std::option::Option::Some(CmdMissionType::CmdFinishedMissionScNotify),
            "CmdGetMainMissionCustomValueCsReq" => ::std::option::Option::Some(CmdMissionType::CmdGetMainMissionCustomValueCsReq),
            "CmdFinishCosumeItemMissionScRsp" => ::std::option::Option::Some(CmdMissionType::CmdFinishCosumeItemMissionScRsp),
            "CmdFinishTalkMissionScRsp" => ::std::option::Option::Some(CmdMissionType::CmdFinishTalkMissionScRsp),
            "CmdGetMissionStatusScRsp" => ::std::option::Option::Some(CmdMissionType::CmdGetMissionStatusScRsp),
            "CmdStartFinishSubMissionScNotify" => ::std::option::Option::Some(CmdMissionType::CmdStartFinishSubMissionScNotify),
            "CmdGetMissionStatusCsReq" => ::std::option::Option::Some(CmdMissionType::CmdGetMissionStatusCsReq),
            "CmdSyncTaskScRsp" => ::std::option::Option::Some(CmdMissionType::CmdSyncTaskScRsp),
            "CmdAcceptMainMissionCsReq" => ::std::option::Option::Some(CmdMissionType::CmdAcceptMainMissionCsReq),
            "CmdTeleportToMissionResetPointCsReq" => ::std::option::Option::Some(CmdMissionType::CmdTeleportToMissionResetPointCsReq),
            "CmdSyncTaskCsReq" => ::std::option::Option::Some(CmdMissionType::CmdSyncTaskCsReq),
            "CmdGetMissionDataScRsp" => ::std::option::Option::Some(CmdMissionType::CmdGetMissionDataScRsp),
            "CmdMissionGroupWarnScNotify" => ::std::option::Option::Some(CmdMissionType::CmdMissionGroupWarnScNotify),
            "CmdMissionAcceptScNotify" => ::std::option::Option::Some(CmdMissionType::CmdMissionAcceptScNotify),
            "CmdTeleportToMissionResetPointScRsp" => ::std::option::Option::Some(CmdMissionType::CmdTeleportToMissionResetPointScRsp),
            "CmdAcceptMainMissionScRsp" => ::std::option::Option::Some(CmdMissionType::CmdAcceptMainMissionScRsp),
            "CmdSubMissionRewardScNotify" => ::std::option::Option::Some(CmdMissionType::CmdSubMissionRewardScNotify),
            "CmdUpdateTrackMainMissionIdScRsp" => ::std::option::Option::Some(CmdMissionType::CmdUpdateTrackMainMissionIdScRsp),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdMissionType] = &[
        CmdMissionType::CmdMissionTypeNone,
        CmdMissionType::CmdFinishTalkMissionCsReq,
        CmdMissionType::CmdFinishCosumeItemMissionCsReq,
        CmdMissionType::CmdGetMainMissionCustomValueScRsp,
        CmdMissionType::CmdStartFinishMainMissionScNotify,
        CmdMissionType::CmdMissionRewardScNotify,
        CmdMissionType::CmdGetMissionDataCsReq,
        CmdMissionType::CmdUpdateTrackMainMissionIdCsReq,
        CmdMissionType::CmdFinishedMissionScNotify,
        CmdMissionType::CmdGetMainMissionCustomValueCsReq,
        CmdMissionType::CmdFinishCosumeItemMissionScRsp,
        CmdMissionType::CmdFinishTalkMissionScRsp,
        CmdMissionType::CmdGetMissionStatusScRsp,
        CmdMissionType::CmdStartFinishSubMissionScNotify,
        CmdMissionType::CmdGetMissionStatusCsReq,
        CmdMissionType::CmdSyncTaskScRsp,
        CmdMissionType::CmdAcceptMainMissionCsReq,
        CmdMissionType::CmdTeleportToMissionResetPointCsReq,
        CmdMissionType::CmdSyncTaskCsReq,
        CmdMissionType::CmdGetMissionDataScRsp,
        CmdMissionType::CmdMissionGroupWarnScNotify,
        CmdMissionType::CmdMissionAcceptScNotify,
        CmdMissionType::CmdTeleportToMissionResetPointScRsp,
        CmdMissionType::CmdAcceptMainMissionScRsp,
        CmdMissionType::CmdSubMissionRewardScNotify,
        CmdMissionType::CmdUpdateTrackMainMissionIdScRsp,
    ];
}

impl ::protobuf::EnumFull for CmdMissionType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdMissionType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdMissionType::CmdMissionTypeNone => 0,
            CmdMissionType::CmdFinishTalkMissionCsReq => 1,
            CmdMissionType::CmdFinishCosumeItemMissionCsReq => 2,
            CmdMissionType::CmdGetMainMissionCustomValueScRsp => 3,
            CmdMissionType::CmdStartFinishMainMissionScNotify => 4,
            CmdMissionType::CmdMissionRewardScNotify => 5,
            CmdMissionType::CmdGetMissionDataCsReq => 6,
            CmdMissionType::CmdUpdateTrackMainMissionIdCsReq => 7,
            CmdMissionType::CmdFinishedMissionScNotify => 8,
            CmdMissionType::CmdGetMainMissionCustomValueCsReq => 9,
            CmdMissionType::CmdFinishCosumeItemMissionScRsp => 10,
            CmdMissionType::CmdFinishTalkMissionScRsp => 11,
            CmdMissionType::CmdGetMissionStatusScRsp => 12,
            CmdMissionType::CmdStartFinishSubMissionScNotify => 13,
            CmdMissionType::CmdGetMissionStatusCsReq => 14,
            CmdMissionType::CmdSyncTaskScRsp => 15,
            CmdMissionType::CmdAcceptMainMissionCsReq => 16,
            CmdMissionType::CmdTeleportToMissionResetPointCsReq => 17,
            CmdMissionType::CmdSyncTaskCsReq => 18,
            CmdMissionType::CmdGetMissionDataScRsp => 19,
            CmdMissionType::CmdMissionGroupWarnScNotify => 20,
            CmdMissionType::CmdMissionAcceptScNotify => 21,
            CmdMissionType::CmdTeleportToMissionResetPointScRsp => 22,
            CmdMissionType::CmdAcceptMainMissionScRsp => 23,
            CmdMissionType::CmdSubMissionRewardScNotify => 24,
            CmdMissionType::CmdUpdateTrackMainMissionIdScRsp => 25,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdMissionType {
    fn default() -> Self {
        CmdMissionType::CmdMissionTypeNone
    }
}

impl CmdMissionType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdMissionType>("CmdMissionType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x14CmdMissionType.proto*\xfe\x06\n\x0eCmdMissionType\x12\x16\n\x12Cmd\
    MissionTypeNone\x10\0\x12\x1e\n\x19CmdFinishTalkMissionCsReq\x10\xea\t\
    \x12$\n\x1fCmdFinishCosumeItemMissionCsReq\x10\xe1\t\x12&\n!CmdGetMainMi\
    ssionCustomValueScRsp\x10\x82\n\x12&\n!CmdStartFinishMainMissionScNotify\
    \x10\xf6\t\x12\x1d\n\x18CmdMissionRewardScNotify\x10\xce\t\x12\x1b\n\x16\
    CmdGetMissionDataCsReq\x10\xb1\t\x12%\n\x20CmdUpdateTrackMainMissionIdCs\
    Req\x10\xf9\t\x12\x1f\n\x1aCmdFinishedMissionScNotify\x10\xb3\t\x12&\n!C\
    mdGetMainMissionCustomValueCsReq\x10\xc3\t\x12$\n\x1fCmdFinishCosumeItem\
    MissionScRsp\x10\xc6\t\x12\x1e\n\x19CmdFinishTalkMissionScRsp\x10\xc8\t\
    \x12\x1d\n\x18CmdGetMissionStatusScRsp\x10\xc1\t\x12%\n\x20CmdStartFinis\
    hSubMissionScNotify\x10\x8c\n\x12\x1d\n\x18CmdGetMissionStatusCsReq\x10\
    \xbc\t\x12\x15\n\x10CmdSyncTaskScRsp\x10\x91\n\x12\x1e\n\x19CmdAcceptMai\
    nMissionCsReq\x10\x81\n\x12(\n#CmdTeleportToMissionResetPointCsReq\x10\
    \xf5\t\x12\x15\n\x10CmdSyncTaskCsReq\x10\xe8\t\x12\x1b\n\x16CmdGetMissio\
    nDataScRsp\x10\xf4\t\x12\x20\n\x1bCmdMissionGroupWarnScNotify\x10\xbe\t\
    \x12\x1d\n\x18CmdMissionAcceptScNotify\x10\xeb\t\x12(\n#CmdTeleportToMis\
    sionResetPointScRsp\x10\xc4\t\x12\x1e\n\x19CmdAcceptMainMissionScRsp\x10\
    \xd6\t\x12\x20\n\x1bCmdSubMissionRewardScNotify\x10\xe0\t\x12%\n\x20CmdU\
    pdateTrackMainMissionIdScRsp\x10\xd8\tb\x06proto3\
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
            enums.push(CmdMissionType::generated_enum_descriptor_data());
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
