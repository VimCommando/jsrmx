/// Bundles multiple JSON objects into a single NDJSON output.
mod bundler;
/// Unbundles an NDJSON input into multiple JSON objects.
mod unbundler;

pub use bundler::Bundler;
pub use unbundler::{Unbundler, UnbundlerBuilder};

fn dots_to_slashes(str: &str) -> String {
    "/".to_string() + &str.split('.').collect::<Vec<&str>>().join("/")
}
