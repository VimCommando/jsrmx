use super::JsonReader;
use eyre::{Result, eyre};
use serde_json::Value;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    sync::{Arc, Mutex},
};

#[derive(Clone)]
pub struct InputFile {
    path: PathBuf,
    reader: Arc<Mutex<BufReader<File>>>,
}

impl InputFile {
    pub fn new(path: PathBuf) -> Result<Self> {
        let file = File::open(&path)?;
        let reader = BufReader::new(file);
        Ok(Self {
            path,
            reader: Arc::new(Mutex::new(reader)),
        })
    }
}

impl JsonReader for InputFile {
    fn get_object(&self) -> Result<Value> {
        read_object(&self.path)
    }

    fn read_line(&self, buf: &mut String) -> Result<()> {
        let mut reader = self.reader.lock().map_err(|e| eyre!("{e}"))?;
        reader.read_line(buf)?;
        Ok(())
    }
}

pub fn read_object(input: &PathBuf) -> Result<Value> {
    let file = File::open(input)?;
    let reader = BufReader::new(file);
    let json_value = serde_json::from_reader(reader)?;
    Ok(json_value)
}
