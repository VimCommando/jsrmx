use super::dots_to_slashes;
use crate::{input::JsonReaderInput, output::JsonWritableOutput, processor::json_field::JsonField};
use eyre::{Result, eyre};
use json_patch::jsonptr::Pointer;
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

        let name_entry = |i: usize, json: &Value| {
            let default_name = format!("object-{i:06}");

            let name = name_list
                .iter()
                .find_map(|name| json.pointer(name))
                .map_or(default_name, |value| {
                    value.as_str().unwrap_or_default().to_string()
                });

            match &type_field {
                Some(field) => json.pointer(&field).map_or(name.clone(), |value| {
                    format!("{name}.{}", value.as_str().unwrap_or_default().to_string())
                }),
                None => name,
            }
        };

        let mut buf = String::new();
        while let Ok(()) = self.input.read_line(&mut buf) {
            match serde_json::from_str::<Value>(&buf) {
                Ok(mut json) => {
                    self.unescape_fields(&mut json);
                    self.drop_fields(&mut json);
                    let entry = vec![(name_entry(i, &json), json)];
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

    fn unescape_fields(&self, json: &mut Value) {
        self.unescape_fields.as_ref().map(|fields| {
            fields.iter().for_each(|field| {
                json.pointer_mut(&dots_to_slashes(field)).map(|value| {
                    log::debug!("Unescaping field {}", field);
                    *value = JsonField::from(value.clone()).unescape();
                });
            })
        });
    }

    fn drop_fields(&self, json: &mut Value) {
        self.drop_fields.as_ref().map(|fields| {
            fields.iter().for_each(|field| {
                let str = dots_to_slashes(field);
                if let Ok(ptr) = Pointer::parse(&str) {
                    ptr.delete(json);
                }
            });
        });
    }
}
