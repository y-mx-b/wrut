use crate::WrutError;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{fmt, fs};

/// A struct representing the `project` table in the configuration file.
#[derive(Deserialize, Serialize)]
pub struct Template {
    /// A list of directories to ignore when generating projects from a template.
    pub ignore_dirs: Vec<String>,
    /// A list of files to ignore when generating projects from a template.
    pub ignore_files: Vec<String>,
}

impl Default for Template {
    fn default() -> Self {
        Template {
            ignore_dirs: vec![
                ".git".to_string(),
                "target".to_string(),
                ".build".to_string(),
            ],
            ignore_files: vec![".wrut.toml".to_string()],
        }
    }
}

/// A struct representing the `project` table in the configuration file.
#[derive(Deserialize, Serialize, Default)]
pub struct Project {}

/// A struct representing the configuration file.
#[derive(Deserialize, Serialize, Default)]
pub struct Config {
    /// Configuration options relating to templates.
    pub template: Template,
    /// Configuration options relating to projects.
    pub project: Project,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            toml::to_string_pretty(self).map_err(|_| fmt::Error)?
        )
        .map_err(|_| fmt::Error)
    }
}

impl Config {
    /// Parse the provided configuration file.
    ///
    /// # Arguments
    /// * `path` - a path to a configuration file
    pub fn from_file(path: PathBuf) -> Result<Config> {
        let data = fs::read(&path).or(Err(WrutError::FailedToReadConfigFile(path)))?;
        Ok(toml::from_slice(data.as_slice())?)
    }

    /// Return an empty configuration struct.
    pub fn empty() -> Self {
        Self {
            template: Template {
                ignore_dirs: Vec::new(),
                ignore_files: Vec::new(),
            },
            project: Project {},
        }
    }
}
