use serde_json::Value;

/// A representation of JSON that can be either valid JSON or string-encoded.
///
/// This type is useful for handling JSON that may be either a string representation or
/// an already-parsed value, which can be string-encoded for storing in a text field.
///
/// The two variants are:
/// - `String`: Contains a string-escaped JSON value.
/// - `Value`: Contains an already parsed serde_json Value.
///
/// With two conversion functions:
/// - `unescape`: Converts string-escaped JSON into a `Value`.
/// - `escape`: Converts a `Value` into a string-escaped JSON `String`.

pub enum JsonText {
    String(String),
    Value(Value),
}

impl JsonText {
    pub fn unescape(self) -> Value {
        match self {
            Self::String(string) => {
                let unescaped_string = string.replace(r#"\\""#, "\"");
                log::trace!("Unescaped value: {}", unescaped_string);
                match serde_json::from_str(&unescaped_string) {
                    Ok(json) => json,
                    Err(e) => {
                        log::error!("Failed to unescape value: {e}");
                        Value::String(string)
                    }
                }
            }
            Self::Value(json) => json,
        }
    }

    pub fn escape(self) -> Value {
        match self {
            Self::String(string) => Value::String(string),
            Self::Value(json) => Value::String(json.to_string()),
        }
    }
}

impl From<String> for JsonText {
    fn from(string: String) -> Self {
        Self::String(string)
    }
}

impl From<Value> for JsonText {
    fn from(json: Value) -> Self {
        match json {
            Value::String(string) => Self::String(string),
            _ => Self::Value(json),
        }
    }
}
