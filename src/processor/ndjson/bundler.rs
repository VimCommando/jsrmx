use super::dots_to_slashes;
use crate::{
    input::{InputDirectory, JsonSource},
    output::JsonAppendableOutput,
    processor::json_field::JsonField,
};
use eyre::{Result, eyre};

pub struct Bundler {
    input: InputDirectory,
    output: JsonAppendableOutput,
}

impl Bundler {
    pub fn new(input: InputDirectory, output: JsonAppendableOutput) -> Self {
        Self { input, output }
    }

    /// Bundles JSON files from the specified directory into a single output.
    ///
    /// # Arguments
    ///
    /// * `dir` - A reference to a `PathBuf` representing the directory containing JSON files.
    /// * `output` - A reference to an `Output` where the bundled JSON will be written.

    pub fn bundle(&self, json_fields: Option<Vec<String>>) -> Result<()> {
        self.read_entries_to_output(json_fields)
    }

    /// Reads all JSON files in the specified directory and appends their contents to the output.
    ///
    /// # Arguments
    ///
    /// * `dir` - A reference to a `PathBuf` representing the directory containing JSON files.
    /// * `output` - A reference to an `Output` where the JSON data will be appended.

    fn read_entries_to_output(&self, json_fields: Option<Vec<String>>) -> Result<()> {
        log::debug!("Escaping fields: {:?}", json_fields);
        let output = self
            .output
            .read()
            .map_err(|_| eyre!("Error acquiring read lock on output"))?;
        self.input
            .get_entries(false)
            .drain(..)
            .try_for_each(|(_name, mut json)| {
                if let Some(ref json_fields) = json_fields {
                    json_fields.iter().for_each(|field| {
                        if let Some(value) = json.pointer_mut(&dots_to_slashes(field)) {
                            log::debug!("Escaping field {}", field);
                            *value = JsonField::from(value.clone()).escape();
                        }
                    });
                }
                output.append(json).map_err(|e| eyre!(e))
            })
    }
}
