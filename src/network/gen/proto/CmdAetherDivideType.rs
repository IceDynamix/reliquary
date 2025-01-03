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

//! Generated file from `CmdAetherDivideType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdAetherDivideType)
pub enum CmdAetherDivideType {
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdAetherDivideTypeNone)
    CmdAetherDivideTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdStartAetherDivideSceneBattleCsReq)
    CmdStartAetherDivideSceneBattleCsReq = 4839,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdSetAetherDivideLineUpCsReq)
    CmdSetAetherDivideLineUpCsReq = 4830,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdStartAetherDivideChallengeBattleScRsp)
    CmdStartAetherDivideChallengeBattleScRsp = 4837,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdAetherDivideLineupScNotify)
    CmdAetherDivideLineupScNotify = 4870,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdEquipAetherDividePassiveSkillScRsp)
    CmdEquipAetherDividePassiveSkillScRsp = 4890,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdAetherDivideFinishChallengeScNotify)
    CmdAetherDivideFinishChallengeScNotify = 4893,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdSetAetherDivideLineUpScRsp)
    CmdSetAetherDivideLineUpScRsp = 4875,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdLeaveAetherDivideSceneScRsp)
    CmdLeaveAetherDivideSceneScRsp = 4846,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdGetAetherDivideChallengeInfoScRsp)
    CmdGetAetherDivideChallengeInfoScRsp = 4809,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdSwitchAetherDivideLineUpSlotCsReq)
    CmdSwitchAetherDivideLineUpSlotCsReq = 4861,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdClearAetherDividePassiveSkillCsReq)
    CmdClearAetherDividePassiveSkillCsReq = 4879,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdAetherDivideSkillItemScNotify)
    CmdAetherDivideSkillItemScNotify = 4894,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdClearAetherDividePassiveSkillScRsp)
    CmdClearAetherDividePassiveSkillScRsp = 4819,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdStartAetherDivideStageBattleCsReq)
    CmdStartAetherDivideStageBattleCsReq = 4829,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdAetherDivideRefreshEndlessScNotify)
    CmdAetherDivideRefreshEndlessScNotify = 4826,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdAetherDivideTainerInfoScNotify)
    CmdAetherDivideTainerInfoScNotify = 4864,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdAetherDivideRefreshEndlessScRsp)
    CmdAetherDivideRefreshEndlessScRsp = 4865,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdAetherDivideRefreshEndlessCsReq)
    CmdAetherDivideRefreshEndlessCsReq = 4836,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdAetherDivideSpiritExpUpScRsp)
    CmdAetherDivideSpiritExpUpScRsp = 4873,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdGetAetherDivideInfoCsReq)
    CmdGetAetherDivideInfoCsReq = 4847,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdSwitchAetherDivideLineUpSlotScRsp)
    CmdSwitchAetherDivideLineUpSlotScRsp = 4825,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdAetherDivideTakeChallengeRewardScRsp)
    CmdAetherDivideTakeChallengeRewardScRsp = 4877,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdStartAetherDivideSceneBattleScRsp)
    CmdStartAetherDivideSceneBattleScRsp = 4853,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdAetherDivideTakeChallengeRewardCsReq)
    CmdAetherDivideTakeChallengeRewardCsReq = 4813,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdGetAetherDivideChallengeInfoCsReq)
    CmdGetAetherDivideChallengeInfoCsReq = 4899,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdAetherDivideSpiritExpUpCsReq)
    CmdAetherDivideSpiritExpUpCsReq = 4851,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdEnterAetherDivideSceneScRsp)
    CmdEnterAetherDivideSceneScRsp = 4820,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdGetAetherDivideInfoScRsp)
    CmdGetAetherDivideInfoScRsp = 4874,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdAetherDivideSpiritInfoScNotify)
    CmdAetherDivideSpiritInfoScNotify = 4883,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdEquipAetherDividePassiveSkillCsReq)
    CmdEquipAetherDividePassiveSkillCsReq = 4848,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdStartAetherDivideStageBattleScRsp)
    CmdStartAetherDivideStageBattleScRsp = 4833,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdLeaveAetherDivideSceneCsReq)
    CmdLeaveAetherDivideSceneCsReq = 4803,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdStartAetherDivideChallengeBattleCsReq)
    CmdStartAetherDivideChallengeBattleCsReq = 4834,
    // @@protoc_insertion_point(enum_value:CmdAetherDivideType.CmdEnterAetherDivideSceneCsReq)
    CmdEnterAetherDivideSceneCsReq = 4859,
}

impl ::protobuf::Enum for CmdAetherDivideType {
    const NAME: &'static str = "CmdAetherDivideType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdAetherDivideType> {
        match value {
            0 => ::std::option::Option::Some(CmdAetherDivideType::CmdAetherDivideTypeNone),
            4839 => ::std::option::Option::Some(CmdAetherDivideType::CmdStartAetherDivideSceneBattleCsReq),
            4830 => ::std::option::Option::Some(CmdAetherDivideType::CmdSetAetherDivideLineUpCsReq),
            4837 => ::std::option::Option::Some(CmdAetherDivideType::CmdStartAetherDivideChallengeBattleScRsp),
            4870 => ::std::option::Option::Some(CmdAetherDivideType::CmdAetherDivideLineupScNotify),
            4890 => ::std::option::Option::Some(CmdAetherDivideType::CmdEquipAetherDividePassiveSkillScRsp),
            4893 => ::std::option::Option::Some(CmdAetherDivideType::CmdAetherDivideFinishChallengeScNotify),
            4875 => ::std::option::Option::Some(CmdAetherDivideType::CmdSetAetherDivideLineUpScRsp),
            4846 => ::std::option::Option::Some(CmdAetherDivideType::CmdLeaveAetherDivideSceneScRsp),
            4809 => ::std::option::Option::Some(CmdAetherDivideType::CmdGetAetherDivideChallengeInfoScRsp),
            4861 => ::std::option::Option::Some(CmdAetherDivideType::CmdSwitchAetherDivideLineUpSlotCsReq),
            4879 => ::std::option::Option::Some(CmdAetherDivideType::CmdClearAetherDividePassiveSkillCsReq),
            4894 => ::std::option::Option::Some(CmdAetherDivideType::CmdAetherDivideSkillItemScNotify),
            4819 => ::std::option::Option::Some(CmdAetherDivideType::CmdClearAetherDividePassiveSkillScRsp),
            4829 => ::std::option::Option::Some(CmdAetherDivideType::CmdStartAetherDivideStageBattleCsReq),
            4826 => ::std::option::Option::Some(CmdAetherDivideType::CmdAetherDivideRefreshEndlessScNotify),
            4864 => ::std::option::Option::Some(CmdAetherDivideType::CmdAetherDivideTainerInfoScNotify),
            4865 => ::std::option::Option::Some(CmdAetherDivideType::CmdAetherDivideRefreshEndlessScRsp),
            4836 => ::std::option::Option::Some(CmdAetherDivideType::CmdAetherDivideRefreshEndlessCsReq),
            4873 => ::std::option::Option::Some(CmdAetherDivideType::CmdAetherDivideSpiritExpUpScRsp),
            4847 => ::std::option::Option::Some(CmdAetherDivideType::CmdGetAetherDivideInfoCsReq),
            4825 => ::std::option::Option::Some(CmdAetherDivideType::CmdSwitchAetherDivideLineUpSlotScRsp),
            4877 => ::std::option::Option::Some(CmdAetherDivideType::CmdAetherDivideTakeChallengeRewardScRsp),
            4853 => ::std::option::Option::Some(CmdAetherDivideType::CmdStartAetherDivideSceneBattleScRsp),
            4813 => ::std::option::Option::Some(CmdAetherDivideType::CmdAetherDivideTakeChallengeRewardCsReq),
            4899 => ::std::option::Option::Some(CmdAetherDivideType::CmdGetAetherDivideChallengeInfoCsReq),
            4851 => ::std::option::Option::Some(CmdAetherDivideType::CmdAetherDivideSpiritExpUpCsReq),
            4820 => ::std::option::Option::Some(CmdAetherDivideType::CmdEnterAetherDivideSceneScRsp),
            4874 => ::std::option::Option::Some(CmdAetherDivideType::CmdGetAetherDivideInfoScRsp),
            4883 => ::std::option::Option::Some(CmdAetherDivideType::CmdAetherDivideSpiritInfoScNotify),
            4848 => ::std::option::Option::Some(CmdAetherDivideType::CmdEquipAetherDividePassiveSkillCsReq),
            4833 => ::std::option::Option::Some(CmdAetherDivideType::CmdStartAetherDivideStageBattleScRsp),
            4803 => ::std::option::Option::Some(CmdAetherDivideType::CmdLeaveAetherDivideSceneCsReq),
            4834 => ::std::option::Option::Some(CmdAetherDivideType::CmdStartAetherDivideChallengeBattleCsReq),
            4859 => ::std::option::Option::Some(CmdAetherDivideType::CmdEnterAetherDivideSceneCsReq),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdAetherDivideType> {
        match str {
            "CmdAetherDivideTypeNone" => ::std::option::Option::Some(CmdAetherDivideType::CmdAetherDivideTypeNone),
            "CmdStartAetherDivideSceneBattleCsReq" => ::std::option::Option::Some(CmdAetherDivideType::CmdStartAetherDivideSceneBattleCsReq),
            "CmdSetAetherDivideLineUpCsReq" => ::std::option::Option::Some(CmdAetherDivideType::CmdSetAetherDivideLineUpCsReq),
            "CmdStartAetherDivideChallengeBattleScRsp" => ::std::option::Option::Some(CmdAetherDivideType::CmdStartAetherDivideChallengeBattleScRsp),
            "CmdAetherDivideLineupScNotify" => ::std::option::Option::Some(CmdAetherDivideType::CmdAetherDivideLineupScNotify),
            "CmdEquipAetherDividePassiveSkillScRsp" => ::std::option::Option::Some(CmdAetherDivideType::CmdEquipAetherDividePassiveSkillScRsp),
            "CmdAetherDivideFinishChallengeScNotify" => ::std::option::Option::Some(CmdAetherDivideType::CmdAetherDivideFinishChallengeScNotify),
            "CmdSetAetherDivideLineUpScRsp" => ::std::option::Option::Some(CmdAetherDivideType::CmdSetAetherDivideLineUpScRsp),
            "CmdLeaveAetherDivideSceneScRsp" => ::std::option::Option::Some(CmdAetherDivideType::CmdLeaveAetherDivideSceneScRsp),
            "CmdGetAetherDivideChallengeInfoScRsp" => ::std::option::Option::Some(CmdAetherDivideType::CmdGetAetherDivideChallengeInfoScRsp),
            "CmdSwitchAetherDivideLineUpSlotCsReq" => ::std::option::Option::Some(CmdAetherDivideType::CmdSwitchAetherDivideLineUpSlotCsReq),
            "CmdClearAetherDividePassiveSkillCsReq" => ::std::option::Option::Some(CmdAetherDivideType::CmdClearAetherDividePassiveSkillCsReq),
            "CmdAetherDivideSkillItemScNotify" => ::std::option::Option::Some(CmdAetherDivideType::CmdAetherDivideSkillItemScNotify),
            "CmdClearAetherDividePassiveSkillScRsp" => ::std::option::Option::Some(CmdAetherDivideType::CmdClearAetherDividePassiveSkillScRsp),
            "CmdStartAetherDivideStageBattleCsReq" => ::std::option::Option::Some(CmdAetherDivideType::CmdStartAetherDivideStageBattleCsReq),
            "CmdAetherDivideRefreshEndlessScNotify" => ::std::option::Option::Some(CmdAetherDivideType::CmdAetherDivideRefreshEndlessScNotify),
            "CmdAetherDivideTainerInfoScNotify" => ::std::option::Option::Some(CmdAetherDivideType::CmdAetherDivideTainerInfoScNotify),
            "CmdAetherDivideRefreshEndlessScRsp" => ::std::option::Option::Some(CmdAetherDivideType::CmdAetherDivideRefreshEndlessScRsp),
            "CmdAetherDivideRefreshEndlessCsReq" => ::std::option::Option::Some(CmdAetherDivideType::CmdAetherDivideRefreshEndlessCsReq),
            "CmdAetherDivideSpiritExpUpScRsp" => ::std::option::Option::Some(CmdAetherDivideType::CmdAetherDivideSpiritExpUpScRsp),
            "CmdGetAetherDivideInfoCsReq" => ::std::option::Option::Some(CmdAetherDivideType::CmdGetAetherDivideInfoCsReq),
            "CmdSwitchAetherDivideLineUpSlotScRsp" => ::std::option::Option::Some(CmdAetherDivideType::CmdSwitchAetherDivideLineUpSlotScRsp),
            "CmdAetherDivideTakeChallengeRewardScRsp" => ::std::option::Option::Some(CmdAetherDivideType::CmdAetherDivideTakeChallengeRewardScRsp),
            "CmdStartAetherDivideSceneBattleScRsp" => ::std::option::Option::Some(CmdAetherDivideType::CmdStartAetherDivideSceneBattleScRsp),
            "CmdAetherDivideTakeChallengeRewardCsReq" => ::std::option::Option::Some(CmdAetherDivideType::CmdAetherDivideTakeChallengeRewardCsReq),
            "CmdGetAetherDivideChallengeInfoCsReq" => ::std::option::Option::Some(CmdAetherDivideType::CmdGetAetherDivideChallengeInfoCsReq),
            "CmdAetherDivideSpiritExpUpCsReq" => ::std::option::Option::Some(CmdAetherDivideType::CmdAetherDivideSpiritExpUpCsReq),
            "CmdEnterAetherDivideSceneScRsp" => ::std::option::Option::Some(CmdAetherDivideType::CmdEnterAetherDivideSceneScRsp),
            "CmdGetAetherDivideInfoScRsp" => ::std::option::Option::Some(CmdAetherDivideType::CmdGetAetherDivideInfoScRsp),
            "CmdAetherDivideSpiritInfoScNotify" => ::std::option::Option::Some(CmdAetherDivideType::CmdAetherDivideSpiritInfoScNotify),
            "CmdEquipAetherDividePassiveSkillCsReq" => ::std::option::Option::Some(CmdAetherDivideType::CmdEquipAetherDividePassiveSkillCsReq),
            "CmdStartAetherDivideStageBattleScRsp" => ::std::option::Option::Some(CmdAetherDivideType::CmdStartAetherDivideStageBattleScRsp),
            "CmdLeaveAetherDivideSceneCsReq" => ::std::option::Option::Some(CmdAetherDivideType::CmdLeaveAetherDivideSceneCsReq),
            "CmdStartAetherDivideChallengeBattleCsReq" => ::std::option::Option::Some(CmdAetherDivideType::CmdStartAetherDivideChallengeBattleCsReq),
            "CmdEnterAetherDivideSceneCsReq" => ::std::option::Option::Some(CmdAetherDivideType::CmdEnterAetherDivideSceneCsReq),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdAetherDivideType] = &[
        CmdAetherDivideType::CmdAetherDivideTypeNone,
        CmdAetherDivideType::CmdStartAetherDivideSceneBattleCsReq,
        CmdAetherDivideType::CmdSetAetherDivideLineUpCsReq,
        CmdAetherDivideType::CmdStartAetherDivideChallengeBattleScRsp,
        CmdAetherDivideType::CmdAetherDivideLineupScNotify,
        CmdAetherDivideType::CmdEquipAetherDividePassiveSkillScRsp,
        CmdAetherDivideType::CmdAetherDivideFinishChallengeScNotify,
        CmdAetherDivideType::CmdSetAetherDivideLineUpScRsp,
        CmdAetherDivideType::CmdLeaveAetherDivideSceneScRsp,
        CmdAetherDivideType::CmdGetAetherDivideChallengeInfoScRsp,
        CmdAetherDivideType::CmdSwitchAetherDivideLineUpSlotCsReq,
        CmdAetherDivideType::CmdClearAetherDividePassiveSkillCsReq,
        CmdAetherDivideType::CmdAetherDivideSkillItemScNotify,
        CmdAetherDivideType::CmdClearAetherDividePassiveSkillScRsp,
        CmdAetherDivideType::CmdStartAetherDivideStageBattleCsReq,
        CmdAetherDivideType::CmdAetherDivideRefreshEndlessScNotify,
        CmdAetherDivideType::CmdAetherDivideTainerInfoScNotify,
        CmdAetherDivideType::CmdAetherDivideRefreshEndlessScRsp,
        CmdAetherDivideType::CmdAetherDivideRefreshEndlessCsReq,
        CmdAetherDivideType::CmdAetherDivideSpiritExpUpScRsp,
        CmdAetherDivideType::CmdGetAetherDivideInfoCsReq,
        CmdAetherDivideType::CmdSwitchAetherDivideLineUpSlotScRsp,
        CmdAetherDivideType::CmdAetherDivideTakeChallengeRewardScRsp,
        CmdAetherDivideType::CmdStartAetherDivideSceneBattleScRsp,
        CmdAetherDivideType::CmdAetherDivideTakeChallengeRewardCsReq,
        CmdAetherDivideType::CmdGetAetherDivideChallengeInfoCsReq,
        CmdAetherDivideType::CmdAetherDivideSpiritExpUpCsReq,
        CmdAetherDivideType::CmdEnterAetherDivideSceneScRsp,
        CmdAetherDivideType::CmdGetAetherDivideInfoScRsp,
        CmdAetherDivideType::CmdAetherDivideSpiritInfoScNotify,
        CmdAetherDivideType::CmdEquipAetherDividePassiveSkillCsReq,
        CmdAetherDivideType::CmdStartAetherDivideStageBattleScRsp,
        CmdAetherDivideType::CmdLeaveAetherDivideSceneCsReq,
        CmdAetherDivideType::CmdStartAetherDivideChallengeBattleCsReq,
        CmdAetherDivideType::CmdEnterAetherDivideSceneCsReq,
    ];
}

impl ::protobuf::EnumFull for CmdAetherDivideType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdAetherDivideType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdAetherDivideType::CmdAetherDivideTypeNone => 0,
            CmdAetherDivideType::CmdStartAetherDivideSceneBattleCsReq => 1,
            CmdAetherDivideType::CmdSetAetherDivideLineUpCsReq => 2,
            CmdAetherDivideType::CmdStartAetherDivideChallengeBattleScRsp => 3,
            CmdAetherDivideType::CmdAetherDivideLineupScNotify => 4,
            CmdAetherDivideType::CmdEquipAetherDividePassiveSkillScRsp => 5,
            CmdAetherDivideType::CmdAetherDivideFinishChallengeScNotify => 6,
            CmdAetherDivideType::CmdSetAetherDivideLineUpScRsp => 7,
            CmdAetherDivideType::CmdLeaveAetherDivideSceneScRsp => 8,
            CmdAetherDivideType::CmdGetAetherDivideChallengeInfoScRsp => 9,
            CmdAetherDivideType::CmdSwitchAetherDivideLineUpSlotCsReq => 10,
            CmdAetherDivideType::CmdClearAetherDividePassiveSkillCsReq => 11,
            CmdAetherDivideType::CmdAetherDivideSkillItemScNotify => 12,
            CmdAetherDivideType::CmdClearAetherDividePassiveSkillScRsp => 13,
            CmdAetherDivideType::CmdStartAetherDivideStageBattleCsReq => 14,
            CmdAetherDivideType::CmdAetherDivideRefreshEndlessScNotify => 15,
            CmdAetherDivideType::CmdAetherDivideTainerInfoScNotify => 16,
            CmdAetherDivideType::CmdAetherDivideRefreshEndlessScRsp => 17,
            CmdAetherDivideType::CmdAetherDivideRefreshEndlessCsReq => 18,
            CmdAetherDivideType::CmdAetherDivideSpiritExpUpScRsp => 19,
            CmdAetherDivideType::CmdGetAetherDivideInfoCsReq => 20,
            CmdAetherDivideType::CmdSwitchAetherDivideLineUpSlotScRsp => 21,
            CmdAetherDivideType::CmdAetherDivideTakeChallengeRewardScRsp => 22,
            CmdAetherDivideType::CmdStartAetherDivideSceneBattleScRsp => 23,
            CmdAetherDivideType::CmdAetherDivideTakeChallengeRewardCsReq => 24,
            CmdAetherDivideType::CmdGetAetherDivideChallengeInfoCsReq => 25,
            CmdAetherDivideType::CmdAetherDivideSpiritExpUpCsReq => 26,
            CmdAetherDivideType::CmdEnterAetherDivideSceneScRsp => 27,
            CmdAetherDivideType::CmdGetAetherDivideInfoScRsp => 28,
            CmdAetherDivideType::CmdAetherDivideSpiritInfoScNotify => 29,
            CmdAetherDivideType::CmdEquipAetherDividePassiveSkillCsReq => 30,
            CmdAetherDivideType::CmdStartAetherDivideStageBattleScRsp => 31,
            CmdAetherDivideType::CmdLeaveAetherDivideSceneCsReq => 32,
            CmdAetherDivideType::CmdStartAetherDivideChallengeBattleCsReq => 33,
            CmdAetherDivideType::CmdEnterAetherDivideSceneCsReq => 34,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdAetherDivideType {
    fn default() -> Self {
        CmdAetherDivideType::CmdAetherDivideTypeNone
    }
}

impl CmdAetherDivideType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdAetherDivideType>("CmdAetherDivideType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x19CmdAetherDivideType.proto*\xa6\x0b\n\x13CmdAetherDivideType\x12\
    \x1b\n\x17CmdAetherDivideTypeNone\x10\0\x12)\n$CmdStartAetherDivideScene\
    BattleCsReq\x10\xe7%\x12\"\n\x1dCmdSetAetherDivideLineUpCsReq\x10\xde%\
    \x12-\n(CmdStartAetherDivideChallengeBattleScRsp\x10\xe5%\x12\"\n\x1dCmd\
    AetherDivideLineupScNotify\x10\x86&\x12*\n%CmdEquipAetherDividePassiveSk\
    illScRsp\x10\x9a&\x12+\n&CmdAetherDivideFinishChallengeScNotify\x10\x9d&\
    \x12\"\n\x1dCmdSetAetherDivideLineUpScRsp\x10\x8b&\x12#\n\x1eCmdLeaveAet\
    herDivideSceneScRsp\x10\xee%\x12)\n$CmdGetAetherDivideChallengeInfoScRsp\
    \x10\xc9%\x12)\n$CmdSwitchAetherDivideLineUpSlotCsReq\x10\xfd%\x12*\n%Cm\
    dClearAetherDividePassiveSkillCsReq\x10\x8f&\x12%\n\x20CmdAetherDivideSk\
    illItemScNotify\x10\x9e&\x12*\n%CmdClearAetherDividePassiveSkillScRsp\
    \x10\xd3%\x12)\n$CmdStartAetherDivideStageBattleCsReq\x10\xdd%\x12*\n%Cm\
    dAetherDivideRefreshEndlessScNotify\x10\xda%\x12&\n!CmdAetherDivideTaine\
    rInfoScNotify\x10\x80&\x12'\n\"CmdAetherDivideRefreshEndlessScRsp\x10\
    \x81&\x12'\n\"CmdAetherDivideRefreshEndlessCsReq\x10\xe4%\x12$\n\x1fCmdA\
    etherDivideSpiritExpUpScRsp\x10\x89&\x12\x20\n\x1bCmdGetAetherDivideInfo\
    CsReq\x10\xef%\x12)\n$CmdSwitchAetherDivideLineUpSlotScRsp\x10\xd9%\x12,\
    \n'CmdAetherDivideTakeChallengeRewardScRsp\x10\x8d&\x12)\n$CmdStartAethe\
    rDivideSceneBattleScRsp\x10\xf5%\x12,\n'CmdAetherDivideTakeChallengeRewa\
    rdCsReq\x10\xcd%\x12)\n$CmdGetAetherDivideChallengeInfoCsReq\x10\xa3&\
    \x12$\n\x1fCmdAetherDivideSpiritExpUpCsReq\x10\xf3%\x12#\n\x1eCmdEnterAe\
    therDivideSceneScRsp\x10\xd4%\x12\x20\n\x1bCmdGetAetherDivideInfoScRsp\
    \x10\x8a&\x12&\n!CmdAetherDivideSpiritInfoScNotify\x10\x93&\x12*\n%CmdEq\
    uipAetherDividePassiveSkillCsReq\x10\xf0%\x12)\n$CmdStartAetherDivideSta\
    geBattleScRsp\x10\xe1%\x12#\n\x1eCmdLeaveAetherDivideSceneCsReq\x10\xc3%\
    \x12-\n(CmdStartAetherDivideChallengeBattleCsReq\x10\xe2%\x12#\n\x1eCmdE\
    nterAetherDivideSceneCsReq\x10\xfb%b\x06proto3\
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
            enums.push(CmdAetherDivideType::generated_enum_descriptor_data());
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
