use crate::backend::WutError;
use anyhow::Result;
use home::home_dir;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum Dirs {
    Config,
    Projects,
    Tags,
    Templates,
    Obj,
}

impl Dirs {
    pub fn dirs() -> Result<HashMap<Dirs, PathBuf>, WutError> {
        let home = home_dir().ok_or(WutError::HomeDirectoryNotFound)?;
        Ok(HashMap::from([
            (Dirs::Config, home.join(".config/wut")),
            (Dirs::Projects, home.join(".wut/projects")),
            (Dirs::Tags, home.join(".wut/tags")),
            (Dirs::Templates, home.join(".wut/templates")),
            (Dirs::Obj, home.join(".wut/.obj")),
        ]))
    }
}
