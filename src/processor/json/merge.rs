use regex::Regex;
use serde_json::Value;

/// Merges a vector of entries into a single JSON object, optionally filtering by a regex pattern.
///
/// # Arguments
///
/// * `entries` - A vector of tuples where each tuple contains a key (String) and a value (Value).
/// * `filter` - An optional string that represents a regex pattern to filter the keys of the entries.
///
/// # Returns
///
/// A `Value` representing a JSON object containing the merged entries that match the filter,
/// or all entries if no filter is provided.

pub fn merge(entries: Vec<(String, Value)>, filter: Option<String>) -> Value {
    if filter.is_none() {
        log::debug!("No key filter given");
        return entries.into_iter().collect();
    } else {
        let regex: Option<Regex> = filter.and_then(|f| Regex::new(&f).ok());
        log::info!("Regex key filter: {:?}", regex);
        let includes_key = |(key, _): &(String, Value)| match regex {
            Some(ref regex) => regex.is_match(key),
            None => true,
        };
        entries.into_iter().filter(includes_key).collect::<Value>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_merge_filtered() {
        let entries = vec![
            ("a".to_string(), Value::String("1".to_string())),
            ("b".to_string(), Value::String("2".to_string())),
            ("c".to_string(), Value::String("3".to_string())),
        ];
        let filter = Some("b".to_string());
        let result = merge(entries, filter);
        assert_eq!(result, json!({"b": "2"}));
    }

    #[test]
    fn test_merge_unfiltered() {
        let entries = vec![
            ("a".to_string(), Value::String("1".to_string())),
            ("b".to_string(), Value::String("2".to_string())),
            ("c".to_string(), Value::String("3".to_string())),
        ];
        let filter = None;
        let result = merge(entries, filter);
        assert_eq!(result, json!({"a": "1", "b": "2", "c": "3"}));
    }
}
