use crate::{
    input::{InputDirectory, JsonSource},
    output::JsonAppendableOutput,
    processor::json::Json,
};
use eyre::{Result, eyre};

pub struct BundlerBuilder {
    input: InputDirectory,
    output: JsonAppendableOutput,
    escape_fields: Option<Vec<String>>,
    drop_fields: Option<Vec<String>>,
}

impl BundlerBuilder {
    pub fn new(input: InputDirectory, output: JsonAppendableOutput) -> Self {
        Self {
            input,
            output,
            escape_fields: None,
            drop_fields: None,
        }
    }

    pub fn escape_fields(mut self, fields: Option<Vec<String>>) -> Self {
        self.escape_fields = fields;
        self
    }

    pub fn drop_fields(mut self, fields: Option<Vec<String>>) -> Self {
        self.drop_fields = fields;
        self
    }

    pub fn build(self) -> Bundler {
        Bundler {
            input: self.input,
            output: self.output,
            escape_fields: self.escape_fields,
            drop_fields: self.drop_fields,
        }
    }
}

pub struct Bundler {
    input: InputDirectory,
    output: JsonAppendableOutput,
    escape_fields: Option<Vec<String>>,
    drop_fields: Option<Vec<String>>,
}

impl Bundler {
    pub fn bundle(&self) -> Result<()> {
        let output = self
            .output
            .read()
            .map_err(|e| eyre!("Error acquiring read lock on output: {}", e))?;
        self.input
            .get_entries(false)
            .drain(..)
            .try_for_each(|(_name, value)| {
                let json = Json::from(value)
                    .escape(self.escape_fields.as_ref())
                    .drop(self.drop_fields.as_ref())
                    .value();
                output.append(json).map_err(|e| eyre!(e))
            })
    }
}
