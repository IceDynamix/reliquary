#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

//! Parse config files from SRData ExcelOutput
//!
//! All `...Config.json` files have a `...Config` struct and a `...ConfigMap` struct counterpart.
//! The `...Config` struct represents a single map entry, while the `...ConfigMap` struct represents the
//! (nested) map all config entries are stored in, in the .json file.
//! Both types can be deserialized into.
//!
//! The `...Config` structs are written manually right now.
//! It might be possible to automate this process.
//!
//! The `...ConfigMap` structs are automatically generated using a macro.
//! They contain a `get` method to get a specific entry from the (nested) map
//! with the specified keys.
//!
//! Floats and text-map entries are stored in a special way.
//! Please refer to [`Float`] and [`TextMapEntry`].
//!
//! ## Example
//!
//! Using RelicMainAffixConfig as an example
//!
//! ```no_run
//! use reliquary::resource::excel::RelicMainAffixConfigMap;
//! let content = "";
//! let config_map: RelicMainAffixConfigMap = serde_json::from_str(&content).unwrap();
//! let config = config_map.get(&0, &1).unwrap(); // GroupID, AffixID
//! ```

use serde::Deserialize;

use reliquary_proc_macro::Resource;

use crate::resource::{Float, TextMapEntry, UnhashedTextMapEntry};

#[derive(Deserialize, Debug)]
pub struct ItemQuantity {
    pub ItemID: u32,
    pub ItemNum: u32,
}

#[derive(Resource, Deserialize, Debug)]
pub struct AvatarConfig {
    #[resource_key]
    pub AvatarID: u32,
    pub AvatarName: TextMapEntry,
    pub AvatarFullName: TextMapEntry,
    pub AdventurePlayerID: u32,
    pub AvatarVOTag: String,
    pub Rarity: String,
    pub AvatarInitialSkinName: TextMapEntry,
    pub AvatarInitialSkinDesc: TextMapEntry,
    pub JsonPath: String,
    pub DamageType: String,
    pub SPNeed: Float,
    pub ExpGroup: u32,
    pub MaxPromotion: u32,
    pub MaxRank: u32,
    pub RankIDList: Vec<u32>,
    pub RewardList: Vec<ItemQuantity>,
    pub RewardListMax: Vec<ItemQuantity>,
    pub SkillList: Vec<u32>,
    pub AvatarBaseType: String,
    pub DefaultAvatarModelPath: String,
    pub DefaultAvatarHeadIconPath: String,
    pub AvatarSideIconPath: String,
    pub AvatarMiniIconPath: String,
    pub AvatarGachaResultImgPath: String,
    pub ActionAvatarHeadIconPath: String,
    pub UltraSkillCutInPrefabPath: String,
    pub UIAvatarModelPath: String,
    pub ManikinJsonPath: String,
    pub AvatarDesc: TextMapEntry,
    pub AIPath: String,
    pub SkilltreePrefabPath: String,
    pub DamageTypeResistance: Vec<()>,
    #[serde(default)]
    pub Release: bool,
    pub SideAvatarHeadIconPath: String,
    pub WaitingAvatarHeadIconPath: String,
    pub AvatarCutinImgPath: String,
    pub AvatarCutinBgImgPath: String,
    pub AvatarCutinFrontImgPath: String,
    pub AvatarCutinIntroText: TextMapEntry,
    pub AvatarDropOffset: Vec<f32>,
    pub AvatarTrialOffset: Vec<f32>,
    pub PlayerCardOffset: Vec<f32>,
    pub AssistOffset: Vec<f32>,
    pub AssistBgOffset: Vec<f32>,
    pub AvatarSelfShowOffset: Vec<f32>,
}

#[derive(Resource, Deserialize, Debug)]
pub struct EquipmentConfig {
    #[resource_key]
    pub EquipmentID: u32,
    #[serde(default)]
    pub Release: bool,
    pub EquipmentName: TextMapEntry,
    pub EquipmentDesc: TextMapEntry,
    pub Rarity: String,
    pub AvatarBaseType: String,
    pub MaxPromotion: u32,
    pub MaxRank: u32,
    pub ExpType: u32,
    pub SkillID: u32,
    pub ExpProvide: u32,
    pub CoinCost: u32,
    pub RankUpCostList: Vec<u32>,
    pub ThumbnailPath: String,
    pub ImagePath: String,
    pub ItemRightPanelOffset: Vec<f32>,
    pub AvatarDetailOffset: Vec<f32>,
    pub BattleDialogOffset: Vec<f32>,
    pub GachaResultOffset: Vec<f32>,
}

#[derive(Resource, Deserialize, Debug)]
pub struct RelicMainAffixConfig {
    #[resource_key]
    pub GroupID: u32,
    #[resource_key]
    pub AffixID: u32,
    pub Property: String,
    pub BaseValue: Float,
    pub LevelAdd: Float,
    pub IsAvailable: bool,
}

#[derive(Resource, Deserialize, Debug)]
pub struct RelicSetConfig {
    #[resource_key]
    pub SetID: u32,
    pub SetSkillList: Vec<u32>,
    pub SetIconPath: String,
    pub SetIconFigurePath: String,
    pub SetName: TextMapEntry,
    pub DisplayItemID: u32,
    #[serde(default)]
    pub Release: bool,
}

#[derive(Resource, Deserialize, Debug)]
pub struct RelicSubAffixConfig {
    #[resource_key]
    pub GroupID: u32,
    #[resource_key]
    pub AffixID: u32,
    pub Property: String,
    pub BaseValue: Float,
    pub StepValue: Float,
    pub StepNum: u32,
}

#[derive(Resource, Deserialize, Debug)]
pub struct RelicConfig {
    #[resource_key]
    pub ID: u32,
    pub SetID: u32,
    pub Type: String,
    pub Rarity: String,
    pub MainAffixGroup: u32,
    pub SubAffixGroup: u32,
    pub MaxLevel: u32,
    pub ExpType: u32,
    pub ExpProvide: u32,
    pub CoinCost: u32,
}

#[derive(Resource, Deserialize, Debug)]
pub struct RelicDataConfig {
    #[resource_key]
    pub SetID: u32,
    #[resource_key]
    pub Type: String,
    pub IconPath: String,
    pub ItemFigureIconPath: String,
    pub RelicName: UnhashedTextMapEntry,
    pub ItemBGDesc: UnhashedTextMapEntry,
    pub BGStoryTitle: UnhashedTextMapEntry,
    pub BGStoryContent: UnhashedTextMapEntry,
}


#[derive(Resource, Deserialize, Debug)]
pub struct AvatarSkillConfig {
    #[resource_key]
    pub SkillID: u32,
    pub SkillName: TextMapEntry,
    pub SkillTag: TextMapEntry,
    pub SkillTypeDesc: TextMapEntry,
    #[resource_key]
    pub Level: u32,
    pub MaxLevel: u32,
    pub SkillTriggerKey: String,
    pub SkillIcon: String,
    pub UltraSkillIcon: String,
    pub LevelUpCostList: Vec<()>,
    pub SkillDesc: TextMapEntry,
    pub SimpleSkillDesc: TextMapEntry,
    pub RatedSkillTreeID: Vec<u32>,
    pub RatedRankID: Vec<u32>,
    pub ExtraEffectIDList: Vec<u32>,
    pub SimpleExtraEffectIDList: Vec<u32>,
    pub ShowStanceList: Vec<Float>,
    pub ShowDamageList: Vec<()>,
    pub ShowHealList: Vec<()>,
    pub InitCoolDown: i32,
    pub CoolDown: i32,
    pub SPBase: Option<Float>,
    pub SPMultipleRatio: Option<Float>,
    pub BPNeed: Option<Float>,
    pub BPAdd: Option<Float>,
    pub SkillNeed: TextMapEntry,
    pub DelayRatio: Option<Float>,
    pub ParamList: Vec<Float>,
    pub SimpleParamList: Vec<Float>,
    pub StanceDamageType: Option<String>,
    pub AttackType: Option<String>,
    pub SkillEffect: String,
    pub SkillComboValueDelta: Option<Float>,
}


#[derive(Deserialize, Debug)]
pub struct PropertyQuantity {
    pub PropertyType: String,
    pub Value: Float,
}

#[derive(Resource, Deserialize, Debug)]
pub struct AvatarSkillTreeConfig {
    #[resource_key]
    pub PointID: u32,
    #[resource_key]
    pub Level: u32,
    pub AvatarID: u32,
    pub PointType: u32,
    pub Anchor: String,
    pub MaxLevel: u32,
    pub PrePoint: Vec<u32>,
    pub StatusAddList: Vec<PropertyQuantity>,
    pub MaterialList: Vec<ItemQuantity>,
    pub AvatarPromotionLimit: Option<u32>,
    pub LevelUpSkillID: Vec<u32>,
    pub IconPath: String,
    pub PointName: String,
    pub PointDesc: String,
    pub AbilityName: String,
    pub PointTriggerKey: TextMapEntry,
    pub ParamList: Vec<Float>,
}