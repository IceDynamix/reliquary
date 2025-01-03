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

//! Generated file from `CmdLineupType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdLineupType)
pub enum CmdLineupType {
    // @@protoc_insertion_point(enum_value:CmdLineupType.CmdLineupTypeNone)
    CmdLineupTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdLineupType.CmdReplaceLineupCsReq)
    CmdReplaceLineupCsReq = 751,
    // @@protoc_insertion_point(enum_value:CmdLineupType.CmdSwitchLineupIndexScRsp)
    CmdSwitchLineupIndexScRsp = 779,
    // @@protoc_insertion_point(enum_value:CmdLineupType.CmdGetStageLineupCsReq)
    CmdGetStageLineupCsReq = 759,
    // @@protoc_insertion_point(enum_value:CmdLineupType.CmdSwitchLineupIndexCsReq)
    CmdSwitchLineupIndexCsReq = 790,
    // @@protoc_insertion_point(enum_value:CmdLineupType.CmdSyncLineupNotify)
    CmdSyncLineupNotify = 747,
    // @@protoc_insertion_point(enum_value:CmdLineupType.CmdQuitLineupScRsp)
    CmdQuitLineupScRsp = 737,
    // @@protoc_insertion_point(enum_value:CmdLineupType.CmdJoinLineupScRsp)
    CmdJoinLineupScRsp = 753,
    // @@protoc_insertion_point(enum_value:CmdLineupType.CmdGetAllLineupDataScRsp)
    CmdGetAllLineupDataScRsp = 729,
    // @@protoc_insertion_point(enum_value:CmdLineupType.CmdGetAllLineupDataCsReq)
    CmdGetAllLineupDataCsReq = 725,
    // @@protoc_insertion_point(enum_value:CmdLineupType.CmdJoinLineupCsReq)
    CmdJoinLineupCsReq = 739,
    // @@protoc_insertion_point(enum_value:CmdLineupType.CmdGetLineupAvatarDataScRsp)
    CmdGetLineupAvatarDataScRsp = 730,
    // @@protoc_insertion_point(enum_value:CmdLineupType.CmdSwapLineupScRsp)
    CmdSwapLineupScRsp = 716,
    // @@protoc_insertion_point(enum_value:CmdLineupType.CmdChangeLineupLeaderScRsp)
    CmdChangeLineupLeaderScRsp = 748,
    // @@protoc_insertion_point(enum_value:CmdLineupType.CmdGetStageLineupScRsp)
    CmdGetStageLineupScRsp = 720,
    // @@protoc_insertion_point(enum_value:CmdLineupType.CmdGetCurLineupDataCsReq)
    CmdGetCurLineupDataCsReq = 703,
    // @@protoc_insertion_point(enum_value:CmdLineupType.CmdQuitLineupCsReq)
    CmdQuitLineupCsReq = 734,
    // @@protoc_insertion_point(enum_value:CmdLineupType.CmdGetCurLineupDataScRsp)
    CmdGetCurLineupDataScRsp = 746,
    // @@protoc_insertion_point(enum_value:CmdLineupType.CmdReplaceLineupScRsp)
    CmdReplaceLineupScRsp = 773,
    // @@protoc_insertion_point(enum_value:CmdLineupType.CmdVirtualLineupTrialAvatarChangeScNotify)
    CmdVirtualLineupTrialAvatarChangeScNotify = 799,
    // @@protoc_insertion_point(enum_value:CmdLineupType.CmdSwapLineupCsReq)
    CmdSwapLineupCsReq = 780,
    // @@protoc_insertion_point(enum_value:CmdLineupType.CmdSetLineupNameCsReq)
    CmdSetLineupNameCsReq = 719,
    // @@protoc_insertion_point(enum_value:CmdLineupType.CmdGetLineupAvatarDataCsReq)
    CmdGetLineupAvatarDataCsReq = 774,
    // @@protoc_insertion_point(enum_value:CmdLineupType.CmdSetLineupNameScRsp)
    CmdSetLineupNameScRsp = 761,
    // @@protoc_insertion_point(enum_value:CmdLineupType.CmdChangeLineupLeaderCsReq)
    CmdChangeLineupLeaderCsReq = 775,
    // @@protoc_insertion_point(enum_value:CmdLineupType.CmdExtraLineupDestroyNotify)
    CmdExtraLineupDestroyNotify = 783,
    // @@protoc_insertion_point(enum_value:CmdLineupType.CmdVirtualLineupDestroyNotify)
    CmdVirtualLineupDestroyNotify = 733,
}

impl ::protobuf::Enum for CmdLineupType {
    const NAME: &'static str = "CmdLineupType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdLineupType> {
        match value {
            0 => ::std::option::Option::Some(CmdLineupType::CmdLineupTypeNone),
            751 => ::std::option::Option::Some(CmdLineupType::CmdReplaceLineupCsReq),
            779 => ::std::option::Option::Some(CmdLineupType::CmdSwitchLineupIndexScRsp),
            759 => ::std::option::Option::Some(CmdLineupType::CmdGetStageLineupCsReq),
            790 => ::std::option::Option::Some(CmdLineupType::CmdSwitchLineupIndexCsReq),
            747 => ::std::option::Option::Some(CmdLineupType::CmdSyncLineupNotify),
            737 => ::std::option::Option::Some(CmdLineupType::CmdQuitLineupScRsp),
            753 => ::std::option::Option::Some(CmdLineupType::CmdJoinLineupScRsp),
            729 => ::std::option::Option::Some(CmdLineupType::CmdGetAllLineupDataScRsp),
            725 => ::std::option::Option::Some(CmdLineupType::CmdGetAllLineupDataCsReq),
            739 => ::std::option::Option::Some(CmdLineupType::CmdJoinLineupCsReq),
            730 => ::std::option::Option::Some(CmdLineupType::CmdGetLineupAvatarDataScRsp),
            716 => ::std::option::Option::Some(CmdLineupType::CmdSwapLineupScRsp),
            748 => ::std::option::Option::Some(CmdLineupType::CmdChangeLineupLeaderScRsp),
            720 => ::std::option::Option::Some(CmdLineupType::CmdGetStageLineupScRsp),
            703 => ::std::option::Option::Some(CmdLineupType::CmdGetCurLineupDataCsReq),
            734 => ::std::option::Option::Some(CmdLineupType::CmdQuitLineupCsReq),
            746 => ::std::option::Option::Some(CmdLineupType::CmdGetCurLineupDataScRsp),
            773 => ::std::option::Option::Some(CmdLineupType::CmdReplaceLineupScRsp),
            799 => ::std::option::Option::Some(CmdLineupType::CmdVirtualLineupTrialAvatarChangeScNotify),
            780 => ::std::option::Option::Some(CmdLineupType::CmdSwapLineupCsReq),
            719 => ::std::option::Option::Some(CmdLineupType::CmdSetLineupNameCsReq),
            774 => ::std::option::Option::Some(CmdLineupType::CmdGetLineupAvatarDataCsReq),
            761 => ::std::option::Option::Some(CmdLineupType::CmdSetLineupNameScRsp),
            775 => ::std::option::Option::Some(CmdLineupType::CmdChangeLineupLeaderCsReq),
            783 => ::std::option::Option::Some(CmdLineupType::CmdExtraLineupDestroyNotify),
            733 => ::std::option::Option::Some(CmdLineupType::CmdVirtualLineupDestroyNotify),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdLineupType> {
        match str {
            "CmdLineupTypeNone" => ::std::option::Option::Some(CmdLineupType::CmdLineupTypeNone),
            "CmdReplaceLineupCsReq" => ::std::option::Option::Some(CmdLineupType::CmdReplaceLineupCsReq),
            "CmdSwitchLineupIndexScRsp" => ::std::option::Option::Some(CmdLineupType::CmdSwitchLineupIndexScRsp),
            "CmdGetStageLineupCsReq" => ::std::option::Option::Some(CmdLineupType::CmdGetStageLineupCsReq),
            "CmdSwitchLineupIndexCsReq" => ::std::option::Option::Some(CmdLineupType::CmdSwitchLineupIndexCsReq),
            "CmdSyncLineupNotify" => ::std::option::Option::Some(CmdLineupType::CmdSyncLineupNotify),
            "CmdQuitLineupScRsp" => ::std::option::Option::Some(CmdLineupType::CmdQuitLineupScRsp),
            "CmdJoinLineupScRsp" => ::std::option::Option::Some(CmdLineupType::CmdJoinLineupScRsp),
            "CmdGetAllLineupDataScRsp" => ::std::option::Option::Some(CmdLineupType::CmdGetAllLineupDataScRsp),
            "CmdGetAllLineupDataCsReq" => ::std::option::Option::Some(CmdLineupType::CmdGetAllLineupDataCsReq),
            "CmdJoinLineupCsReq" => ::std::option::Option::Some(CmdLineupType::CmdJoinLineupCsReq),
            "CmdGetLineupAvatarDataScRsp" => ::std::option::Option::Some(CmdLineupType::CmdGetLineupAvatarDataScRsp),
            "CmdSwapLineupScRsp" => ::std::option::Option::Some(CmdLineupType::CmdSwapLineupScRsp),
            "CmdChangeLineupLeaderScRsp" => ::std::option::Option::Some(CmdLineupType::CmdChangeLineupLeaderScRsp),
            "CmdGetStageLineupScRsp" => ::std::option::Option::Some(CmdLineupType::CmdGetStageLineupScRsp),
            "CmdGetCurLineupDataCsReq" => ::std::option::Option::Some(CmdLineupType::CmdGetCurLineupDataCsReq),
            "CmdQuitLineupCsReq" => ::std::option::Option::Some(CmdLineupType::CmdQuitLineupCsReq),
            "CmdGetCurLineupDataScRsp" => ::std::option::Option::Some(CmdLineupType::CmdGetCurLineupDataScRsp),
            "CmdReplaceLineupScRsp" => ::std::option::Option::Some(CmdLineupType::CmdReplaceLineupScRsp),
            "CmdVirtualLineupTrialAvatarChangeScNotify" => ::std::option::Option::Some(CmdLineupType::CmdVirtualLineupTrialAvatarChangeScNotify),
            "CmdSwapLineupCsReq" => ::std::option::Option::Some(CmdLineupType::CmdSwapLineupCsReq),
            "CmdSetLineupNameCsReq" => ::std::option::Option::Some(CmdLineupType::CmdSetLineupNameCsReq),
            "CmdGetLineupAvatarDataCsReq" => ::std::option::Option::Some(CmdLineupType::CmdGetLineupAvatarDataCsReq),
            "CmdSetLineupNameScRsp" => ::std::option::Option::Some(CmdLineupType::CmdSetLineupNameScRsp),
            "CmdChangeLineupLeaderCsReq" => ::std::option::Option::Some(CmdLineupType::CmdChangeLineupLeaderCsReq),
            "CmdExtraLineupDestroyNotify" => ::std::option::Option::Some(CmdLineupType::CmdExtraLineupDestroyNotify),
            "CmdVirtualLineupDestroyNotify" => ::std::option::Option::Some(CmdLineupType::CmdVirtualLineupDestroyNotify),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdLineupType] = &[
        CmdLineupType::CmdLineupTypeNone,
        CmdLineupType::CmdReplaceLineupCsReq,
        CmdLineupType::CmdSwitchLineupIndexScRsp,
        CmdLineupType::CmdGetStageLineupCsReq,
        CmdLineupType::CmdSwitchLineupIndexCsReq,
        CmdLineupType::CmdSyncLineupNotify,
        CmdLineupType::CmdQuitLineupScRsp,
        CmdLineupType::CmdJoinLineupScRsp,
        CmdLineupType::CmdGetAllLineupDataScRsp,
        CmdLineupType::CmdGetAllLineupDataCsReq,
        CmdLineupType::CmdJoinLineupCsReq,
        CmdLineupType::CmdGetLineupAvatarDataScRsp,
        CmdLineupType::CmdSwapLineupScRsp,
        CmdLineupType::CmdChangeLineupLeaderScRsp,
        CmdLineupType::CmdGetStageLineupScRsp,
        CmdLineupType::CmdGetCurLineupDataCsReq,
        CmdLineupType::CmdQuitLineupCsReq,
        CmdLineupType::CmdGetCurLineupDataScRsp,
        CmdLineupType::CmdReplaceLineupScRsp,
        CmdLineupType::CmdVirtualLineupTrialAvatarChangeScNotify,
        CmdLineupType::CmdSwapLineupCsReq,
        CmdLineupType::CmdSetLineupNameCsReq,
        CmdLineupType::CmdGetLineupAvatarDataCsReq,
        CmdLineupType::CmdSetLineupNameScRsp,
        CmdLineupType::CmdChangeLineupLeaderCsReq,
        CmdLineupType::CmdExtraLineupDestroyNotify,
        CmdLineupType::CmdVirtualLineupDestroyNotify,
    ];
}

impl ::protobuf::EnumFull for CmdLineupType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdLineupType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdLineupType::CmdLineupTypeNone => 0,
            CmdLineupType::CmdReplaceLineupCsReq => 1,
            CmdLineupType::CmdSwitchLineupIndexScRsp => 2,
            CmdLineupType::CmdGetStageLineupCsReq => 3,
            CmdLineupType::CmdSwitchLineupIndexCsReq => 4,
            CmdLineupType::CmdSyncLineupNotify => 5,
            CmdLineupType::CmdQuitLineupScRsp => 6,
            CmdLineupType::CmdJoinLineupScRsp => 7,
            CmdLineupType::CmdGetAllLineupDataScRsp => 8,
            CmdLineupType::CmdGetAllLineupDataCsReq => 9,
            CmdLineupType::CmdJoinLineupCsReq => 10,
            CmdLineupType::CmdGetLineupAvatarDataScRsp => 11,
            CmdLineupType::CmdSwapLineupScRsp => 12,
            CmdLineupType::CmdChangeLineupLeaderScRsp => 13,
            CmdLineupType::CmdGetStageLineupScRsp => 14,
            CmdLineupType::CmdGetCurLineupDataCsReq => 15,
            CmdLineupType::CmdQuitLineupCsReq => 16,
            CmdLineupType::CmdGetCurLineupDataScRsp => 17,
            CmdLineupType::CmdReplaceLineupScRsp => 18,
            CmdLineupType::CmdVirtualLineupTrialAvatarChangeScNotify => 19,
            CmdLineupType::CmdSwapLineupCsReq => 20,
            CmdLineupType::CmdSetLineupNameCsReq => 21,
            CmdLineupType::CmdGetLineupAvatarDataCsReq => 22,
            CmdLineupType::CmdSetLineupNameScRsp => 23,
            CmdLineupType::CmdChangeLineupLeaderCsReq => 24,
            CmdLineupType::CmdExtraLineupDestroyNotify => 25,
            CmdLineupType::CmdVirtualLineupDestroyNotify => 26,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdLineupType {
    fn default() -> Self {
        CmdLineupType::CmdLineupTypeNone
    }
}

impl CmdLineupType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdLineupType>("CmdLineupType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x13CmdLineupType.proto*\xb8\x06\n\rCmdLineupType\x12\x15\n\x11CmdLine\
    upTypeNone\x10\0\x12\x1a\n\x15CmdReplaceLineupCsReq\x10\xef\x05\x12\x1e\
    \n\x19CmdSwitchLineupIndexScRsp\x10\x8b\x06\x12\x1b\n\x16CmdGetStageLine\
    upCsReq\x10\xf7\x05\x12\x1e\n\x19CmdSwitchLineupIndexCsReq\x10\x96\x06\
    \x12\x18\n\x13CmdSyncLineupNotify\x10\xeb\x05\x12\x17\n\x12CmdQuitLineup\
    ScRsp\x10\xe1\x05\x12\x17\n\x12CmdJoinLineupScRsp\x10\xf1\x05\x12\x1d\n\
    \x18CmdGetAllLineupDataScRsp\x10\xd9\x05\x12\x1d\n\x18CmdGetAllLineupDat\
    aCsReq\x10\xd5\x05\x12\x17\n\x12CmdJoinLineupCsReq\x10\xe3\x05\x12\x20\n\
    \x1bCmdGetLineupAvatarDataScRsp\x10\xda\x05\x12\x17\n\x12CmdSwapLineupSc\
    Rsp\x10\xcc\x05\x12\x1f\n\x1aCmdChangeLineupLeaderScRsp\x10\xec\x05\x12\
    \x1b\n\x16CmdGetStageLineupScRsp\x10\xd0\x05\x12\x1d\n\x18CmdGetCurLineu\
    pDataCsReq\x10\xbf\x05\x12\x17\n\x12CmdQuitLineupCsReq\x10\xde\x05\x12\
    \x1d\n\x18CmdGetCurLineupDataScRsp\x10\xea\x05\x12\x1a\n\x15CmdReplaceLi\
    neupScRsp\x10\x85\x06\x12.\n)CmdVirtualLineupTrialAvatarChangeScNotify\
    \x10\x9f\x06\x12\x17\n\x12CmdSwapLineupCsReq\x10\x8c\x06\x12\x1a\n\x15Cm\
    dSetLineupNameCsReq\x10\xcf\x05\x12\x20\n\x1bCmdGetLineupAvatarDataCsReq\
    \x10\x86\x06\x12\x1a\n\x15CmdSetLineupNameScRsp\x10\xf9\x05\x12\x1f\n\
    \x1aCmdChangeLineupLeaderCsReq\x10\x87\x06\x12\x20\n\x1bCmdExtraLineupDe\
    stroyNotify\x10\x8f\x06\x12\"\n\x1dCmdVirtualLineupDestroyNotify\x10\xdd\
    \x05b\x06proto3\
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
            enums.push(CmdLineupType::generated_enum_descriptor_data());
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
