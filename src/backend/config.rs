use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use crate::backend::WutError;
use std::path::PathBuf;

#[derive(Deserialize, Serialize, Default)]
pub struct Config {
    template: Template,
    project: Project,
}

#[derive(Deserialize, Serialize, Default)]
pub struct Template {
    ignore_dirs: Vec<String>,
    ignore_files: Vec<String>,
}

#[derive(Deserialize, Serialize, Default)]
pub struct Project {
    default_template: Option<String>,
}

pub fn default_config() -> Result<String> {
    Ok(toml::to_string(&Config::default())?)
}

pub fn get_config(config: PathBuf) -> Result<Config> {
    // TODO return WutError if fail
    let data = fs::read(&config).or(Err(WutError::FailedToReadConfigFile(config)))?;
    Ok(toml::from_slice(data.as_slice())?)
}
