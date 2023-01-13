use crate::WrutError;
use anyhow::Result;
use home::home_dir;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum Files {
    Config,
}

pub fn files() -> Result<HashMap<Files, PathBuf>> {
    let home = home_dir().ok_or(WrutError::HomeDirectoryNotFound)?;
    Ok(HashMap::from([(
        Files::Config,
        home.join(".config/wrut/config.toml"),
    )]))
}

pub fn file(file: Files) -> Result<PathBuf> {
    let home = home_dir().ok_or(WrutError::HomeDirectoryNotFound)?;
    Ok(match file {
        Files::Config => home.join(".config/wrut/config.toml"),
    })
}
