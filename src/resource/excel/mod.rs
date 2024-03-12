#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

//! Parse config files from SRData ExcelOutput
//!
//! All `...Config.json` files have a `...Config` struct and a `...ConfigMap` struct counterpart.
//! The `...Config` struct represents a single nao entry, while the `...ConfigMap` struct represents the
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
//! Please refer to [`Float`] and [`TextMapString`].
//!
//! ## Example
//! Using RelicMainAffixConfig as an example
//! ```
//! use reliquary::resource::excel::RelicMainAffixConfig::RelicMainAffixConfigMap;
//! let content = "";
//! let config_map: RelicMainAffixConfigMap = serde_json::from_str(&content)?;
//! let config = config_map.get(&0, &1).unwrap(); // GroupID, AffixID
//! ```

use std::ops::Deref;
use serde::Deserialize;
use crate::resource::text_map::TextMap;

pub mod RelicMainAffixConfig;

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
#[derive(Deserialize)]
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
#[derive(Deserialize)]
pub struct TextMapString {
    pub Hash: i32,
}

impl Deref for TextMapString {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        &self.Hash
    }
}

impl TextMapString {
    pub fn lookup(&self, text_map: TextMap) -> Option<&str> {
        text_map.get(self)
    }
}