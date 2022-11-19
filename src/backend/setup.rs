use crate::backend::WutError;
use crate::cli::subcommands::SetupArgs;
use crate::cli::Type;
use anyhow::{Context, Result};
use home::home_dir;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Each variant refers to a specific directory required for `wut` to function
#[derive(Hash, Eq, PartialEq, Debug)]
pub enum Dirs {
    Config,
    Projects,
    Tags,
    Templates,
    Obj,
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
    let home = home_dir().ok_or(WutError::HomeDirectoryNotFound)?;
    Ok(HashMap::from([
        (Dirs::Config, home.join(".config/wut")),
        (Dirs::Projects, home.join(".wut/projects")),
        (Dirs::Tags, home.join(".wut/tags")),
        (Dirs::Templates, home.join(".wut/templates")),
        (Dirs::Obj, home.join(".wut/.obj")),
    ]))
}

/// Initializes all prerequisites for `wut` to function
pub fn setup(args: &SetupArgs) -> Result<()> {
    let dirs = dirs()?;

    // delete and create all dirs if `force` is set
    if args.force {
        for dir in dirs.values() {
            if dir.is_dir() {
                fs::remove_dir_all(dir)
                    .with_context(|| format!("Attempted to recursively remove {:?}", dir))?;
            }

            fs::create_dir_all(dir)?;
        }

        return Ok(());
    }

    // initial checks
    let mut exists = Vec::new();
    for dir in dirs.values() {
        if dir.is_dir() {
            exists.push(dir.clone());
        }
    }

    if exists.len() != 0 {
        // err if dirs already exist
        Err(WutError::SetupDirAlreadyExists(exists).into())
    } else {
        // create dirs
        for dir in dirs.values() {
            fs::create_dir_all(dir)?;
        }

        Ok(())
    }
}
