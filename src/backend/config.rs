use crate::WrutError;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{fmt, fs};

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
pub struct Project {}

#[derive(Deserialize, Serialize, Default)]
pub struct Config {
    pub template: Template,
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
    pub fn from_file(path: PathBuf) -> Result<Config> {
        let data = fs::read(&path).or(Err(WrutError::FailedToReadConfigFile(path)))?;
        Ok(toml::from_slice(data.as_slice())?)
    }

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
