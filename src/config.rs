use std::{fs, path::Path};

use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    archives: Option<Vec<Archive>>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Archive {
    name: String,
    input: String,
    output: String,
}

impl Archive {
    pub(crate) fn new(name: String, input: String, output: String) -> Self {
        Self {
            name,
            input,
            output,
        }
    }

    pub(crate) fn get_name(&self) -> &str {
        &self.name
    }

    pub(crate) fn get_input(&self) -> &str {
        &self.input
    }

    pub(crate) fn get_output(&self) -> &str {
        &self.output
    }
}

pub(crate) fn parse_config(path: &Path) -> Result<Config> {
    let contents = fs::read_to_string(path).with_context(|| format!("{}", path.display()))?;
    Ok(toml::from_str(contents.as_str())?)
}

pub(crate) fn parse_archives(config: &Config) -> Option<impl Iterator<Item = &Archive>> {
    config.archives.as_ref().map(|archives| archives.iter())
}
