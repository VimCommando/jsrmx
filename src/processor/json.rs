use super::json_text::JsonText;
use crate::processor::dots_to_slashes;
use eyre::Result;
use json_patch::jsonptr::Pointer;
use rayon::prelude::*;
use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;

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

    pub fn unescape(mut self, fields: Option<&Vec<String>>) -> Self {
        log::debug!("Unescaping fields: {:?}", fields);
        if let Some(fields) = fields {
            fields.iter().for_each(|field| {
                self.value
                    .pointer_mut(&dots_to_slashes(field))
                    .map(|value| {
                        log::debug!("Unescaping field {}", field);
                        *value = JsonText::from(value.clone()).unescape();
                    });
            });
        };
        self
    }

    pub fn escape(mut self, fields: Option<&Vec<String>>) -> Self {
        log::debug!("Escaping fields: {:?}", fields);
        if let Some(fields) = fields {
            fields.iter().for_each(|field| {
                self.value
                    .pointer_mut(&dots_to_slashes(field))
                    .map(|value| {
                        log::debug!("Escaping field {}", field);
                        *value = JsonText::from(value.clone()).escape();
                    });
            });
        };
        self
    }

    pub fn drop(mut self, fields: Option<&Vec<String>>) -> Self {
        log::debug!("Dropping fields: {:?}", fields);
        if let Some(fields) = fields {
            fields.iter().for_each(|field| {
                let str = dots_to_slashes(field);
                if let Ok(ptr) = Pointer::parse(&str) {
                    ptr.delete(&mut self.value);
                }
            });
        };
        self
    }

    pub fn filter(mut self, filter: Option<String>) -> Result<Self> {
        log::debug!("Filtering keys: {:?}", filter);
        if let Some(filter) = filter {
            let regex = Regex::new(&filter)?;
            log::info!("Regex key filter: {:?}", regex);
            self.value = self
                .value
                .as_object()
                .expect("Expected object")
                .iter()
                .filter(|(key, _)| regex.is_match(key))
                .map(|(key, value)| (key.clone(), value.clone()))
                .collect();
        }
        Ok(self)
    }

    pub fn entries(self, filter: Option<String>) -> Result<Vec<(String, Value)>> {
        let regex: Option<Regex> = filter.and_then(|f| match Regex::new(&f) {
            Ok(r) => Some(r),
            Err(e) => {
                log::error!("Error parsing regex: {}", e);
                None
            }
        });
        let mut entries: HashMap<String, Value> = serde_json::from_value(self.value)?;
        let entries: Vec<(String, Value)> = entries
            .par_drain()
            .filter_map(|(key, value)| match regex {
                Some(ref regex) => regex.is_match(&key).then_some((key, value)),
                None => Some((key, value)),
            })
            .collect();
        Ok(entries)
    }

    pub fn value(self) -> Value {
        self.value
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

impl From<Vec<(String, Value)>> for Json {
    fn from(value: Vec<(String, Value)>) -> Self {
        Json {
            value: serde_json::Value::Object(
                value.into_iter().map(|(k, v)| (k, v.into())).collect(),
            ),
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn merge_filtered() {
        let entries = vec![
            ("a".to_string(), Value::String("1".to_string())),
            ("b".to_string(), Value::String("2".to_string())),
            ("c".to_string(), Value::String("3".to_string())),
        ];
        let filter = Some("b".to_string());
        let result = Json::from(entries)
            .filter(filter)
            .expect("Failed to filter JSON")
            .value();
        assert_eq!(result, json!({"b": "2"}));
    }

    #[test]
    fn merge_unfiltered() {
        let entries = vec![
            ("a".to_string(), Value::String("1".to_string())),
            ("b".to_string(), Value::String("2".to_string())),
            ("c".to_string(), Value::String("3".to_string())),
        ];
        let filter = None;
        let result = Json::from(entries)
            .filter(filter)
            .expect("Failed to filter JSON")
            .value();
        assert_eq!(result, json!({"a": "1", "b": "2", "c": "3"}));
    }

    #[test]
    fn split_filtered() {
        let object = json!({
            "a": "1",
            "b": "2",
            "c": "3"
        });
        let filter = Some("b".to_string());
        let result = Json::from(object)
            .entries(filter)
            .expect("Failed to split JSON");
        assert_eq!(
            result,
            vec![("b".to_string(), Value::String("2".to_string()))]
        );
    }

    #[test]
    fn split_unfiltered() {
        let object = json!({
            "a": "1",
            "b": "2",
            "c": "3"
        });
        let filter = None;
        let mut entries = Json::from(object)
            .entries(filter)
            .expect("Failed to split JSON");
        // Split's output is non-deterministic, so we sort it to compare
        entries.sort_unstable_by_key(|(key, _)| key.clone());
        assert_eq!(
            entries,
            vec![
                ("a".to_string(), Value::String("1".to_string())),
                ("b".to_string(), Value::String("2".to_string())),
                ("c".to_string(), Value::String("3".to_string())),
            ]
        );
    }
}
