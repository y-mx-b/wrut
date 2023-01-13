use anyhow::Result;
use serde::{Deserialize, Serialize};

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

pub fn config() -> Result<String> {
    Ok(toml::to_string(&Config::default())?)
}
