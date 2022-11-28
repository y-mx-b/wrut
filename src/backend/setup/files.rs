use crate::WrutError;
use anyhow::Result;
use home::home_dir;
use std::collections::HashMap;
use std::path::PathBuf;

/// Each variant refers to a file required for `wrut` to function.
#[derive(Hash, Eq, PartialEq, Debug)]
pub enum Files {
    Config,
}

/// Returns a `HashMap` of file paths mapped to `Files` variants.
pub fn files() -> Result<HashMap<Files, PathBuf>> {
    let home = home_dir().ok_or(WrutError::HomeDirectoryNotFound)?;
    Ok(HashMap::from([(
        Files::Config,
        home.join(".config/wrut/config.toml"),
    )]))
}

/// Given a `Files` variant, it will return the path to that file.
pub fn file(file: Files) -> Result<PathBuf> {
    let home = home_dir().ok_or(WrutError::HomeDirectoryNotFound)?;
    Ok(match file {
        Files::Config => home.join(".config/wrut/config.toml"),
    })
}
