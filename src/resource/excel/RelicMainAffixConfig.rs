use serde::Deserialize;
use reliquary_proc_macro::Resource;
use crate::resource::excel::Float;

#[derive(Resource, Deserialize)]
pub struct RelicMainAffixConfig {
    #[resource_key]
    pub GroupID: i32,
    #[resource_key]
    pub AffixID: i32,
    pub Property: String,
    pub BaseValue: Float,
    pub LevelAdd: Float,
    pub IsAvailable: bool,
}