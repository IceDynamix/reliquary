#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

//! Parse resource files from SRData

use std::ops::Deref;
use serde::Deserialize;
use tracing::warn;
use crate::resource::text_map::TextMap;

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
#[derive(Deserialize, Debug)]
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
/// They can be directly dereferenced to [`i32`].
/// The appropriate text entry can be looked up in a [`TextMap`].
#[derive(Deserialize, Debug)]
pub struct TextMapEntry {
    pub Hash: i32,
}

impl Deref for TextMapEntry {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        &self.Hash
    }
}

impl TextMapEntry {
    pub fn lookup<'a>(&self, text_map: &'a TextMap) -> Option<&'a str> {
        let entry = text_map.0.get(&self.Hash).map(|s| s.as_str());
        if entry.is_none() {
            warn!(hash=self.Hash, "could not find text map entry")
        }
        entry
    }
}

pub trait ResourceMap {
    fn get_json_name() -> &'static str;
}

#[derive(Deserialize, Debug)]
#[serde(transparent)]
pub struct UnhashedTextMapEntry(String);

impl UnhashedTextMapEntry {
    pub fn hash(&self) -> i32 {
        let bytes = self.0.as_bytes();
        let mut hash1 = 5381i32;
        let mut hash2 = hash1;

        for (i, b) in bytes.iter().enumerate() {
            let hash = if i % 2 == 0 {
                &mut hash1
            } else {
                &mut hash2
            };

            *hash = (*hash).wrapping_shl(5).wrapping_add(*hash) ^ *b as i32;
        }

        hash1.wrapping_add(hash2.wrapping_mul(1566083941))
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
        assert_eq!(s.hash(), 386090711);
    }
}
