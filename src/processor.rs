/// Process JSON objects
pub mod json;
/// Encode and decode nested string-escaped JSON objects
pub mod json_text;
/// Process newline-delimited lists of JSON objects
mod ndjson;

pub use ndjson::{Bundler, BundlerBuilder, Unbundler, UnbundlerBuilder};

pub fn dots_to_slashes(str: &str) -> String {
    "/".to_string() + &str.split('.').collect::<Vec<&str>>().join("/")
}
