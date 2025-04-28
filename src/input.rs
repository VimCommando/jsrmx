mod directory;
mod file;
mod stdin;

pub use directory::InputDirectory;
use eyre::{Report, Result, eyre};
use file::InputFile;
use serde_json::Value;
use std::{ops::Deref, path::PathBuf, sync::Arc};
use stdin::InputStdin;

pub trait JsonSource: Send + Sync {
    fn get_entries(&self, sort: bool) -> Vec<(String, Value)>;
    fn read_entry(&self) -> Result<(String, Value)>;
}

pub trait JsonReader: Send + Sync {
    fn get_object(&self) -> Result<Value>;
    fn read_line(&self, buf: &mut String) -> Result<()>;
}

#[derive(Clone)]
pub struct JsonSourceInput(Arc<dyn JsonSource>);

impl std::str::FromStr for JsonSourceInput {
    type Err = Report;

    fn from_str(input: &str) -> Result<Self> {
        match input {
            "-" => Ok(JsonSourceInput(Arc::new(InputStdin::new()))),
            input => {
                let path = PathBuf::from(input);
                if path.is_dir() {
                    Ok(JsonSourceInput(Arc::new(InputDirectory::new(path))))
                } else {
                    Err(eyre!("Cannot read entries from file: {input}"))
                }
            }
        }
    }
}

impl Deref for JsonSourceInput {
    type Target = Arc<dyn JsonSource>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone)]
pub struct JsonReaderInput(Arc<dyn JsonReader>);

impl std::str::FromStr for JsonReaderInput {
    type Err = Report;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "-" => Ok(JsonReaderInput(Arc::new(InputStdin::new()))),
            input => {
                let path = PathBuf::from(input);
                if path.is_dir() {
                    Err(eyre!("Cannot read object from directory: {input}"))
                } else {
                    Ok(JsonReaderInput(Arc::new(InputFile::new(path)?)))
                }
            }
        }
    }
}

impl Deref for JsonReaderInput {
    type Target = Arc<dyn JsonReader>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
