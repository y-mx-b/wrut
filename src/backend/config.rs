use crate::WrutError;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{fmt, fs};

/// A struct representing the template configuration file.
#[derive(Deserialize, Serialize, Default, Debug)]
pub struct TemplateConfig {
    /// A list of directories to ignore when generating projects from a template.
    pub ignore_dirs: Vec<String>,
    /// A list of files to ignore when generating projects from a template.
    pub ignore_files: Vec<String>,
}

impl fmt::Display for TemplateConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            toml::to_string_pretty(self).map_err(|_| fmt::Error)?
        )
        .map_err(|_| fmt::Error)
    }
}

impl TemplateConfig {
    /// Parse the provided configuration file.
    ///
    /// # Arguments
    /// * `path` - a path to a configuration file
    pub fn from_file(path: PathBuf) -> Result<Self> {
        let data = fs::read(&path).or(Err(WrutError::FailedToReadConfigFile(path)))?;
        Ok(toml::from_slice(data.as_slice())?)
    }
}
