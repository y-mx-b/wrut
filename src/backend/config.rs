use crate::backend::WrutError;
use anyhow::{Result, Error};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::fmt;

#[derive(Deserialize, Serialize)]
pub struct Template {
    pub ignore_dirs: Vec<String>,
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
            ignore_files: vec![".wut.toml".to_string()],
        }
    }
}

#[derive(Deserialize, Serialize, Default)]
pub struct Project {
    pub default_template: Option<String>,
}

#[derive(Deserialize, Serialize, Default)]
pub struct Config {
    pub template: Template,
    pub project: Project,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", toml::to_string_pretty(self).map_err(|_| fmt::Error)?).map_err(|_| fmt::Error)
    }
}

impl Config {
    pub fn from_file(path: PathBuf) -> Result<Config> {
        // TODO return WrutError if fail
        let data = fs::read(&path).or(Err(WrutError::FailedToReadConfigFile(path)))?;
        Ok(toml::from_slice(data.as_slice())?)
    }
}

