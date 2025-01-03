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
    // @@protoc_insertion_point(enum_value:CmdHeliobusType.CmdHeliobusActivityDataCsReq)
    CmdHeliobusActivityDataCsReq = 5859,
    // @@protoc_insertion_point(enum_value:CmdHeliobusType.CmdHeliobusSnsLikeScRsp)
    CmdHeliobusSnsLikeScRsp = 5837,
    // @@protoc_insertion_point(enum_value:CmdHeliobusType.CmdHeliobusSnsUpdateScNotify)
    CmdHeliobusSnsUpdateScNotify = 5847,
    // @@protoc_insertion_point(enum_value:CmdHeliobusType.CmdHeliobusChallengeUpdateScNotify)
    CmdHeliobusChallengeUpdateScNotify = 5873,
    // @@protoc_insertion_point(enum_value:CmdHeliobusType.CmdHeliobusSelectSkillCsReq)
    CmdHeliobusSelectSkillCsReq = 5890,
    // @@protoc_insertion_point(enum_value:CmdHeliobusType.CmdHeliobusUpgradeLevelCsReq)
    CmdHeliobusUpgradeLevelCsReq = 5830,
    // @@protoc_insertion_point(enum_value:CmdHeliobusType.CmdHeliobusUpgradeLevelScRsp)
    CmdHeliobusUpgradeLevelScRsp = 5875,
    // @@protoc_insertion_point(enum_value:CmdHeliobusType.CmdHeliobusSnsPostScRsp)
    CmdHeliobusSnsPostScRsp = 5853,
    // @@protoc_insertion_point(enum_value:CmdHeliobusType.CmdHeliobusStartRaidCsReq)
    CmdHeliobusStartRaidCsReq = 5833,
    // @@protoc_insertion_point(enum_value:CmdHeliobusType.CmdHeliobusEnterBattleScRsp)
    CmdHeliobusEnterBattleScRsp = 5829,
    // @@protoc_insertion_point(enum_value:CmdHeliobusType.CmdHeliobusUnlockSkillScNotify)
    CmdHeliobusUnlockSkillScNotify = 5848,
    // @@protoc_insertion_point(enum_value:CmdHeliobusType.CmdHeliobusStartRaidScRsp)
    CmdHeliobusStartRaidScRsp = 5851,
    // @@protoc_insertion_point(enum_value:CmdHeliobusType.CmdHeliobusSnsCommentScRsp)
    CmdHeliobusSnsCommentScRsp = 5816,
    // @@protoc_insertion_point(enum_value:CmdHeliobusType.CmdHeliobusInfoChangedScNotify)
    CmdHeliobusInfoChangedScNotify = 5874,
    // @@protoc_insertion_point(enum_value:CmdHeliobusType.CmdHeliobusSnsLikeCsReq)
    CmdHeliobusSnsLikeCsReq = 5834,
    // @@protoc_insertion_point(enum_value:CmdHeliobusType.CmdHeliobusActivityDataScRsp)
    CmdHeliobusActivityDataScRsp = 5820,
    // @@protoc_insertion_point(enum_value:CmdHeliobusType.CmdHeliobusSelectSkillScRsp)
    CmdHeliobusSelectSkillScRsp = 5879,
    // @@protoc_insertion_point(enum_value:CmdHeliobusType.CmdHeliobusSnsPostCsReq)
    CmdHeliobusSnsPostCsReq = 5839,
    // @@protoc_insertion_point(enum_value:CmdHeliobusType.CmdHeliobusLineupUpdateScNotify)
    CmdHeliobusLineupUpdateScNotify = 5883,
    // @@protoc_insertion_point(enum_value:CmdHeliobusType.CmdHeliobusSnsReadCsReq)
    CmdHeliobusSnsReadCsReq = 5803,
    // @@protoc_insertion_point(enum_value:CmdHeliobusType.CmdHeliobusEnterBattleCsReq)
    CmdHeliobusEnterBattleCsReq = 5825,
    // @@protoc_insertion_point(enum_value:CmdHeliobusType.CmdHeliobusSnsReadScRsp)
    CmdHeliobusSnsReadScRsp = 5846,
    // @@protoc_insertion_point(enum_value:CmdHeliobusType.CmdHeliobusSnsCommentCsReq)
    CmdHeliobusSnsCommentCsReq = 5880,
}

impl ::protobuf::Enum for CmdHeliobusType {
    const NAME: &'static str = "CmdHeliobusType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdHeliobusType> {
        match value {
            0 => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusTypeNone),
            5859 => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusActivityDataCsReq),
            5837 => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusSnsLikeScRsp),
            5847 => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusSnsUpdateScNotify),
            5873 => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusChallengeUpdateScNotify),
            5890 => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusSelectSkillCsReq),
            5830 => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusUpgradeLevelCsReq),
            5875 => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusUpgradeLevelScRsp),
            5853 => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusSnsPostScRsp),
            5833 => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusStartRaidCsReq),
            5829 => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusEnterBattleScRsp),
            5848 => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusUnlockSkillScNotify),
            5851 => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusStartRaidScRsp),
            5816 => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusSnsCommentScRsp),
            5874 => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusInfoChangedScNotify),
            5834 => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusSnsLikeCsReq),
            5820 => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusActivityDataScRsp),
            5879 => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusSelectSkillScRsp),
            5839 => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusSnsPostCsReq),
            5883 => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusLineupUpdateScNotify),
            5803 => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusSnsReadCsReq),
            5825 => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusEnterBattleCsReq),
            5846 => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusSnsReadScRsp),
            5880 => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusSnsCommentCsReq),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdHeliobusType> {
        match str {
            "CmdHeliobusTypeNone" => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusTypeNone),
            "CmdHeliobusActivityDataCsReq" => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusActivityDataCsReq),
            "CmdHeliobusSnsLikeScRsp" => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusSnsLikeScRsp),
            "CmdHeliobusSnsUpdateScNotify" => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusSnsUpdateScNotify),
            "CmdHeliobusChallengeUpdateScNotify" => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusChallengeUpdateScNotify),
            "CmdHeliobusSelectSkillCsReq" => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusSelectSkillCsReq),
            "CmdHeliobusUpgradeLevelCsReq" => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusUpgradeLevelCsReq),
            "CmdHeliobusUpgradeLevelScRsp" => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusUpgradeLevelScRsp),
            "CmdHeliobusSnsPostScRsp" => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusSnsPostScRsp),
            "CmdHeliobusStartRaidCsReq" => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusStartRaidCsReq),
            "CmdHeliobusEnterBattleScRsp" => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusEnterBattleScRsp),
            "CmdHeliobusUnlockSkillScNotify" => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusUnlockSkillScNotify),
            "CmdHeliobusStartRaidScRsp" => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusStartRaidScRsp),
            "CmdHeliobusSnsCommentScRsp" => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusSnsCommentScRsp),
            "CmdHeliobusInfoChangedScNotify" => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusInfoChangedScNotify),
            "CmdHeliobusSnsLikeCsReq" => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusSnsLikeCsReq),
            "CmdHeliobusActivityDataScRsp" => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusActivityDataScRsp),
            "CmdHeliobusSelectSkillScRsp" => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusSelectSkillScRsp),
            "CmdHeliobusSnsPostCsReq" => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusSnsPostCsReq),
            "CmdHeliobusLineupUpdateScNotify" => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusLineupUpdateScNotify),
            "CmdHeliobusSnsReadCsReq" => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusSnsReadCsReq),
            "CmdHeliobusEnterBattleCsReq" => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusEnterBattleCsReq),
            "CmdHeliobusSnsReadScRsp" => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusSnsReadScRsp),
            "CmdHeliobusSnsCommentCsReq" => ::std::option::Option::Some(CmdHeliobusType::CmdHeliobusSnsCommentCsReq),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdHeliobusType] = &[
        CmdHeliobusType::CmdHeliobusTypeNone,
        CmdHeliobusType::CmdHeliobusActivityDataCsReq,
        CmdHeliobusType::CmdHeliobusSnsLikeScRsp,
        CmdHeliobusType::CmdHeliobusSnsUpdateScNotify,
        CmdHeliobusType::CmdHeliobusChallengeUpdateScNotify,
        CmdHeliobusType::CmdHeliobusSelectSkillCsReq,
        CmdHeliobusType::CmdHeliobusUpgradeLevelCsReq,
        CmdHeliobusType::CmdHeliobusUpgradeLevelScRsp,
        CmdHeliobusType::CmdHeliobusSnsPostScRsp,
        CmdHeliobusType::CmdHeliobusStartRaidCsReq,
        CmdHeliobusType::CmdHeliobusEnterBattleScRsp,
        CmdHeliobusType::CmdHeliobusUnlockSkillScNotify,
        CmdHeliobusType::CmdHeliobusStartRaidScRsp,
        CmdHeliobusType::CmdHeliobusSnsCommentScRsp,
        CmdHeliobusType::CmdHeliobusInfoChangedScNotify,
        CmdHeliobusType::CmdHeliobusSnsLikeCsReq,
        CmdHeliobusType::CmdHeliobusActivityDataScRsp,
        CmdHeliobusType::CmdHeliobusSelectSkillScRsp,
        CmdHeliobusType::CmdHeliobusSnsPostCsReq,
        CmdHeliobusType::CmdHeliobusLineupUpdateScNotify,
        CmdHeliobusType::CmdHeliobusSnsReadCsReq,
        CmdHeliobusType::CmdHeliobusEnterBattleCsReq,
        CmdHeliobusType::CmdHeliobusSnsReadScRsp,
        CmdHeliobusType::CmdHeliobusSnsCommentCsReq,
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
            CmdHeliobusType::CmdHeliobusActivityDataCsReq => 1,
            CmdHeliobusType::CmdHeliobusSnsLikeScRsp => 2,
            CmdHeliobusType::CmdHeliobusSnsUpdateScNotify => 3,
            CmdHeliobusType::CmdHeliobusChallengeUpdateScNotify => 4,
            CmdHeliobusType::CmdHeliobusSelectSkillCsReq => 5,
            CmdHeliobusType::CmdHeliobusUpgradeLevelCsReq => 6,
            CmdHeliobusType::CmdHeliobusUpgradeLevelScRsp => 7,
            CmdHeliobusType::CmdHeliobusSnsPostScRsp => 8,
            CmdHeliobusType::CmdHeliobusStartRaidCsReq => 9,
            CmdHeliobusType::CmdHeliobusEnterBattleScRsp => 10,
            CmdHeliobusType::CmdHeliobusUnlockSkillScNotify => 11,
            CmdHeliobusType::CmdHeliobusStartRaidScRsp => 12,
            CmdHeliobusType::CmdHeliobusSnsCommentScRsp => 13,
            CmdHeliobusType::CmdHeliobusInfoChangedScNotify => 14,
            CmdHeliobusType::CmdHeliobusSnsLikeCsReq => 15,
            CmdHeliobusType::CmdHeliobusActivityDataScRsp => 16,
            CmdHeliobusType::CmdHeliobusSelectSkillScRsp => 17,
            CmdHeliobusType::CmdHeliobusSnsPostCsReq => 18,
            CmdHeliobusType::CmdHeliobusLineupUpdateScNotify => 19,
            CmdHeliobusType::CmdHeliobusSnsReadCsReq => 20,
            CmdHeliobusType::CmdHeliobusEnterBattleCsReq => 21,
            CmdHeliobusType::CmdHeliobusSnsReadScRsp => 22,
            CmdHeliobusType::CmdHeliobusSnsCommentCsReq => 23,
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
    mdHeliobusTypeNone\x10\0\x12!\n\x1cCmdHeliobusActivityDataCsReq\x10\xe3-\
    \x12\x1c\n\x17CmdHeliobusSnsLikeScRsp\x10\xcd-\x12!\n\x1cCmdHeliobusSnsU\
    pdateScNotify\x10\xd7-\x12'\n\"CmdHeliobusChallengeUpdateScNotify\x10\
    \xf1-\x12\x20\n\x1bCmdHeliobusSelectSkillCsReq\x10\x82.\x12!\n\x1cCmdHel\
    iobusUpgradeLevelCsReq\x10\xc6-\x12!\n\x1cCmdHeliobusUpgradeLevelScRsp\
    \x10\xf3-\x12\x1c\n\x17CmdHeliobusSnsPostScRsp\x10\xdd-\x12\x1e\n\x19Cmd\
    HeliobusStartRaidCsReq\x10\xc9-\x12\x20\n\x1bCmdHeliobusEnterBattleScRsp\
    \x10\xc5-\x12#\n\x1eCmdHeliobusUnlockSkillScNotify\x10\xd8-\x12\x1e\n\
    \x19CmdHeliobusStartRaidScRsp\x10\xdb-\x12\x1f\n\x1aCmdHeliobusSnsCommen\
    tScRsp\x10\xb8-\x12#\n\x1eCmdHeliobusInfoChangedScNotify\x10\xf2-\x12\
    \x1c\n\x17CmdHeliobusSnsLikeCsReq\x10\xca-\x12!\n\x1cCmdHeliobusActivity\
    DataScRsp\x10\xbc-\x12\x20\n\x1bCmdHeliobusSelectSkillScRsp\x10\xf7-\x12\
    \x1c\n\x17CmdHeliobusSnsPostCsReq\x10\xcf-\x12$\n\x1fCmdHeliobusLineupUp\
    dateScNotify\x10\xfb-\x12\x1c\n\x17CmdHeliobusSnsReadCsReq\x10\xab-\x12\
    \x20\n\x1bCmdHeliobusEnterBattleCsReq\x10\xc1-\x12\x1c\n\x17CmdHeliobusS\
    nsReadScRsp\x10\xd6-\x12\x1f\n\x1aCmdHeliobusSnsCommentCsReq\x10\xf8-b\
    \x06proto3\
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
