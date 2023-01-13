use crate::{Type, WrutError};
use anyhow::Result;
use home::home_dir;
use std::collections::HashMap;
use std::path::PathBuf;

/// Each variant refers to a specific directory required for `wrut` to function
#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum Dirs {
    Config,
    Data,
    Obj,
    Projects,
    Tags,
    Templates,
}

impl From<Type> for Dirs {
    fn from(item: Type) -> Self {
        match item {
            Type::Project => Dirs::Projects,
            Type::Template => Dirs::Templates,
            Type::Tag => Dirs::Tags,
        }
    }
}

/// Returns a `HashMap` of directory paths mapped to `Dirs` variants
pub fn dirs() -> Result<HashMap<Dirs, PathBuf>> {
    let home = home_dir().ok_or(WrutError::HomeDirectoryNotFound)?;
    Ok(HashMap::from([
        (Dirs::Config, home.join(".config/wrut")),
        (Dirs::Data, home.join(".wrut")),
        (Dirs::Obj, home.join(".wrut/.obj")),
        (Dirs::Projects, home.join(".wrut/projects")),
        (Dirs::Tags, home.join(".wrut/tags")),
        (Dirs::Templates, home.join(".wrut/templates")),
    ]))
}

pub fn dir(dir: Dirs) -> Result<PathBuf> {
    let home = home_dir().ok_or(WrutError::HomeDirectoryNotFound)?;
    Ok(match dir {
        Dirs::Config => home.join(".config/wrut"),
        Dirs::Data => home.join(".wrut"),
        Dirs::Obj => home.join(".wrut/.obj"),
        Dirs::Projects => home.join(".wrut/projects"),
        Dirs::Tags => home.join(".wrut/tags"),
        Dirs::Templates => home.join(".wrut/templates"),
    })
}
