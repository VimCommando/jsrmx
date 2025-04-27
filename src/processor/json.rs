/// Merges multiple JSON objects into a single JSON object.
mod merge;
/// Splits a single JSON object into multiple JSON objects.
mod split;

pub use merge::merge;
pub use split::split;

use super::json_text::JsonText;
use crate::processor::dots_to_slashes;
use json_patch::jsonptr::Pointer;

/// Contains the JSON data as a serde_json::Value and provides utility functions for manipulating fields
///
/// Provides methods for:
/// - Escaping/unescaping specific fields
/// - Dropping fields
/// - Converting to/from serde_json::Value

pub struct Json {
    pub value: serde_json::Value,
}

impl Json {
    pub fn new(value: serde_json::Value) -> Self {
        Json { value }
    }
}

impl Json {
    pub fn unescape_fields(&mut self, unescape_fields: Option<&Vec<String>>) {
        unescape_fields.map(|fields| {
            fields.iter().for_each(|field| {
                self.value
                    .pointer_mut(&dots_to_slashes(field))
                    .map(|value| {
                        log::debug!("Unescaping field {}", field);
                        *value = JsonText::from(value.clone()).unescape();
                    });
            });
        });
    }

    pub fn escape_fields(&mut self, escape_fields: Option<&Vec<String>>) {
        escape_fields.map(|fields| {
            fields.iter().for_each(|field| {
                self.value
                    .pointer_mut(&dots_to_slashes(field))
                    .map(|value| {
                        log::debug!("Escaping field {}", field);
                        *value = JsonText::from(value.clone()).escape();
                    });
            });
        });
    }

    pub fn drop_fields(&mut self, drop_fields: Option<&Vec<String>>) {
        drop_fields.map(|fields| {
            fields.iter().for_each(|field| {
                let str = dots_to_slashes(field);
                if let Ok(ptr) = Pointer::parse(&str) {
                    ptr.delete(&mut self.value);
                }
            });
        });
    }
}

impl From<serde_json::Value> for Json {
    fn from(value: serde_json::Value) -> Self {
        Json { value }
    }
}

impl From<Json> for serde_json::Value {
    fn from(json: Json) -> Self {
        json.value
    }
}

impl TryFrom<&str> for Json {
    type Error = serde_json::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Json {
            value: serde_json::from_str(value)?,
        })
    }
}

impl TryFrom<&String> for Json {
    type Error = serde_json::Error;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Ok(Json {
            value: serde_json::from_str(value)?,
        })
    }
}
