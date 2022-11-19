use anyhow::Result;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Hash, PartialEq, Eq, Debug)]
pub enum Dirs {
    Wut,
}

#[derive(Hash, PartialEq, Eq, Debug)]
pub enum Files {
    Wut,
}

pub fn dirs(root: PathBuf) -> Result<HashMap<Dirs, PathBuf>> {
    Ok(HashMap::from([(Dirs::Wut, root.join(".wut"))]))
}

pub fn files(root: PathBuf) -> Result<HashMap<Files, PathBuf>> {
    Ok(HashMap::from([(Files::Wut, root.join(".wut/wut.toml"))]))
}
