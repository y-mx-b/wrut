use crate::backend::WutError;
use anyhow::Result;
use home::home_dir;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum Dirs {
    Wut,
    Config,
    Data,
}

impl Dirs {
    pub fn dirs() -> Result<HashMap<Dirs, PathBuf>, WutError> {
        // TODO use actual errors instead of expect
        let home;
        match home_dir() {
            Some(path) => home = path,
            None => return Err(WutError::HomeDirectoryNotFound),
        }
        Ok(HashMap::from([
            (Dirs::Wut, home.join(".wut")),
            (Dirs::Config, home.join(".config/wut")),
            (Dirs::Data, home.join(".wut/data"))
        ]))
    }
}
