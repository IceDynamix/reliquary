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

//! Generated file from `CmdHeliobusType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdHeliobusType)
pub enum CmdHeliobusType {
    // @@protoc_insertion_point(enum_value:CmdHeliobusType.CmdHeliobusTypeNone)
    CmdHeliobusTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdHeliobusType.CmdHeliobusSnsCommentScRsp)
    CmdHeliobusSnsCommentScRsp = 5838,
    // @@protoc_insertion_point(enum_value:CmdHeliobusType.CmdHeliobusUpgradeLevelScRsp)
    CmdHeliobusUpgradeLevelScRsp = 5822,
    // @@protoc_insertion_point(enum_value:CmdHeliobusType.CmdHeliobusSnsPostScRsp)
    CmdHeliobusSnsPostScRsp = 5877,
    // @@protoc_insertion_point(enum_value:CmdHeliobusType.CmdHeliobusSnsLikeScRsp)
    CmdHeliobusSnsLikeScRsp = 5812,
    // @@protoc_insertion_point(enum_value:CmdHeliobusType.CmdHeliobusSnsPostCsReq)
    CmdHeliobusSnsPostCsReq = 5879,
    // @@protoc_insertion_point(enum_value:CmdHeliobusType.CmdHeliobusUpgradeLevelCsReq)
    CmdHeliobusUpgradeLevelCsReq = 5889,
    // @@protoc_insertion_point(enum_value:CmdHeliobusType.CmdHeliobusStartRaidScRsp)
    CmdHeliobusStartRaidScRsp = 5852,
    // @@protoc_insertion_point(enum_value:CmdHeliobusType.CmdHeliobusSelectSkillCsReq)
    CmdHeliobusSelectSkillCsReq = 5805,
    // @@protoc_insertion_point(enum_value:CmdHeliobusType.CmdHeliobusUnlockSkillScNotify)
    CmdHeliobusUnlockSkillScNotify = 5866,
    // @@protoc_insertion_point(enum_value:CmdHeliobusType.CmdHeliobusEnterBattleCsReq)
    CmdHeliobusEnterBattleCsReq = 5831,
    // @@protoc_insertion_point(enum_value:CmdHeliobusType.CmdHeliobusSnsReadScRsp)
    CmdHeliobusSnsReadScRsp = 5842,
    // @@protoc_insertion_point(enum_value:CmdHeliobusType.CmdHeliobusInfoChangedScNotify)
    CmdHeliobusInfoChangedScNotify = 5856,
    // @@protoc_insertion_point(enum_value:CmdHeliobusType.CmdHeliobusActivityDataCsReq)
    CmdHeliobusActivityDataCsReq = 5898,
    // @@protoc_insertion_point(enum_value:CmdHeliobusType.CmdHeliobusChallengeUpdateScNotify)
    CmdHeliobusChallengeUpdateScNotify = 5811,
    // @@protoc_insertion_point(enum_value:CmdHeliobusType.CmdHeliobusStartRaidCsReq)
    CmdHeliobusStartRaidCsReq = 5860,
    // @@protoc_insertion_point(enum_value:CmdHeliobusType.CmdHeliobusSelectSkillScRsp)
    CmdHeliobusSelectSkillScRsp = 5845,
    // @@protoc_insertion_point(enum_value:CmdHeliobusType.CmdHeliobusActivityDataScRsp)
    CmdHeliobusActivityDataScRsp = 5871,
    // @@protoc_insertion_point(enum_value:CmdHeliobusType.CmdHeliobusEnterBattleScRsp)
    CmdHeliobusEnterBattleScRsp = 5804,
    // @@protoc_insertion_point(enum_value:CmdHeliobusType.CmdHeliobusLineupUpdateScNotify)
    CmdHeliobusLineupUpdateScNotify = 5810,
    // @@protoc_insertion_point(enum_value:CmdHeliobusType.CmdHeliobusSnsReadCsReq)
    CmdHeliobusSnsReadCsReq = 5883,
    // @@protoc_insertion_point(enum_value:CmdHeliobusType.CmdHeliobusSnsCommentCsReq)
    CmdHeliobusSnsCommentCsReq = 5828,
    // @@protoc_insertion_point(enum_value:CmdHeliobusType.CmdHeliobusSnsLikeCsReq)
    CmdHeliobusSnsLikeCsReq = 5833,
    // @@protoc_insertion_point(enum_value:CmdHeliobusType.CmdHeliobusSnsUpdateScNotify)
    CmdHeliobusSnsUpdateScNotify = 5878,
}

impl ::protobuf::Enum for CmdHeliobusType {
    const NAME: &'static str = "CmdHeliobusType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdHeliobusType> {
        match value {
            0 => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusTypeNone),
            5838 => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusSnsCommentScRsp),
            5822 => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusUpgradeLevelScRsp),
            5877 => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusSnsPostScRsp),
            5812 => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusSnsLikeScRsp),
            5879 => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusSnsPostCsReq),
            5889 => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusUpgradeLevelCsReq),
            5852 => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusStartRaidScRsp),
            5805 => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusSelectSkillCsReq),
            5866 => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusUnlockSkillScNotify),
            5831 => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusEnterBattleCsReq),
            5842 => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusSnsReadScRsp),
            5856 => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusInfoChangedScNotify),
            5898 => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusActivityDataCsReq),
            5811 => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusChallengeUpdateScNotify),
            5860 => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusStartRaidCsReq),
            5845 => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusSelectSkillScRsp),
            5871 => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusActivityDataScRsp),
            5804 => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusEnterBattleScRsp),
            5810 => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusLineupUpdateScNotify),
            5883 => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusSnsReadCsReq),
            5828 => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusSnsCommentCsReq),
            5833 => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusSnsLikeCsReq),
            5878 => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusSnsUpdateScNotify),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdHeliobusType> {
        match str {
            "CmdHeliobusTypeNone" => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusTypeNone),
            "CmdHeliobusSnsCommentScRsp" => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusSnsCommentScRsp),
            "CmdHeliobusUpgradeLevelScRsp" => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusUpgradeLevelScRsp),
            "CmdHeliobusSnsPostScRsp" => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusSnsPostScRsp),
            "CmdHeliobusSnsLikeScRsp" => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusSnsLikeScRsp),
            "CmdHeliobusSnsPostCsReq" => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusSnsPostCsReq),
            "CmdHeliobusUpgradeLevelCsReq" => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusUpgradeLevelCsReq),
            "CmdHeliobusStartRaidScRsp" => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusStartRaidScRsp),
            "CmdHeliobusSelectSkillCsReq" => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusSelectSkillCsReq),
            "CmdHeliobusUnlockSkillScNotify" => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusUnlockSkillScNotify),
            "CmdHeliobusEnterBattleCsReq" => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusEnterBattleCsReq),
            "CmdHeliobusSnsReadScRsp" => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusSnsReadScRsp),
            "CmdHeliobusInfoChangedScNotify" => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusInfoChangedScNotify),
            "CmdHeliobusActivityDataCsReq" => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusActivityDataCsReq),
            "CmdHeliobusChallengeUpdateScNotify" => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusChallengeUpdateScNotify),
            "CmdHeliobusStartRaidCsReq" => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusStartRaidCsReq),
            "CmdHeliobusSelectSkillScRsp" => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusSelectSkillScRsp),
            "CmdHeliobusActivityDataScRsp" => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusActivityDataScRsp),
            "CmdHeliobusEnterBattleScRsp" => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusEnterBattleScRsp),
            "CmdHeliobusLineupUpdateScNotify" => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusLineupUpdateScNotify),
            "CmdHeliobusSnsReadCsReq" => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusSnsReadCsReq),
            "CmdHeliobusSnsCommentCsReq" => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusSnsCommentCsReq),
            "CmdHeliobusSnsLikeCsReq" => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusSnsLikeCsReq),
            "CmdHeliobusSnsUpdateScNotify" => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusSnsUpdateScNotify),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdHeliobusType] = &[
        CmdHeliobusType::CmdHeliobusTypeNone,
        CmdHeliobusType::CmdHeliobusSnsCommentScRsp,
        CmdHeliobusType::CmdHeliobusUpgradeLevelScRsp,
        CmdHeliobusType::CmdHeliobusSnsPostScRsp,
        CmdHeliobusType::CmdHeliobusSnsLikeScRsp,
        CmdHeliobusType::CmdHeliobusSnsPostCsReq,
        CmdHeliobusType::CmdHeliobusUpgradeLevelCsReq,
        CmdHeliobusType::CmdHeliobusStartRaidScRsp,
        CmdHeliobusType::CmdHeliobusSelectSkillCsReq,
        CmdHeliobusType::CmdHeliobusUnlockSkillScNotify,
        CmdHeliobusType::CmdHeliobusEnterBattleCsReq,
        CmdHeliobusType::CmdHeliobusSnsReadScRsp,
        CmdHeliobusType::CmdHeliobusInfoChangedScNotify,
        CmdHeliobusType::CmdHeliobusActivityDataCsReq,
        CmdHeliobusType::CmdHeliobusChallengeUpdateScNotify,
        CmdHeliobusType::CmdHeliobusStartRaidCsReq,
        CmdHeliobusType::CmdHeliobusSelectSkillScRsp,
        CmdHeliobusType::CmdHeliobusActivityDataScRsp,
        CmdHeliobusType::CmdHeliobusEnterBattleScRsp,
        CmdHeliobusType::CmdHeliobusLineupUpdateScNotify,
        CmdHeliobusType::CmdHeliobusSnsReadCsReq,
        CmdHeliobusType::CmdHeliobusSnsCommentCsReq,
        CmdHeliobusType::CmdHeliobusSnsLikeCsReq,
        CmdHeliobusType::CmdHeliobusSnsUpdateScNotify,
    ];
}

impl ::protobuf::EnumFull for CmdHeliobusType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdHeliobusType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdHeliobusType::CmdHeliobusTypeNone => 0,
            CmdHeliobusType::CmdHeliobusSnsCommentScRsp => 1,
            CmdHeliobusType::CmdHeliobusUpgradeLevelScRsp => 2,
            CmdHeliobusType::CmdHeliobusSnsPostScRsp => 3,
            CmdHeliobusType::CmdHeliobusSnsLikeScRsp => 4,
            CmdHeliobusType::CmdHeliobusSnsPostCsReq => 5,
            CmdHeliobusType::CmdHeliobusUpgradeLevelCsReq => 6,
            CmdHeliobusType::CmdHeliobusStartRaidScRsp => 7,
            CmdHeliobusType::CmdHeliobusSelectSkillCsReq => 8,
            CmdHeliobusType::CmdHeliobusUnlockSkillScNotify => 9,
            CmdHeliobusType::CmdHeliobusEnterBattleCsReq => 10,
            CmdHeliobusType::CmdHeliobusSnsReadScRsp => 11,
            CmdHeliobusType::CmdHeliobusInfoChangedScNotify => 12,
            CmdHeliobusType::CmdHeliobusActivityDataCsReq => 13,
            CmdHeliobusType::CmdHeliobusChallengeUpdateScNotify => 14,
            CmdHeliobusType::CmdHeliobusStartRaidCsReq => 15,
            CmdHeliobusType::CmdHeliobusSelectSkillScRsp => 16,
            CmdHeliobusType::CmdHeliobusActivityDataScRsp => 17,
            CmdHeliobusType::CmdHeliobusEnterBattleScRsp => 18,
            CmdHeliobusType::CmdHeliobusLineupUpdateScNotify => 19,
            CmdHeliobusType::CmdHeliobusSnsReadCsReq => 20,
            CmdHeliobusType::CmdHeliobusSnsCommentCsReq => 21,
            CmdHeliobusType::CmdHeliobusSnsLikeCsReq => 22,
            CmdHeliobusType::CmdHeliobusSnsUpdateScNotify => 23,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdHeliobusType {
    fn default() -> Self {
        CmdHeliobusType::CmdHeliobusTypeNone
    }
}

impl CmdHeliobusType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdHeliobusType>("CmdHeliobusType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15CmdHeliobusType.proto*\xb0\x06\n\x0fCmdHeliobusType\x12\x17\n\x13C\
    mdHeliobusTypeNone\x10\0\x12\x1f\n\x1aCmdHeliobusSnsCommentScRsp\x10\xce\
    -\x12!\n\x1cCmdHeliobusUpgradeLevelScRsp\x10\xbe-\x12\x1c\n\x17CmdHeliob\
    usSnsPostScRsp\x10\xf5-\x12\x1c\n\x17CmdHeliobusSnsLikeScRsp\x10\xb4-\
    \x12\x1c\n\x17CmdHeliobusSnsPostCsReq\x10\xf7-\x12!\n\x1cCmdHeliobusUpgr\
    adeLevelCsReq\x10\x81.\x12\x1e\n\x19CmdHeliobusStartRaidScRsp\x10\xdc-\
    \x12\x20\n\x1bCmdHeliobusSelectSkillCsReq\x10\xad-\x12#\n\x1eCmdHeliobus\
    UnlockSkillScNotify\x10\xea-\x12\x20\n\x1bCmdHeliobusEnterBattleCsReq\
    \x10\xc7-\x12\x1c\n\x17CmdHeliobusSnsReadScRsp\x10\xd2-\x12#\n\x1eCmdHel\
    iobusInfoChangedScNotify\x10\xe0-\x12!\n\x1cCmdHeliobusActivityDataCsReq\
    \x10\x8a.\x12'\n\"CmdHeliobusChallengeUpdateScNotify\x10\xb3-\x12\x1e\n\
    \x19CmdHeliobusStartRaidCsReq\x10\xe4-\x12\x20\n\x1bCmdHeliobusSelectSki\
    llScRsp\x10\xd5-\x12!\n\x1cCmdHeliobusActivityDataScRsp\x10\xef-\x12\x20\
    \n\x1bCmdHeliobusEnterBattleScRsp\x10\xac-\x12$\n\x1fCmdHeliobusLineupUp\
    dateScNotify\x10\xb2-\x12\x1c\n\x17CmdHeliobusSnsReadCsReq\x10\xfb-\x12\
    \x1f\n\x1aCmdHeliobusSnsCommentCsReq\x10\xc4-\x12\x1c\n\x17CmdHeliobusSn\
    sLikeCsReq\x10\xc9-\x12!\n\x1cCmdHeliobusSnsUpdateScNotify\x10\xf6-b\x06\
    proto3\
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
            enums.push(CmdHeliobusType::generated_enum_descriptor_data());
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
