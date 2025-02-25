#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

//! Parse resource files from SRData

use crate::resource::text_map::TextMap;
use serde::Deserialize;
use serde::Serialize;
use std::ops::Deref;
use tracing::warn;

pub mod excel;
pub mod text_map;

/// A float stored in a config file
///
/// Floats are stored in a nested structure as following:
/// ```json
/// "Attribute": {
///     "Value": 1.23456
/// }
/// ```
///
/// They can be directly dereferenced to [`f32`].
#[derive(Deserialize, Serialize, Debug)]
pub struct Float {
    Value: f32,
}

impl Deref for Float {
    type Target = f32;
    fn deref(&self) -> &Self::Target {
        &self.Value
    }
}

/// A text-map entry stored in a config file
///
/// Text-map entries are stored in a nested structure as following:
/// ```json
/// "Attribute": {
///     "Hash": 123456
/// }
/// ```
///
/// They can be directly dereferenced to [`u64`].
/// The appropriate text entry can be looked up in a [`TextMap`].
#[derive(Deserialize, Serialize, Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TextMapEntry {
    pub Hash: u64,
}

impl Deref for TextMapEntry {
    type Target = u64;
    fn deref(&self) -> &Self::Target {
        &self.Hash
    }
}

impl TextMapEntry {
    pub fn lookup<'a>(&self, text_map: &'a TextMap) -> Option<&'a str> {
        let entry = text_map.0.get(&self.Hash).map(|s| s.as_str());
        if entry.is_none() {
            warn!(hash = self.Hash, "could not find text map entry")
        }
        entry
    }
}

pub trait ResourceMap {
    fn get_json_name() -> &'static str;
}

/// A text-map entry that hasn't been converted to its hash.
///
/// The hash function used is the 64-bit xxhash with seed `0`.
#[derive(Deserialize, Serialize, Debug)]
#[serde(transparent)]
pub struct UnhashedTextMapEntry(String);

impl UnhashedTextMapEntry {
    pub fn hash(&self) -> u64 {
        twox_hash::XxHash64::oneshot(0, self.0.as_bytes())
    }

    pub fn lookup<'a>(&self, text_map: &'a TextMap) -> Option<&'a str> {
        text_map.0.get(&self.hash()).map(|s| s.as_str())
    }
}

#[cfg(test)]
mod test {
    use crate::resource::UnhashedTextMapEntry;

    #[test]
    fn hash() {
        let s = UnhashedTextMapEntry("RelicName_31011".to_string());
        assert_eq!(s.hash(), 2829070117814105902);
    }
}
