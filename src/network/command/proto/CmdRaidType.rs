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

//! Generated file from `CmdRaidType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdRaidType)
pub enum CmdRaidType {
    // @@protoc_insertion_point(enum_value:CmdRaidType.CmdRaidTypeNone)
    CmdRaidTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdRaidType.CmdStartRaidScRsp)
    CmdStartRaidScRsp = 2232,
    // @@protoc_insertion_point(enum_value:CmdRaidType.CmdRaidInfoNotify)
    CmdRaidInfoNotify = 2276,
    // @@protoc_insertion_point(enum_value:CmdRaidType.CmdLeaveRaidScRsp)
    CmdLeaveRaidScRsp = 2240,
    // @@protoc_insertion_point(enum_value:CmdRaidType.CmdDelSaveRaidScNotify)
    CmdDelSaveRaidScNotify = 2243,
    // @@protoc_insertion_point(enum_value:CmdRaidType.CmdGetSaveRaidCsReq)
    CmdGetSaveRaidCsReq = 2283,
    // @@protoc_insertion_point(enum_value:CmdRaidType.CmdGetAllSaveRaidCsReq)
    CmdGetAllSaveRaidCsReq = 2202,
    // @@protoc_insertion_point(enum_value:CmdRaidType.CmdStartRaidCsReq)
    CmdStartRaidCsReq = 2295,
    // @@protoc_insertion_point(enum_value:CmdRaidType.CmdGetSaveRaidScRsp)
    CmdGetSaveRaidScRsp = 2260,
    // @@protoc_insertion_point(enum_value:CmdRaidType.CmdGetRaidInfoCsReq)
    CmdGetRaidInfoCsReq = 2272,
    // @@protoc_insertion_point(enum_value:CmdRaidType.CmdTakeChallengeRaidRewardCsReq)
    CmdTakeChallengeRaidRewardCsReq = 2252,
    // @@protoc_insertion_point(enum_value:CmdRaidType.CmdGetRaidInfoScRsp)
    CmdGetRaidInfoScRsp = 2285,
    // @@protoc_insertion_point(enum_value:CmdRaidType.CmdGetChallengeRaidInfoCsReq)
    CmdGetChallengeRaidInfoCsReq = 2248,
    // @@protoc_insertion_point(enum_value:CmdRaidType.CmdSetClientRaidTargetCountCsReq)
    CmdSetClientRaidTargetCountCsReq = 2216,
    // @@protoc_insertion_point(enum_value:CmdRaidType.CmdLeaveRaidCsReq)
    CmdLeaveRaidCsReq = 2231,
    // @@protoc_insertion_point(enum_value:CmdRaidType.CmdChallengeRaidNotify)
    CmdChallengeRaidNotify = 2256,
    // @@protoc_insertion_point(enum_value:CmdRaidType.CmdGetAllSaveRaidScRsp)
    CmdGetAllSaveRaidScRsp = 2239,
    // @@protoc_insertion_point(enum_value:CmdRaidType.CmdGetChallengeRaidInfoScRsp)
    CmdGetChallengeRaidInfoScRsp = 2271,
    // @@protoc_insertion_point(enum_value:CmdRaidType.CmdTakeChallengeRaidRewardScRsp)
    CmdTakeChallengeRaidRewardScRsp = 2222,
    // @@protoc_insertion_point(enum_value:CmdRaidType.CmdRaidKickByServerScNotify)
    CmdRaidKickByServerScNotify = 2233,
    // @@protoc_insertion_point(enum_value:CmdRaidType.CmdSetClientRaidTargetCountScRsp)
    CmdSetClientRaidTargetCountScRsp = 2246,
}

impl ::protobuf::Enum for CmdRaidType {
    const NAME: &'static str = "CmdRaidType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdRaidType> {
        match value {
            0 => ::std::option::Option::Some(CmdRaidType::CmdRaidTypeNone),
            2232 => ::std::option::Option::Some(CmdRaidType::CmdStartRaidScRsp),
            2276 => ::std::option::Option::Some(CmdRaidType::CmdRaidInfoNotify),
            2240 => ::std::option::Option::Some(CmdRaidType::CmdLeaveRaidScRsp),
            2243 => ::std::option::Option::Some(CmdRaidType::CmdDelSaveRaidScNotify),
            2283 => ::std::option::Option::Some(CmdRaidType::CmdGetSaveRaidCsReq),
            2202 => ::std::option::Option::Some(CmdRaidType::CmdGetAllSaveRaidCsReq),
            2295 => ::std::option::Option::Some(CmdRaidType::CmdStartRaidCsReq),
            2260 => ::std::option::Option::Some(CmdRaidType::CmdGetSaveRaidScRsp),
            2272 => ::std::option::Option::Some(CmdRaidType::CmdGetRaidInfoCsReq),
            2252 => ::std::option::Option::Some(CmdRaidType::CmdTakeChallengeRaidRewardCsReq),
            2285 => ::std::option::Option::Some(CmdRaidType::CmdGetRaidInfoScRsp),
            2248 => ::std::option::Option::Some(CmdRaidType::CmdGetChallengeRaidInfoCsReq),
            2216 => ::std::option::Option::Some(CmdRaidType::CmdSetClientRaidTargetCountCsReq),
            2231 => ::std::option::Option::Some(CmdRaidType::CmdLeaveRaidCsReq),
            2256 => ::std::option::Option::Some(CmdRaidType::CmdChallengeRaidNotify),
            2239 => ::std::option::Option::Some(CmdRaidType::CmdGetAllSaveRaidScRsp),
            2271 => ::std::option::Option::Some(CmdRaidType::CmdGetChallengeRaidInfoScRsp),
            2222 => ::std::option::Option::Some(CmdRaidType::CmdTakeChallengeRaidRewardScRsp),
            2233 => ::std::option::Option::Some(CmdRaidType::CmdRaidKickByServerScNotify),
            2246 => ::std::option::Option::Some(CmdRaidType::CmdSetClientRaidTargetCountScRsp),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdRaidType> {
        match str {
            "CmdRaidTypeNone" => ::std::option::Option::Some(CmdRaidType::CmdRaidTypeNone),
            "CmdStartRaidScRsp" => ::std::option::Option::Some(CmdRaidType::CmdStartRaidScRsp),
            "CmdRaidInfoNotify" => ::std::option::Option::Some(CmdRaidType::CmdRaidInfoNotify),
            "CmdLeaveRaidScRsp" => ::std::option::Option::Some(CmdRaidType::CmdLeaveRaidScRsp),
            "CmdDelSaveRaidScNotify" => ::std::option::Option::Some(CmdRaidType::CmdDelSaveRaidScNotify),
            "CmdGetSaveRaidCsReq" => ::std::option::Option::Some(CmdRaidType::CmdGetSaveRaidCsReq),
            "CmdGetAllSaveRaidCsReq" => ::std::option::Option::Some(CmdRaidType::CmdGetAllSaveRaidCsReq),
            "CmdStartRaidCsReq" => ::std::option::Option::Some(CmdRaidType::CmdStartRaidCsReq),
            "CmdGetSaveRaidScRsp" => ::std::option::Option::Some(CmdRaidType::CmdGetSaveRaidScRsp),
            "CmdGetRaidInfoCsReq" => ::std::option::Option::Some(CmdRaidType::CmdGetRaidInfoCsReq),
            "CmdTakeChallengeRaidRewardCsReq" => ::std::option::Option::Some(CmdRaidType::CmdTakeChallengeRaidRewardCsReq),
            "CmdGetRaidInfoScRsp" => ::std::option::Option::Some(CmdRaidType::CmdGetRaidInfoScRsp),
            "CmdGetChallengeRaidInfoCsReq" => ::std::option::Option::Some(CmdRaidType::CmdGetChallengeRaidInfoCsReq),
            "CmdSetClientRaidTargetCountCsReq" => ::std::option::Option::Some(CmdRaidType::CmdSetClientRaidTargetCountCsReq),
            "CmdLeaveRaidCsReq" => ::std::option::Option::Some(CmdRaidType::CmdLeaveRaidCsReq),
            "CmdChallengeRaidNotify" => ::std::option::Option::Some(CmdRaidType::CmdChallengeRaidNotify),
            "CmdGetAllSaveRaidScRsp" => ::std::option::Option::Some(CmdRaidType::CmdGetAllSaveRaidScRsp),
            "CmdGetChallengeRaidInfoScRsp" => ::std::option::Option::Some(CmdRaidType::CmdGetChallengeRaidInfoScRsp),
            "CmdTakeChallengeRaidRewardScRsp" => ::std::option::Option::Some(CmdRaidType::CmdTakeChallengeRaidRewardScRsp),
            "CmdRaidKickByServerScNotify" => ::std::option::Option::Some(CmdRaidType::CmdRaidKickByServerScNotify),
            "CmdSetClientRaidTargetCountScRsp" => ::std::option::Option::Some(CmdRaidType::CmdSetClientRaidTargetCountScRsp),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdRaidType] = &[
        CmdRaidType::CmdRaidTypeNone,
        CmdRaidType::CmdStartRaidScRsp,
        CmdRaidType::CmdRaidInfoNotify,
        CmdRaidType::CmdLeaveRaidScRsp,
        CmdRaidType::CmdDelSaveRaidScNotify,
        CmdRaidType::CmdGetSaveRaidCsReq,
        CmdRaidType::CmdGetAllSaveRaidCsReq,
        CmdRaidType::CmdStartRaidCsReq,
        CmdRaidType::CmdGetSaveRaidScRsp,
        CmdRaidType::CmdGetRaidInfoCsReq,
        CmdRaidType::CmdTakeChallengeRaidRewardCsReq,
        CmdRaidType::CmdGetRaidInfoScRsp,
        CmdRaidType::CmdGetChallengeRaidInfoCsReq,
        CmdRaidType::CmdSetClientRaidTargetCountCsReq,
        CmdRaidType::CmdLeaveRaidCsReq,
        CmdRaidType::CmdChallengeRaidNotify,
        CmdRaidType::CmdGetAllSaveRaidScRsp,
        CmdRaidType::CmdGetChallengeRaidInfoScRsp,
        CmdRaidType::CmdTakeChallengeRaidRewardScRsp,
        CmdRaidType::CmdRaidKickByServerScNotify,
        CmdRaidType::CmdSetClientRaidTargetCountScRsp,
    ];
}

impl ::protobuf::EnumFull for CmdRaidType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdRaidType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdRaidType::CmdRaidTypeNone => 0,
            CmdRaidType::CmdStartRaidScRsp => 1,
            CmdRaidType::CmdRaidInfoNotify => 2,
            CmdRaidType::CmdLeaveRaidScRsp => 3,
            CmdRaidType::CmdDelSaveRaidScNotify => 4,
            CmdRaidType::CmdGetSaveRaidCsReq => 5,
            CmdRaidType::CmdGetAllSaveRaidCsReq => 6,
            CmdRaidType::CmdStartRaidCsReq => 7,
            CmdRaidType::CmdGetSaveRaidScRsp => 8,
            CmdRaidType::CmdGetRaidInfoCsReq => 9,
            CmdRaidType::CmdTakeChallengeRaidRewardCsReq => 10,
            CmdRaidType::CmdGetRaidInfoScRsp => 11,
            CmdRaidType::CmdGetChallengeRaidInfoCsReq => 12,
            CmdRaidType::CmdSetClientRaidTargetCountCsReq => 13,
            CmdRaidType::CmdLeaveRaidCsReq => 14,
            CmdRaidType::CmdChallengeRaidNotify => 15,
            CmdRaidType::CmdGetAllSaveRaidScRsp => 16,
            CmdRaidType::CmdGetChallengeRaidInfoScRsp => 17,
            CmdRaidType::CmdTakeChallengeRaidRewardScRsp => 18,
            CmdRaidType::CmdRaidKickByServerScNotify => 19,
            CmdRaidType::CmdSetClientRaidTargetCountScRsp => 20,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdRaidType {
    fn default() -> Self {
        CmdRaidType::CmdRaidTypeNone
    }
}

impl CmdRaidType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdRaidType>("CmdRaidType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11CmdRaidType.proto*\xf8\x04\n\x0bCmdRaidType\x12\x13\n\x0fCmdRaidTy\
    peNone\x10\0\x12\x16\n\x11CmdStartRaidScRsp\x10\xb8\x11\x12\x16\n\x11Cmd\
    RaidInfoNotify\x10\xe4\x11\x12\x16\n\x11CmdLeaveRaidScRsp\x10\xc0\x11\
    \x12\x1b\n\x16CmdDelSaveRaidScNotify\x10\xc3\x11\x12\x18\n\x13CmdGetSave\
    RaidCsReq\x10\xeb\x11\x12\x1b\n\x16CmdGetAllSaveRaidCsReq\x10\x9a\x11\
    \x12\x16\n\x11CmdStartRaidCsReq\x10\xf7\x11\x12\x18\n\x13CmdGetSaveRaidS\
    cRsp\x10\xd4\x11\x12\x18\n\x13CmdGetRaidInfoCsReq\x10\xe0\x11\x12$\n\x1f\
    CmdTakeChallengeRaidRewardCsReq\x10\xcc\x11\x12\x18\n\x13CmdGetRaidInfoS\
    cRsp\x10\xed\x11\x12!\n\x1cCmdGetChallengeRaidInfoCsReq\x10\xc8\x11\x12%\
    \n\x20CmdSetClientRaidTargetCountCsReq\x10\xa8\x11\x12\x16\n\x11CmdLeave\
    RaidCsReq\x10\xb7\x11\x12\x1b\n\x16CmdChallengeRaidNotify\x10\xd0\x11\
    \x12\x1b\n\x16CmdGetAllSaveRaidScRsp\x10\xbf\x11\x12!\n\x1cCmdGetChallen\
    geRaidInfoScRsp\x10\xdf\x11\x12$\n\x1fCmdTakeChallengeRaidRewardScRsp\
    \x10\xae\x11\x12\x20\n\x1bCmdRaidKickByServerScNotify\x10\xb9\x11\x12%\n\
    \x20CmdSetClientRaidTargetCountScRsp\x10\xc6\x11b\x06proto3\
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
            enums.push(CmdRaidType::generated_enum_descriptor_data());
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
