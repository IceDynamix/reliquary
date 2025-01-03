//! Parse a SRData text map
//!
//! Text maps are indexed using a hash, refer to [`TextMapEntry`].

use crate::resource::TextMapEntry;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
pub struct TextMap(pub HashMap<i32, String>);

impl TextMap {
    pub fn get(&self, text_map_string: &TextMapEntry) -> Option<&str> {
        self.0.get(&text_map_string.Hash).map(|s| s.as_str())
    }
}
