//! Parse a SRData text map
//!
//! Text maps are indexed using a hash, refer to [`TextMapString`].

use std::collections::HashMap;
use serde::Deserialize;
use crate::resource::excel::TextMapString;

#[derive(Deserialize)]
pub struct TextMap(pub HashMap<i32, String>);

impl TextMap {
    pub fn get(&self, text_map_string: &TextMapString) -> Option<&str> {
        self.0.get(&text_map_string.Hash)
            .map(|s| s.as_str())
    }
}