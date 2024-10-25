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

//! Generated file from `CmdItemType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdItemType)
pub enum CmdItemType {
    // @@protoc_insertion_point(enum_value:CmdItemType.CmdItemTypeNone)
    CmdItemTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdItemType.CmdLockRelicScRsp)
    CmdLockRelicScRsp = 520,
    // @@protoc_insertion_point(enum_value:CmdItemType.CmdComposeSelectedRelicScRsp)
    CmdComposeSelectedRelicScRsp = 510,
    // @@protoc_insertion_point(enum_value:CmdItemType.CmdRankUpEquipmentCsReq)
    CmdRankUpEquipmentCsReq = 528,
    // @@protoc_insertion_point(enum_value:CmdItemType.CmdRankUpEquipmentScRsp)
    CmdRankUpEquipmentScRsp = 538,
    // @@protoc_insertion_point(enum_value:CmdItemType.CmdDestroyItemScRsp)
    CmdDestroyItemScRsp = 514,
    // @@protoc_insertion_point(enum_value:CmdItemType.CmdGetMarkItemListScRsp)
    CmdGetMarkItemListScRsp = 593,
    // @@protoc_insertion_point(enum_value:CmdItemType.CmdExchangeHcoinScRsp)
    CmdExchangeHcoinScRsp = 552,
    // @@protoc_insertion_point(enum_value:CmdItemType.CmdComposeItemScRsp)
    CmdComposeItemScRsp = 522,
    // @@protoc_insertion_point(enum_value:CmdItemType.CmdUseItemScRsp)
    CmdUseItemScRsp = 512,
    // @@protoc_insertion_point(enum_value:CmdItemType.CmdLockEquipmentScRsp)
    CmdLockEquipmentScRsp = 577,
    // @@protoc_insertion_point(enum_value:CmdItemType.CmdExchangeHcoinCsReq)
    CmdExchangeHcoinCsReq = 560,
    // @@protoc_insertion_point(enum_value:CmdItemType.CmdDeleteRelicFilterPlanScRsp)
    CmdDeleteRelicFilterPlanScRsp = 576,
    // @@protoc_insertion_point(enum_value:CmdItemType.CmdRelicFilterPlanClearNameScNotify)
    CmdRelicFilterPlanClearNameScNotify = 503,
    // @@protoc_insertion_point(enum_value:CmdItemType.CmdPromoteEquipmentScRsp)
    CmdPromoteEquipmentScRsp = 542,
    // @@protoc_insertion_point(enum_value:CmdItemType.CmdAddEquipmentScNotify)
    CmdAddEquipmentScNotify = 600,
    // @@protoc_insertion_point(enum_value:CmdItemType.CmdMarkRelicFilterPlanScRsp)
    CmdMarkRelicFilterPlanScRsp = 549,
    // @@protoc_insertion_point(enum_value:CmdItemType.CmdModifyRelicFilterPlanScRsp)
    CmdModifyRelicFilterPlanScRsp = 558,
    // @@protoc_insertion_point(enum_value:CmdItemType.CmdRechargeSuccNotify)
    CmdRechargeSuccNotify = 504,
    // @@protoc_insertion_point(enum_value:CmdItemType.CmdExpUpEquipmentScRsp)
    CmdExpUpEquipmentScRsp = 556,
    // @@protoc_insertion_point(enum_value:CmdItemType.CmdGetRecyleTimeCsReq)
    CmdGetRecyleTimeCsReq = 562,
    // @@protoc_insertion_point(enum_value:CmdItemType.CmdSetTurnFoodSwitchScRsp)
    CmdSetTurnFoodSwitchScRsp = 557,
    // @@protoc_insertion_point(enum_value:CmdItemType.CmdCancelMarkItemNotify)
    CmdCancelMarkItemNotify = 536,
    // @@protoc_insertion_point(enum_value:CmdItemType.CmdComposeSelectedRelicCsReq)
    CmdComposeSelectedRelicCsReq = 511,
    // @@protoc_insertion_point(enum_value:CmdItemType.CmdGetMarkItemListCsReq)
    CmdGetMarkItemListCsReq = 588,
    // @@protoc_insertion_point(enum_value:CmdItemType.CmdComposeLimitNumCompleteNotify)
    CmdComposeLimitNumCompleteNotify = 523,
    // @@protoc_insertion_point(enum_value:CmdItemType.CmdComposeItemCsReq)
    CmdComposeItemCsReq = 589,
    // @@protoc_insertion_point(enum_value:CmdItemType.CmdGeneralVirtualItemDataNotify)
    CmdGeneralVirtualItemDataNotify = 525,
    // @@protoc_insertion_point(enum_value:CmdItemType.CmdGetBagScRsp)
    CmdGetBagScRsp = 571,
    // @@protoc_insertion_point(enum_value:CmdItemType.CmdExpUpRelicCsReq)
    CmdExpUpRelicCsReq = 566,
    // @@protoc_insertion_point(enum_value:CmdItemType.CmdModifyRelicFilterPlanCsReq)
    CmdModifyRelicFilterPlanCsReq = 569,
    // @@protoc_insertion_point(enum_value:CmdItemType.CmdSetTurnFoodSwitchCsReq)
    CmdSetTurnFoodSwitchCsReq = 565,
    // @@protoc_insertion_point(enum_value:CmdItemType.CmdLockRelicCsReq)
    CmdLockRelicCsReq = 545,
    // @@protoc_insertion_point(enum_value:CmdItemType.CmdPromoteEquipmentCsReq)
    CmdPromoteEquipmentCsReq = 583,
    // @@protoc_insertion_point(enum_value:CmdItemType.CmdAddRelicFilterPlanScRsp)
    CmdAddRelicFilterPlanScRsp = 554,
    // @@protoc_insertion_point(enum_value:CmdItemType.CmdComposeLimitNumUpdateNotify)
    CmdComposeLimitNumUpdateNotify = 585,
    // @@protoc_insertion_point(enum_value:CmdItemType.CmdSellItemScRsp)
    CmdSellItemScRsp = 531,
    // @@protoc_insertion_point(enum_value:CmdItemType.CmdGetRelicFilterPlanCsReq)
    CmdGetRelicFilterPlanCsReq = 564,
    // @@protoc_insertion_point(enum_value:CmdItemType.CmdAddRelicFilterPlanCsReq)
    CmdAddRelicFilterPlanCsReq = 587,
    // @@protoc_insertion_point(enum_value:CmdItemType.CmdExpUpEquipmentCsReq)
    CmdExpUpEquipmentCsReq = 578,
    // @@protoc_insertion_point(enum_value:CmdItemType.CmdLockEquipmentCsReq)
    CmdLockEquipmentCsReq = 579,
    // @@protoc_insertion_point(enum_value:CmdItemType.CmdMarkItemCsReq)
    CmdMarkItemCsReq = 563,
    // @@protoc_insertion_point(enum_value:CmdItemType.CmdDestroyItemCsReq)
    CmdDestroyItemCsReq = 516,
    // @@protoc_insertion_point(enum_value:CmdItemType.CmdGetBagCsReq)
    CmdGetBagCsReq = 598,
    // @@protoc_insertion_point(enum_value:CmdItemType.CmdUseItemCsReq)
    CmdUseItemCsReq = 533,
    // @@protoc_insertion_point(enum_value:CmdItemType.CmdMarkItemScRsp)
    CmdMarkItemScRsp = 555,
    // @@protoc_insertion_point(enum_value:CmdItemType.CmdExpUpRelicScRsp)
    CmdExpUpRelicScRsp = 505,
    // @@protoc_insertion_point(enum_value:CmdItemType.CmdGetRecyleTimeScRsp)
    CmdGetRecyleTimeScRsp = 506,
    // @@protoc_insertion_point(enum_value:CmdItemType.CmdMarkRelicFilterPlanCsReq)
    CmdMarkRelicFilterPlanCsReq = 539,
    // @@protoc_insertion_point(enum_value:CmdItemType.CmdSyncTurnFoodNotify)
    CmdSyncTurnFoodNotify = 502,
    // @@protoc_insertion_point(enum_value:CmdItemType.CmdSellItemCsReq)
    CmdSellItemCsReq = 550,
    // @@protoc_insertion_point(enum_value:CmdItemType.CmdDiscardRelicCsReq)
    CmdDiscardRelicCsReq = 515,
    // @@protoc_insertion_point(enum_value:CmdItemType.CmdGetRelicFilterPlanScRsp)
    CmdGetRelicFilterPlanScRsp = 535,
    // @@protoc_insertion_point(enum_value:CmdItemType.CmdDiscardRelicScRsp)
    CmdDiscardRelicScRsp = 519,
    // @@protoc_insertion_point(enum_value:CmdItemType.CmdDeleteRelicFilterPlanCsReq)
    CmdDeleteRelicFilterPlanCsReq = 567,
}

impl ::protobuf::Enum for CmdItemType {
    const NAME: &'static str = "CmdItemType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdItemType> {
        match value {
            0 => ::std::option::Option::Some(CmdItemType::CmdItemTypeNone),
            520 => ::std::option::Option::Some(CmdItemType::CmdLockRelicScRsp),
            510 => ::std::option::Option::Some(CmdItemType::CmdComposeSelectedRelicScRsp),
            528 => ::std::option::Option::Some(CmdItemType::CmdRankUpEquipmentCsReq),
            538 => ::std::option::Option::Some(CmdItemType::CmdRankUpEquipmentScRsp),
            514 => ::std::option::Option::Some(CmdItemType::CmdDestroyItemScRsp),
            593 => ::std::option::Option::Some(CmdItemType::CmdGetMarkItemListScRsp),
            552 => ::std::option::Option::Some(CmdItemType::CmdExchangeHcoinScRsp),
            522 => ::std::option::Option::Some(CmdItemType::CmdComposeItemScRsp),
            512 => ::std::option::Option::Some(CmdItemType::CmdUseItemScRsp),
            577 => ::std::option::Option::Some(CmdItemType::CmdLockEquipmentScRsp),
            560 => ::std::option::Option::Some(CmdItemType::CmdExchangeHcoinCsReq),
            576 => ::std::option::Option::Some(CmdItemType::CmdDeleteRelicFilterPlanScRsp),
            503 => ::std::option::Option::Some(CmdItemType::CmdRelicFilterPlanClearNameScNotify),
            542 => ::std::option::Option::Some(CmdItemType::CmdPromoteEquipmentScRsp),
            600 => ::std::option::Option::Some(CmdItemType::CmdAddEquipmentScNotify),
            549 => ::std::option::Option::Some(CmdItemType::CmdMarkRelicFilterPlanScRsp),
            558 => ::std::option::Option::Some(CmdItemType::CmdModifyRelicFilterPlanScRsp),
            504 => ::std::option::Option::Some(CmdItemType::CmdRechargeSuccNotify),
            556 => ::std::option::Option::Some(CmdItemType::CmdExpUpEquipmentScRsp),
            562 => ::std::option::Option::Some(CmdItemType::CmdGetRecyleTimeCsReq),
            557 => ::std::option::Option::Some(CmdItemType::CmdSetTurnFoodSwitchScRsp),
            536 => ::std::option::Option::Some(CmdItemType::CmdCancelMarkItemNotify),
            511 => ::std::option::Option::Some(CmdItemType::CmdComposeSelectedRelicCsReq),
            588 => ::std::option::Option::Some(CmdItemType::CmdGetMarkItemListCsReq),
            523 => ::std::option::Option::Some(CmdItemType::CmdComposeLimitNumCompleteNotify),
            589 => ::std::option::Option::Some(CmdItemType::CmdComposeItemCsReq),
            525 => ::std::option::Option::Some(CmdItemType::CmdGeneralVirtualItemDataNotify),
            571 => ::std::option::Option::Some(CmdItemType::CmdGetBagScRsp),
            566 => ::std::option::Option::Some(CmdItemType::CmdExpUpRelicCsReq),
            569 => ::std::option::Option::Some(CmdItemType::CmdModifyRelicFilterPlanCsReq),
            565 => ::std::option::Option::Some(CmdItemType::CmdSetTurnFoodSwitchCsReq),
            545 => ::std::option::Option::Some(CmdItemType::CmdLockRelicCsReq),
            583 => ::std::option::Option::Some(CmdItemType::CmdPromoteEquipmentCsReq),
            554 => ::std::option::Option::Some(CmdItemType::CmdAddRelicFilterPlanScRsp),
            585 => ::std::option::Option::Some(CmdItemType::CmdComposeLimitNumUpdateNotify),
            531 => ::std::option::Option::Some(CmdItemType::CmdSellItemScRsp),
            564 => ::std::option::Option::Some(CmdItemType::CmdGetRelicFilterPlanCsReq),
            587 => ::std::option::Option::Some(CmdItemType::CmdAddRelicFilterPlanCsReq),
            578 => ::std::option::Option::Some(CmdItemType::CmdExpUpEquipmentCsReq),
            579 => ::std::option::Option::Some(CmdItemType::CmdLockEquipmentCsReq),
            563 => ::std::option::Option::Some(CmdItemType::CmdMarkItemCsReq),
            516 => ::std::option::Option::Some(CmdItemType::CmdDestroyItemCsReq),
            598 => ::std::option::Option::Some(CmdItemType::CmdGetBagCsReq),
            533 => ::std::option::Option::Some(CmdItemType::CmdUseItemCsReq),
            555 => ::std::option::Option::Some(CmdItemType::CmdMarkItemScRsp),
            505 => ::std::option::Option::Some(CmdItemType::CmdExpUpRelicScRsp),
            506 => ::std::option::Option::Some(CmdItemType::CmdGetRecyleTimeScRsp),
            539 => ::std::option::Option::Some(CmdItemType::CmdMarkRelicFilterPlanCsReq),
            502 => ::std::option::Option::Some(CmdItemType::CmdSyncTurnFoodNotify),
            550 => ::std::option::Option::Some(CmdItemType::CmdSellItemCsReq),
            515 => ::std::option::Option::Some(CmdItemType::CmdDiscardRelicCsReq),
            535 => ::std::option::Option::Some(CmdItemType::CmdGetRelicFilterPlanScRsp),
            519 => ::std::option::Option::Some(CmdItemType::CmdDiscardRelicScRsp),
            567 => ::std::option::Option::Some(CmdItemType::CmdDeleteRelicFilterPlanCsReq),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdItemType> {
        match str {
            "CmdItemTypeNone" => ::std::option::Option::Some(CmdItemType::CmdItemTypeNone),
            "CmdLockRelicScRsp" => ::std::option::Option::Some(CmdItemType::CmdLockRelicScRsp),
            "CmdComposeSelectedRelicScRsp" => ::std::option::Option::Some(CmdItemType::CmdComposeSelectedRelicScRsp),
            "CmdRankUpEquipmentCsReq" => ::std::option::Option::Some(CmdItemType::CmdRankUpEquipmentCsReq),
            "CmdRankUpEquipmentScRsp" => ::std::option::Option::Some(CmdItemType::CmdRankUpEquipmentScRsp),
            "CmdDestroyItemScRsp" => ::std::option::Option::Some(CmdItemType::CmdDestroyItemScRsp),
            "CmdGetMarkItemListScRsp" => ::std::option::Option::Some(CmdItemType::CmdGetMarkItemListScRsp),
            "CmdExchangeHcoinScRsp" => ::std::option::Option::Some(CmdItemType::CmdExchangeHcoinScRsp),
            "CmdComposeItemScRsp" => ::std::option::Option::Some(CmdItemType::CmdComposeItemScRsp),
            "CmdUseItemScRsp" => ::std::option::Option::Some(CmdItemType::CmdUseItemScRsp),
            "CmdLockEquipmentScRsp" => ::std::option::Option::Some(CmdItemType::CmdLockEquipmentScRsp),
            "CmdExchangeHcoinCsReq" => ::std::option::Option::Some(CmdItemType::CmdExchangeHcoinCsReq),
            "CmdDeleteRelicFilterPlanScRsp" => ::std::option::Option::Some(CmdItemType::CmdDeleteRelicFilterPlanScRsp),
            "CmdRelicFilterPlanClearNameScNotify" => ::std::option::Option::Some(CmdItemType::CmdRelicFilterPlanClearNameScNotify),
            "CmdPromoteEquipmentScRsp" => ::std::option::Option::Some(CmdItemType::CmdPromoteEquipmentScRsp),
            "CmdAddEquipmentScNotify" => ::std::option::Option::Some(CmdItemType::CmdAddEquipmentScNotify),
            "CmdMarkRelicFilterPlanScRsp" => ::std::option::Option::Some(CmdItemType::CmdMarkRelicFilterPlanScRsp),
            "CmdModifyRelicFilterPlanScRsp" => ::std::option::Option::Some(CmdItemType::CmdModifyRelicFilterPlanScRsp),
            "CmdRechargeSuccNotify" => ::std::option::Option::Some(CmdItemType::CmdRechargeSuccNotify),
            "CmdExpUpEquipmentScRsp" => ::std::option::Option::Some(CmdItemType::CmdExpUpEquipmentScRsp),
            "CmdGetRecyleTimeCsReq" => ::std::option::Option::Some(CmdItemType::CmdGetRecyleTimeCsReq),
            "CmdSetTurnFoodSwitchScRsp" => ::std::option::Option::Some(CmdItemType::CmdSetTurnFoodSwitchScRsp),
            "CmdCancelMarkItemNotify" => ::std::option::Option::Some(CmdItemType::CmdCancelMarkItemNotify),
            "CmdComposeSelectedRelicCsReq" => ::std::option::Option::Some(CmdItemType::CmdComposeSelectedRelicCsReq),
            "CmdGetMarkItemListCsReq" => ::std::option::Option::Some(CmdItemType::CmdGetMarkItemListCsReq),
            "CmdComposeLimitNumCompleteNotify" => ::std::option::Option::Some(CmdItemType::CmdComposeLimitNumCompleteNotify),
            "CmdComposeItemCsReq" => ::std::option::Option::Some(CmdItemType::CmdComposeItemCsReq),
            "CmdGeneralVirtualItemDataNotify" => ::std::option::Option::Some(CmdItemType::CmdGeneralVirtualItemDataNotify),
            "CmdGetBagScRsp" => ::std::option::Option::Some(CmdItemType::CmdGetBagScRsp),
            "CmdExpUpRelicCsReq" => ::std::option::Option::Some(CmdItemType::CmdExpUpRelicCsReq),
            "CmdModifyRelicFilterPlanCsReq" => ::std::option::Option::Some(CmdItemType::CmdModifyRelicFilterPlanCsReq),
            "CmdSetTurnFoodSwitchCsReq" => ::std::option::Option::Some(CmdItemType::CmdSetTurnFoodSwitchCsReq),
            "CmdLockRelicCsReq" => ::std::option::Option::Some(CmdItemType::CmdLockRelicCsReq),
            "CmdPromoteEquipmentCsReq" => ::std::option::Option::Some(CmdItemType::CmdPromoteEquipmentCsReq),
            "CmdAddRelicFilterPlanScRsp" => ::std::option::Option::Some(CmdItemType::CmdAddRelicFilterPlanScRsp),
            "CmdComposeLimitNumUpdateNotify" => ::std::option::Option::Some(CmdItemType::CmdComposeLimitNumUpdateNotify),
            "CmdSellItemScRsp" => ::std::option::Option::Some(CmdItemType::CmdSellItemScRsp),
            "CmdGetRelicFilterPlanCsReq" => ::std::option::Option::Some(CmdItemType::CmdGetRelicFilterPlanCsReq),
            "CmdAddRelicFilterPlanCsReq" => ::std::option::Option::Some(CmdItemType::CmdAddRelicFilterPlanCsReq),
            "CmdExpUpEquipmentCsReq" => ::std::option::Option::Some(CmdItemType::CmdExpUpEquipmentCsReq),
            "CmdLockEquipmentCsReq" => ::std::option::Option::Some(CmdItemType::CmdLockEquipmentCsReq),
            "CmdMarkItemCsReq" => ::std::option::Option::Some(CmdItemType::CmdMarkItemCsReq),
            "CmdDestroyItemCsReq" => ::std::option::Option::Some(CmdItemType::CmdDestroyItemCsReq),
            "CmdGetBagCsReq" => ::std::option::Option::Some(CmdItemType::CmdGetBagCsReq),
            "CmdUseItemCsReq" => ::std::option::Option::Some(CmdItemType::CmdUseItemCsReq),
            "CmdMarkItemScRsp" => ::std::option::Option::Some(CmdItemType::CmdMarkItemScRsp),
            "CmdExpUpRelicScRsp" => ::std::option::Option::Some(CmdItemType::CmdExpUpRelicScRsp),
            "CmdGetRecyleTimeScRsp" => ::std::option::Option::Some(CmdItemType::CmdGetRecyleTimeScRsp),
            "CmdMarkRelicFilterPlanCsReq" => ::std::option::Option::Some(CmdItemType::CmdMarkRelicFilterPlanCsReq),
            "CmdSyncTurnFoodNotify" => ::std::option::Option::Some(CmdItemType::CmdSyncTurnFoodNotify),
            "CmdSellItemCsReq" => ::std::option::Option::Some(CmdItemType::CmdSellItemCsReq),
            "CmdDiscardRelicCsReq" => ::std::option::Option::Some(CmdItemType::CmdDiscardRelicCsReq),
            "CmdGetRelicFilterPlanScRsp" => ::std::option::Option::Some(CmdItemType::CmdGetRelicFilterPlanScRsp),
            "CmdDiscardRelicScRsp" => ::std::option::Option::Some(CmdItemType::CmdDiscardRelicScRsp),
            "CmdDeleteRelicFilterPlanCsReq" => ::std::option::Option::Some(CmdItemType::CmdDeleteRelicFilterPlanCsReq),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdItemType] = &[
        CmdItemType::CmdItemTypeNone,
        CmdItemType::CmdLockRelicScRsp,
        CmdItemType::CmdComposeSelectedRelicScRsp,
        CmdItemType::CmdRankUpEquipmentCsReq,
        CmdItemType::CmdRankUpEquipmentScRsp,
        CmdItemType::CmdDestroyItemScRsp,
        CmdItemType::CmdGetMarkItemListScRsp,
        CmdItemType::CmdExchangeHcoinScRsp,
        CmdItemType::CmdComposeItemScRsp,
        CmdItemType::CmdUseItemScRsp,
        CmdItemType::CmdLockEquipmentScRsp,
        CmdItemType::CmdExchangeHcoinCsReq,
        CmdItemType::CmdDeleteRelicFilterPlanScRsp,
        CmdItemType::CmdRelicFilterPlanClearNameScNotify,
        CmdItemType::CmdPromoteEquipmentScRsp,
        CmdItemType::CmdAddEquipmentScNotify,
        CmdItemType::CmdMarkRelicFilterPlanScRsp,
        CmdItemType::CmdModifyRelicFilterPlanScRsp,
        CmdItemType::CmdRechargeSuccNotify,
        CmdItemType::CmdExpUpEquipmentScRsp,
        CmdItemType::CmdGetRecyleTimeCsReq,
        CmdItemType::CmdSetTurnFoodSwitchScRsp,
        CmdItemType::CmdCancelMarkItemNotify,
        CmdItemType::CmdComposeSelectedRelicCsReq,
        CmdItemType::CmdGetMarkItemListCsReq,
        CmdItemType::CmdComposeLimitNumCompleteNotify,
        CmdItemType::CmdComposeItemCsReq,
        CmdItemType::CmdGeneralVirtualItemDataNotify,
        CmdItemType::CmdGetBagScRsp,
        CmdItemType::CmdExpUpRelicCsReq,
        CmdItemType::CmdModifyRelicFilterPlanCsReq,
        CmdItemType::CmdSetTurnFoodSwitchCsReq,
        CmdItemType::CmdLockRelicCsReq,
        CmdItemType::CmdPromoteEquipmentCsReq,
        CmdItemType::CmdAddRelicFilterPlanScRsp,
        CmdItemType::CmdComposeLimitNumUpdateNotify,
        CmdItemType::CmdSellItemScRsp,
        CmdItemType::CmdGetRelicFilterPlanCsReq,
        CmdItemType::CmdAddRelicFilterPlanCsReq,
        CmdItemType::CmdExpUpEquipmentCsReq,
        CmdItemType::CmdLockEquipmentCsReq,
        CmdItemType::CmdMarkItemCsReq,
        CmdItemType::CmdDestroyItemCsReq,
        CmdItemType::CmdGetBagCsReq,
        CmdItemType::CmdUseItemCsReq,
        CmdItemType::CmdMarkItemScRsp,
        CmdItemType::CmdExpUpRelicScRsp,
        CmdItemType::CmdGetRecyleTimeScRsp,
        CmdItemType::CmdMarkRelicFilterPlanCsReq,
        CmdItemType::CmdSyncTurnFoodNotify,
        CmdItemType::CmdSellItemCsReq,
        CmdItemType::CmdDiscardRelicCsReq,
        CmdItemType::CmdGetRelicFilterPlanScRsp,
        CmdItemType::CmdDiscardRelicScRsp,
        CmdItemType::CmdDeleteRelicFilterPlanCsReq,
    ];
}

impl ::protobuf::EnumFull for CmdItemType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdItemType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdItemType::CmdItemTypeNone => 0,
            CmdItemType::CmdLockRelicScRsp => 1,
            CmdItemType::CmdComposeSelectedRelicScRsp => 2,
            CmdItemType::CmdRankUpEquipmentCsReq => 3,
            CmdItemType::CmdRankUpEquipmentScRsp => 4,
            CmdItemType::CmdDestroyItemScRsp => 5,
            CmdItemType::CmdGetMarkItemListScRsp => 6,
            CmdItemType::CmdExchangeHcoinScRsp => 7,
            CmdItemType::CmdComposeItemScRsp => 8,
            CmdItemType::CmdUseItemScRsp => 9,
            CmdItemType::CmdLockEquipmentScRsp => 10,
            CmdItemType::CmdExchangeHcoinCsReq => 11,
            CmdItemType::CmdDeleteRelicFilterPlanScRsp => 12,
            CmdItemType::CmdRelicFilterPlanClearNameScNotify => 13,
            CmdItemType::CmdPromoteEquipmentScRsp => 14,
            CmdItemType::CmdAddEquipmentScNotify => 15,
            CmdItemType::CmdMarkRelicFilterPlanScRsp => 16,
            CmdItemType::CmdModifyRelicFilterPlanScRsp => 17,
            CmdItemType::CmdRechargeSuccNotify => 18,
            CmdItemType::CmdExpUpEquipmentScRsp => 19,
            CmdItemType::CmdGetRecyleTimeCsReq => 20,
            CmdItemType::CmdSetTurnFoodSwitchScRsp => 21,
            CmdItemType::CmdCancelMarkItemNotify => 22,
            CmdItemType::CmdComposeSelectedRelicCsReq => 23,
            CmdItemType::CmdGetMarkItemListCsReq => 24,
            CmdItemType::CmdComposeLimitNumCompleteNotify => 25,
            CmdItemType::CmdComposeItemCsReq => 26,
            CmdItemType::CmdGeneralVirtualItemDataNotify => 27,
            CmdItemType::CmdGetBagScRsp => 28,
            CmdItemType::CmdExpUpRelicCsReq => 29,
            CmdItemType::CmdModifyRelicFilterPlanCsReq => 30,
            CmdItemType::CmdSetTurnFoodSwitchCsReq => 31,
            CmdItemType::CmdLockRelicCsReq => 32,
            CmdItemType::CmdPromoteEquipmentCsReq => 33,
            CmdItemType::CmdAddRelicFilterPlanScRsp => 34,
            CmdItemType::CmdComposeLimitNumUpdateNotify => 35,
            CmdItemType::CmdSellItemScRsp => 36,
            CmdItemType::CmdGetRelicFilterPlanCsReq => 37,
            CmdItemType::CmdAddRelicFilterPlanCsReq => 38,
            CmdItemType::CmdExpUpEquipmentCsReq => 39,
            CmdItemType::CmdLockEquipmentCsReq => 40,
            CmdItemType::CmdMarkItemCsReq => 41,
            CmdItemType::CmdDestroyItemCsReq => 42,
            CmdItemType::CmdGetBagCsReq => 43,
            CmdItemType::CmdUseItemCsReq => 44,
            CmdItemType::CmdMarkItemScRsp => 45,
            CmdItemType::CmdExpUpRelicScRsp => 46,
            CmdItemType::CmdGetRecyleTimeScRsp => 47,
            CmdItemType::CmdMarkRelicFilterPlanCsReq => 48,
            CmdItemType::CmdSyncTurnFoodNotify => 49,
            CmdItemType::CmdSellItemCsReq => 50,
            CmdItemType::CmdDiscardRelicCsReq => 51,
            CmdItemType::CmdGetRelicFilterPlanScRsp => 52,
            CmdItemType::CmdDiscardRelicScRsp => 53,
            CmdItemType::CmdDeleteRelicFilterPlanCsReq => 54,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdItemType {
    fn default() -> Self {
        CmdItemType::CmdItemTypeNone
    }
}

impl CmdItemType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdItemType>("CmdItemType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11CmdItemType.proto*\xda\x0c\n\x0bCmdItemType\x12\x13\n\x0fCmdItemTy\
    peNone\x10\0\x12\x16\n\x11CmdLockRelicScRsp\x10\x88\x04\x12!\n\x1cCmdCom\
    poseSelectedRelicScRsp\x10\xfe\x03\x12\x1c\n\x17CmdRankUpEquipmentCsReq\
    \x10\x90\x04\x12\x1c\n\x17CmdRankUpEquipmentScRsp\x10\x9a\x04\x12\x18\n\
    \x13CmdDestroyItemScRsp\x10\x82\x04\x12\x1c\n\x17CmdGetMarkItemListScRsp\
    \x10\xd1\x04\x12\x1a\n\x15CmdExchangeHcoinScRsp\x10\xa8\x04\x12\x18\n\
    \x13CmdComposeItemScRsp\x10\x8a\x04\x12\x14\n\x0fCmdUseItemScRsp\x10\x80\
    \x04\x12\x1a\n\x15CmdLockEquipmentScRsp\x10\xc1\x04\x12\x1a\n\x15CmdExch\
    angeHcoinCsReq\x10\xb0\x04\x12\"\n\x1dCmdDeleteRelicFilterPlanScRsp\x10\
    \xc0\x04\x12(\n#CmdRelicFilterPlanClearNameScNotify\x10\xf7\x03\x12\x1d\
    \n\x18CmdPromoteEquipmentScRsp\x10\x9e\x04\x12\x1c\n\x17CmdAddEquipmentS\
    cNotify\x10\xd8\x04\x12\x20\n\x1bCmdMarkRelicFilterPlanScRsp\x10\xa5\x04\
    \x12\"\n\x1dCmdModifyRelicFilterPlanScRsp\x10\xae\x04\x12\x1a\n\x15CmdRe\
    chargeSuccNotify\x10\xf8\x03\x12\x1b\n\x16CmdExpUpEquipmentScRsp\x10\xac\
    \x04\x12\x1a\n\x15CmdGetRecyleTimeCsReq\x10\xb2\x04\x12\x1e\n\x19CmdSetT\
    urnFoodSwitchScRsp\x10\xad\x04\x12\x1c\n\x17CmdCancelMarkItemNotify\x10\
    \x98\x04\x12!\n\x1cCmdComposeSelectedRelicCsReq\x10\xff\x03\x12\x1c\n\
    \x17CmdGetMarkItemListCsReq\x10\xcc\x04\x12%\n\x20CmdComposeLimitNumComp\
    leteNotify\x10\x8b\x04\x12\x18\n\x13CmdComposeItemCsReq\x10\xcd\x04\x12$\
    \n\x1fCmdGeneralVirtualItemDataNotify\x10\x8d\x04\x12\x13\n\x0eCmdGetBag\
    ScRsp\x10\xbb\x04\x12\x17\n\x12CmdExpUpRelicCsReq\x10\xb6\x04\x12\"\n\
    \x1dCmdModifyRelicFilterPlanCsReq\x10\xb9\x04\x12\x1e\n\x19CmdSetTurnFoo\
    dSwitchCsReq\x10\xb5\x04\x12\x16\n\x11CmdLockRelicCsReq\x10\xa1\x04\x12\
    \x1d\n\x18CmdPromoteEquipmentCsReq\x10\xc7\x04\x12\x1f\n\x1aCmdAddRelicF\
    ilterPlanScRsp\x10\xaa\x04\x12#\n\x1eCmdComposeLimitNumUpdateNotify\x10\
    \xc9\x04\x12\x15\n\x10CmdSellItemScRsp\x10\x93\x04\x12\x1f\n\x1aCmdGetRe\
    licFilterPlanCsReq\x10\xb4\x04\x12\x1f\n\x1aCmdAddRelicFilterPlanCsReq\
    \x10\xcb\x04\x12\x1b\n\x16CmdExpUpEquipmentCsReq\x10\xc2\x04\x12\x1a\n\
    \x15CmdLockEquipmentCsReq\x10\xc3\x04\x12\x15\n\x10CmdMarkItemCsReq\x10\
    \xb3\x04\x12\x18\n\x13CmdDestroyItemCsReq\x10\x84\x04\x12\x13\n\x0eCmdGe\
    tBagCsReq\x10\xd6\x04\x12\x14\n\x0fCmdUseItemCsReq\x10\x95\x04\x12\x15\n\
    \x10CmdMarkItemScRsp\x10\xab\x04\x12\x17\n\x12CmdExpUpRelicScRsp\x10\xf9\
    \x03\x12\x1a\n\x15CmdGetRecyleTimeScRsp\x10\xfa\x03\x12\x20\n\x1bCmdMark\
    RelicFilterPlanCsReq\x10\x9b\x04\x12\x1a\n\x15CmdSyncTurnFoodNotify\x10\
    \xf6\x03\x12\x15\n\x10CmdSellItemCsReq\x10\xa6\x04\x12\x19\n\x14CmdDisca\
    rdRelicCsReq\x10\x83\x04\x12\x1f\n\x1aCmdGetRelicFilterPlanScRsp\x10\x97\
    \x04\x12\x19\n\x14CmdDiscardRelicScRsp\x10\x87\x04\x12\"\n\x1dCmdDeleteR\
    elicFilterPlanCsReq\x10\xb7\x04b\x06proto3\
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
            enums.push(CmdItemType::generated_enum_descriptor_data());
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
