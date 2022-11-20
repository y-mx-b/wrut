use crate::backend::WutError;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Serialize, Default)]
pub struct Config {
    pub template: Template,
    pub project: Project,
}

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

/// Return a default config as a `String`
pub fn default_config() -> Result<String> {
    Ok(toml::to_string(&Config::default())?)
}

/// Read the given file and return a `Config` struct
pub fn get_config(config: PathBuf) -> Result<Config> {
    // TODO return WutError if fail
    println!("{:?}", config);
    let data = fs::read(&config).or(Err(WutError::FailedToReadConfigFile(config)))?;
    Ok(toml::from_slice(data.as_slice())?)
}
