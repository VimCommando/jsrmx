use super::{JsonReader, JsonSource};
use eyre::{eyre, Result};
use serde_json::Value;
use std::{
    collections::HashMap,
    io::{stdin, BufRead, BufReader, Read, Stdin},
    sync::{Arc, Mutex},
};

#[derive(Clone)]
pub struct InputStdin {
    reader: Arc<Mutex<BufReader<Stdin>>>,
}

impl InputStdin {
    pub fn new() -> Self {
        Self {
            reader: Arc::new(Mutex::new(BufReader::new(stdin()))),
        }
    }
}

impl JsonSource for InputStdin {
    fn get_entries(&self, _sort: bool) -> Vec<(String, Value)> {
        let mut entries = Vec::new();
        let mut buf = String::new();
        while self.read_line(&mut buf).is_ok() {
            if buf.is_empty() {
                break;
            }
            match serde_json::from_str(&buf) {
                Ok(entry) => entries.push(entry),
                Err(e) => {
                    log::error!("Error parsing JSON: {}", e);
                }
            }
            buf.clear();
        }
        entries
    }
}

impl JsonReader for InputStdin {
    fn get_object(&self) -> Result<HashMap<String, Value>> {
        let mut buffer = String::new();
        stdin().read_to_string(&mut buffer)?;
        Ok(serde_json::from_str(&buffer)?)
    }

    fn read_line(&self, buf: &mut String) -> Result<()> {
        let mut reader = self.reader.lock().map_err(|e| eyre!("{e}"))?;
        reader.read_line(buf)?;
        Ok(())
    }
}

impl std::io::Read for InputStdin {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut reader = self.reader.lock().map_err(|_| std::io::ErrorKind::Other)?;
        reader.read(buf)
    }
}
