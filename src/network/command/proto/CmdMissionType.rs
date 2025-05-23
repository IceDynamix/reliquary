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
    // @@protoc_insertion_point(enum_value:CmdMissionType.CmdSubMissionRewardScNotify)
    CmdSubMissionRewardScNotify = 1263,
    // @@protoc_insertion_point(enum_value:CmdMissionType.CmdStartFinishMainMissionScNotify)
    CmdStartFinishMainMissionScNotify = 1214,
    // @@protoc_insertion_point(enum_value:CmdMissionType.CmdUpdateMainMissionCustomValueScRsp)
    CmdUpdateMainMissionCustomValueScRsp = 1264,
    // @@protoc_insertion_point(enum_value:CmdMissionType.CmdStartFinishSubMissionScNotify)
    CmdStartFinishSubMissionScNotify = 1225,
    // @@protoc_insertion_point(enum_value:CmdMissionType.CmdGetMissionDataScRsp)
    CmdGetMissionDataScRsp = 1232,
    // @@protoc_insertion_point(enum_value:CmdMissionType.CmdGetMissionStatusCsReq)
    CmdGetMissionStatusCsReq = 1233,
    // @@protoc_insertion_point(enum_value:CmdMissionType.CmdGetMainMissionCustomValueScRsp)
    CmdGetMainMissionCustomValueScRsp = 1275,
    // @@protoc_insertion_point(enum_value:CmdMissionType.CmdFinishedMissionScNotify)
    CmdFinishedMissionScNotify = 1207,
    // @@protoc_insertion_point(enum_value:CmdMissionType.CmdGetMissionStatusScRsp)
    CmdGetMissionStatusScRsp = 1242,
    // @@protoc_insertion_point(enum_value:CmdMissionType.CmdSyncTaskScRsp)
    CmdSyncTaskScRsp = 1271,
    // @@protoc_insertion_point(enum_value:CmdMissionType.CmdGetMainMissionCustomValueCsReq)
    CmdGetMainMissionCustomValueCsReq = 1230,
    // @@protoc_insertion_point(enum_value:CmdMissionType.CmdUpdateMainMissionCustomValueCsReq)
    CmdUpdateMainMissionCustomValueCsReq = 1298,
    // @@protoc_insertion_point(enum_value:CmdMissionType.CmdFinishTalkMissionCsReq)
    CmdFinishTalkMissionCsReq = 1231,
    // @@protoc_insertion_point(enum_value:CmdMissionType.CmdUpdateTrackMainMissionIdCsReq)
    CmdUpdateTrackMainMissionIdCsReq = 1224,
    // @@protoc_insertion_point(enum_value:CmdMissionType.CmdTeleportToMissionResetPointScRsp)
    CmdTeleportToMissionResetPointScRsp = 1293,
    // @@protoc_insertion_point(enum_value:CmdMissionType.CmdAcceptMainMissionScRsp)
    CmdAcceptMainMissionScRsp = 1237,
    // @@protoc_insertion_point(enum_value:CmdMissionType.CmdMissionAcceptScNotify)
    CmdMissionAcceptScNotify = 1251,
    // @@protoc_insertion_point(enum_value:CmdMissionType.CmdAcceptMainMissionCsReq)
    CmdAcceptMainMissionCsReq = 1284,
    // @@protoc_insertion_point(enum_value:CmdMissionType.CmdFinishTalkMissionScRsp)
    CmdFinishTalkMissionScRsp = 1240,
    // @@protoc_insertion_point(enum_value:CmdMissionType.CmdMissionGroupWarnScNotify)
    CmdMissionGroupWarnScNotify = 1285,
    // @@protoc_insertion_point(enum_value:CmdMissionType.CmdUpdateTrackMainMissionIdScRsp)
    CmdUpdateTrackMainMissionIdScRsp = 1257,
    // @@protoc_insertion_point(enum_value:CmdMissionType.CmdSyncTaskCsReq)
    CmdSyncTaskCsReq = 1248,
    // @@protoc_insertion_point(enum_value:CmdMissionType.CmdFinishCosumeItemMissionScRsp)
    CmdFinishCosumeItemMissionScRsp = 1246,
    // @@protoc_insertion_point(enum_value:CmdMissionType.CmdGetMissionDataCsReq)
    CmdGetMissionDataCsReq = 1295,
    // @@protoc_insertion_point(enum_value:CmdMissionType.CmdFinishCosumeItemMissionCsReq)
    CmdFinishCosumeItemMissionCsReq = 1216,
    // @@protoc_insertion_point(enum_value:CmdMissionType.CmdTeleportToMissionResetPointCsReq)
    CmdTeleportToMissionResetPointCsReq = 1211,
    // @@protoc_insertion_point(enum_value:CmdMissionType.CmdMissionRewardScNotify)
    CmdMissionRewardScNotify = 1276,
}

impl ::protobuf::Enum for CmdMissionType {
    const NAME: &'static str = "CmdMissionType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdMissionType> {
        match value {
            0 => ::std::option::Option::Some(CmdMissionType::CmdMissionTypeNone),
            1263 => ::std::option::Option::Some(CmdMissionType::CmdSubMissionRewardScNotify),
            1214 => ::std::option::Option::Some(CmdMissionType::CmdStartFinishMainMissionScNotify),
            1264 => ::std::option::Option::Some(CmdMissionType::CmdUpdateMainMissionCustomValueScRsp),
            1225 => ::std::option::Option::Some(CmdMissionType::CmdStartFinishSubMissionScNotify),
            1232 => ::std::option::Option::Some(CmdMissionType::CmdGetMissionDataScRsp),
            1233 => ::std::option::Option::Some(CmdMissionType::CmdGetMissionStatusCsReq),
            1275 => ::std::option::Option::Some(CmdMissionType::CmdGetMainMissionCustomValueScRsp),
            1207 => ::std::option::Option::Some(CmdMissionType::CmdFinishedMissionScNotify),
            1242 => ::std::option::Option::Some(CmdMissionType::CmdGetMissionStatusScRsp),
            1271 => ::std::option::Option::Some(CmdMissionType::CmdSyncTaskScRsp),
            1230 => ::std::option::Option::Some(CmdMissionType::CmdGetMainMissionCustomValueCsReq),
            1298 => ::std::option::Option::Some(CmdMissionType::CmdUpdateMainMissionCustomValueCsReq),
            1231 => ::std::option::Option::Some(CmdMissionType::CmdFinishTalkMissionCsReq),
            1224 => ::std::option::Option::Some(CmdMissionType::CmdUpdateTrackMainMissionIdCsReq),
            1293 => ::std::option::Option::Some(CmdMissionType::CmdTeleportToMissionResetPointScRsp),
            1237 => ::std::option::Option::Some(CmdMissionType::CmdAcceptMainMissionScRsp),
            1251 => ::std::option::Option::Some(CmdMissionType::CmdMissionAcceptScNotify),
            1284 => ::std::option::Option::Some(CmdMissionType::CmdAcceptMainMissionCsReq),
            1240 => ::std::option::Option::Some(CmdMissionType::CmdFinishTalkMissionScRsp),
            1285 => ::std::option::Option::Some(CmdMissionType::CmdMissionGroupWarnScNotify),
            1257 => ::std::option::Option::Some(CmdMissionType::CmdUpdateTrackMainMissionIdScRsp),
            1248 => ::std::option::Option::Some(CmdMissionType::CmdSyncTaskCsReq),
            1246 => ::std::option::Option::Some(CmdMissionType::CmdFinishCosumeItemMissionScRsp),
            1295 => ::std::option::Option::Some(CmdMissionType::CmdGetMissionDataCsReq),
            1216 => ::std::option::Option::Some(CmdMissionType::CmdFinishCosumeItemMissionCsReq),
            1211 => ::std::option::Option::Some(CmdMissionType::CmdTeleportToMissionResetPointCsReq),
            1276 => ::std::option::Option::Some(CmdMissionType::CmdMissionRewardScNotify),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdMissionType> {
        match str {
            "CmdMissionTypeNone" => ::std::option::Option::Some(CmdMissionType::CmdMissionTypeNone),
            "CmdSubMissionRewardScNotify" => ::std::option::Option::Some(CmdMissionType::CmdSubMissionRewardScNotify),
            "CmdStartFinishMainMissionScNotify" => ::std::option::Option::Some(CmdMissionType::CmdStartFinishMainMissionScNotify),
            "CmdUpdateMainMissionCustomValueScRsp" => ::std::option::Option::Some(CmdMissionType::CmdUpdateMainMissionCustomValueScRsp),
            "CmdStartFinishSubMissionScNotify" => ::std::option::Option::Some(CmdMissionType::CmdStartFinishSubMissionScNotify),
            "CmdGetMissionDataScRsp" => ::std::option::Option::Some(CmdMissionType::CmdGetMissionDataScRsp),
            "CmdGetMissionStatusCsReq" => ::std::option::Option::Some(CmdMissionType::CmdGetMissionStatusCsReq),
            "CmdGetMainMissionCustomValueScRsp" => ::std::option::Option::Some(CmdMissionType::CmdGetMainMissionCustomValueScRsp),
            "CmdFinishedMissionScNotify" => ::std::option::Option::Some(CmdMissionType::CmdFinishedMissionScNotify),
            "CmdGetMissionStatusScRsp" => ::std::option::Option::Some(CmdMissionType::CmdGetMissionStatusScRsp),
            "CmdSyncTaskScRsp" => ::std::option::Option::Some(CmdMissionType::CmdSyncTaskScRsp),
            "CmdGetMainMissionCustomValueCsReq" => ::std::option::Option::Some(CmdMissionType::CmdGetMainMissionCustomValueCsReq),
            "CmdUpdateMainMissionCustomValueCsReq" => ::std::option::Option::Some(CmdMissionType::CmdUpdateMainMissionCustomValueCsReq),
            "CmdFinishTalkMissionCsReq" => ::std::option::Option::Some(CmdMissionType::CmdFinishTalkMissionCsReq),
            "CmdUpdateTrackMainMissionIdCsReq" => ::std::option::Option::Some(CmdMissionType::CmdUpdateTrackMainMissionIdCsReq),
            "CmdTeleportToMissionResetPointScRsp" => ::std::option::Option::Some(CmdMissionType::CmdTeleportToMissionResetPointScRsp),
            "CmdAcceptMainMissionScRsp" => ::std::option::Option::Some(CmdMissionType::CmdAcceptMainMissionScRsp),
            "CmdMissionAcceptScNotify" => ::std::option::Option::Some(CmdMissionType::CmdMissionAcceptScNotify),
            "CmdAcceptMainMissionCsReq" => ::std::option::Option::Some(CmdMissionType::CmdAcceptMainMissionCsReq),
            "CmdFinishTalkMissionScRsp" => ::std::option::Option::Some(CmdMissionType::CmdFinishTalkMissionScRsp),
            "CmdMissionGroupWarnScNotify" => ::std::option::Option::Some(CmdMissionType::CmdMissionGroupWarnScNotify),
            "CmdUpdateTrackMainMissionIdScRsp" => ::std::option::Option::Some(CmdMissionType::CmdUpdateTrackMainMissionIdScRsp),
            "CmdSyncTaskCsReq" => ::std::option::Option::Some(CmdMissionType::CmdSyncTaskCsReq),
            "CmdFinishCosumeItemMissionScRsp" => ::std::option::Option::Some(CmdMissionType::CmdFinishCosumeItemMissionScRsp),
            "CmdGetMissionDataCsReq" => ::std::option::Option::Some(CmdMissionType::CmdGetMissionDataCsReq),
            "CmdFinishCosumeItemMissionCsReq" => ::std::option::Option::Some(CmdMissionType::CmdFinishCosumeItemMissionCsReq),
            "CmdTeleportToMissionResetPointCsReq" => ::std::option::Option::Some(CmdMissionType::CmdTeleportToMissionResetPointCsReq),
            "CmdMissionRewardScNotify" => ::std::option::Option::Some(CmdMissionType::CmdMissionRewardScNotify),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdMissionType] = &[
        CmdMissionType::CmdMissionTypeNone,
        CmdMissionType::CmdSubMissionRewardScNotify,
        CmdMissionType::CmdStartFinishMainMissionScNotify,
        CmdMissionType::CmdUpdateMainMissionCustomValueScRsp,
        CmdMissionType::CmdStartFinishSubMissionScNotify,
        CmdMissionType::CmdGetMissionDataScRsp,
        CmdMissionType::CmdGetMissionStatusCsReq,
        CmdMissionType::CmdGetMainMissionCustomValueScRsp,
        CmdMissionType::CmdFinishedMissionScNotify,
        CmdMissionType::CmdGetMissionStatusScRsp,
        CmdMissionType::CmdSyncTaskScRsp,
        CmdMissionType::CmdGetMainMissionCustomValueCsReq,
        CmdMissionType::CmdUpdateMainMissionCustomValueCsReq,
        CmdMissionType::CmdFinishTalkMissionCsReq,
        CmdMissionType::CmdUpdateTrackMainMissionIdCsReq,
        CmdMissionType::CmdTeleportToMissionResetPointScRsp,
        CmdMissionType::CmdAcceptMainMissionScRsp,
        CmdMissionType::CmdMissionAcceptScNotify,
        CmdMissionType::CmdAcceptMainMissionCsReq,
        CmdMissionType::CmdFinishTalkMissionScRsp,
        CmdMissionType::CmdMissionGroupWarnScNotify,
        CmdMissionType::CmdUpdateTrackMainMissionIdScRsp,
        CmdMissionType::CmdSyncTaskCsReq,
        CmdMissionType::CmdFinishCosumeItemMissionScRsp,
        CmdMissionType::CmdGetMissionDataCsReq,
        CmdMissionType::CmdFinishCosumeItemMissionCsReq,
        CmdMissionType::CmdTeleportToMissionResetPointCsReq,
        CmdMissionType::CmdMissionRewardScNotify,
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
            CmdMissionType::CmdSubMissionRewardScNotify => 1,
            CmdMissionType::CmdStartFinishMainMissionScNotify => 2,
            CmdMissionType::CmdUpdateMainMissionCustomValueScRsp => 3,
            CmdMissionType::CmdStartFinishSubMissionScNotify => 4,
            CmdMissionType::CmdGetMissionDataScRsp => 5,
            CmdMissionType::CmdGetMissionStatusCsReq => 6,
            CmdMissionType::CmdGetMainMissionCustomValueScRsp => 7,
            CmdMissionType::CmdFinishedMissionScNotify => 8,
            CmdMissionType::CmdGetMissionStatusScRsp => 9,
            CmdMissionType::CmdSyncTaskScRsp => 10,
            CmdMissionType::CmdGetMainMissionCustomValueCsReq => 11,
            CmdMissionType::CmdUpdateMainMissionCustomValueCsReq => 12,
            CmdMissionType::CmdFinishTalkMissionCsReq => 13,
            CmdMissionType::CmdUpdateTrackMainMissionIdCsReq => 14,
            CmdMissionType::CmdTeleportToMissionResetPointScRsp => 15,
            CmdMissionType::CmdAcceptMainMissionScRsp => 16,
            CmdMissionType::CmdMissionAcceptScNotify => 17,
            CmdMissionType::CmdAcceptMainMissionCsReq => 18,
            CmdMissionType::CmdFinishTalkMissionScRsp => 19,
            CmdMissionType::CmdMissionGroupWarnScNotify => 20,
            CmdMissionType::CmdUpdateTrackMainMissionIdScRsp => 21,
            CmdMissionType::CmdSyncTaskCsReq => 22,
            CmdMissionType::CmdFinishCosumeItemMissionScRsp => 23,
            CmdMissionType::CmdGetMissionDataCsReq => 24,
            CmdMissionType::CmdFinishCosumeItemMissionCsReq => 25,
            CmdMissionType::CmdTeleportToMissionResetPointCsReq => 26,
            CmdMissionType::CmdMissionRewardScNotify => 27,
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
    \n\x14CmdMissionType.proto*\xd4\x07\n\x0eCmdMissionType\x12\x16\n\x12Cmd\
    MissionTypeNone\x10\0\x12\x20\n\x1bCmdSubMissionRewardScNotify\x10\xef\t\
    \x12&\n!CmdStartFinishMainMissionScNotify\x10\xbe\t\x12)\n$CmdUpdateMain\
    MissionCustomValueScRsp\x10\xf0\t\x12%\n\x20CmdStartFinishSubMissionScNo\
    tify\x10\xc9\t\x12\x1b\n\x16CmdGetMissionDataScRsp\x10\xd0\t\x12\x1d\n\
    \x18CmdGetMissionStatusCsReq\x10\xd1\t\x12&\n!CmdGetMainMissionCustomVal\
    ueScRsp\x10\xfb\t\x12\x1f\n\x1aCmdFinishedMissionScNotify\x10\xb7\t\x12\
    \x1d\n\x18CmdGetMissionStatusScRsp\x10\xda\t\x12\x15\n\x10CmdSyncTaskScR\
    sp\x10\xf7\t\x12&\n!CmdGetMainMissionCustomValueCsReq\x10\xce\t\x12)\n$C\
    mdUpdateMainMissionCustomValueCsReq\x10\x92\n\x12\x1e\n\x19CmdFinishTalk\
    MissionCsReq\x10\xcf\t\x12%\n\x20CmdUpdateTrackMainMissionIdCsReq\x10\
    \xc8\t\x12(\n#CmdTeleportToMissionResetPointScRsp\x10\x8d\n\x12\x1e\n\
    \x19CmdAcceptMainMissionScRsp\x10\xd5\t\x12\x1d\n\x18CmdMissionAcceptScN\
    otify\x10\xe3\t\x12\x1e\n\x19CmdAcceptMainMissionCsReq\x10\x84\n\x12\x1e\
    \n\x19CmdFinishTalkMissionScRsp\x10\xd8\t\x12\x20\n\x1bCmdMissionGroupWa\
    rnScNotify\x10\x85\n\x12%\n\x20CmdUpdateTrackMainMissionIdScRsp\x10\xe9\
    \t\x12\x15\n\x10CmdSyncTaskCsReq\x10\xe0\t\x12$\n\x1fCmdFinishCosumeItem\
    MissionScRsp\x10\xde\t\x12\x1b\n\x16CmdGetMissionDataCsReq\x10\x8f\n\x12\
    $\n\x1fCmdFinishCosumeItemMissionCsReq\x10\xc0\t\x12(\n#CmdTeleportToMis\
    sionResetPointCsReq\x10\xbb\t\x12\x1d\n\x18CmdMissionRewardScNotify\x10\
    \xfc\tb\x06proto3\
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
