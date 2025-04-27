use crate::{
    input::JsonReaderInput,
    output::JsonWritableOutput,
    processor::{dots_to_slashes, json::Json},
};
use eyre::{Result, eyre};
use serde_json::Value;

pub struct UnbundlerBuilder {
    input: JsonReaderInput,
    output: JsonWritableOutput,
    filename: Option<Vec<String>>,
    unescape_fields: Option<Vec<String>>,
    drop_fields: Option<Vec<String>>,
    type_field: Option<String>,
}

impl UnbundlerBuilder {
    pub fn new(input: JsonReaderInput, output: JsonWritableOutput) -> Self {
        Self {
            input,
            output,
            filename: None,
            unescape_fields: None,
            drop_fields: None,
            type_field: None,
        }
    }

    pub fn filename(mut self, filename: Option<Vec<String>>) -> Self {
        self.filename = filename;
        self
    }

    pub fn unescape_fields(mut self, fields: Option<Vec<String>>) -> Self {
        self.unescape_fields = fields;
        self
    }

    pub fn drop_fields(mut self, fields: Option<Vec<String>>) -> Self {
        self.drop_fields = fields;
        self
    }

    pub fn type_field(mut self, field: Option<String>) -> Self {
        self.type_field = field;
        self
    }

    pub fn build(self) -> Unbundler {
        Unbundler {
            input: self.input,
            output: self.output,
            filename: self.filename,
            unescape_fields: self.unescape_fields,
            drop_fields: self.drop_fields,
            type_field: self.type_field,
        }
    }
}

pub struct Unbundler {
    input: JsonReaderInput,
    output: JsonWritableOutput,
    filename: Option<Vec<String>>,
    unescape_fields: Option<Vec<String>>,
    drop_fields: Option<Vec<String>>,
    type_field: Option<String>,
}

impl Unbundler {
    /// Unbundles NDJSON file and writes separate JSON files to the specified output.
    ///
    /// # Arguments
    ///
    /// * `input` - A refeence to an `Input` representing the source of NDJSON data.
    /// * `output` - A reference to an `Output` where the JSON files will be written.
    /// * `name` - An optional name for the JSON objects, used as a key to extract values.

    pub fn unbundle(&self) -> Result<()> {
        let mut i: usize = 0;
        let name_list = match &self.filename {
            Some(list) => list
                .iter()
                .map(|name| dots_to_slashes(&name))
                .collect::<Vec<String>>(),
            None => vec![],
        };
        let type_field = self
            .type_field
            .as_ref()
            .map(|field| dots_to_slashes(&field));

        let name_entry = |i: usize, value: &Value| {
            let default_name = format!("object-{i:06}");

            let name = name_list
                .iter()
                .find_map(|name| value.pointer(name))
                .map_or(default_name, |value| {
                    value.as_str().unwrap_or_default().to_string()
                });

            match &type_field {
                Some(field) => value.pointer(&field).map_or(name.clone(), |value| {
                    format!("{name}.{}", value.as_str().unwrap_or_default().to_string())
                }),
                None => name,
            }
        };

        let mut buf = String::new();
        while let Ok(()) = self.input.read_line(&mut buf) {
            match Json::try_from(&buf) {
                Ok(mut json) => {
                    json.unescape_fields(self.unescape_fields.as_ref());
                    json.drop_fields(self.drop_fields.as_ref());
                    let entry = vec![(name_entry(i, &json.value), json.value)];
                    self.output
                        .read()
                        .map_err(|_| eyre!("Error acquiring read lock on output"))?
                        .write_entries(entry)?
                }
                Err(e) if serde_json::Error::is_eof(&e) => break,
                Err(e) => log::error!("Failed to parse line {}: {}", i, e),
            }
            buf.clear();
            i += 1;
        }
        Ok(())
    }
}
