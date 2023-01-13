use crate::backend::{config::default_config, WrutError};
use crate::cli::subcommands::{SetupArgs, SetupFlags};
use crate::cli::Type;
use anyhow::Result;
use home::home_dir;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

/// Each variant refers to a specific directory required for `wrut` to function
#[derive(Hash, Eq, PartialEq, Debug)]
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

fn overwrite_dir(d: Dirs) -> Result<()> {
    let dir_path = dir(d)?;
    if dir_path.is_dir() {
        fs::remove_dir_all(&dir_path)?;
    }

    fs::create_dir(&dir_path)?;

    // Special case for overwriting data dir
    if d == Dirs::Data {
        fs::create_dir(dir(Dirs::Obj)?)?;
        fs::create_dir(dir(Dirs::Projects)?)?;
        fs::create_dir(dir(Dirs::Tags)?)?;
        fs::create_dir(dir(Dirs::Templates)?)?;
    }

    Ok(())
}

fn overwrite_config() -> Result<()> {
    let config_path = file(Files::Config)?;
    if config_path.is_file() {
        fs::remove_file(&config_path)?;
    }

    let config_string = default_config()?;
    let mut config_file = fs::File::create(&config_path)?;
    write!(&mut config_file, "{}", config_string)?;

    Ok(())
}

fn overwrite(flag: SetupFlags) -> Result<()> {
    Ok(match flag {
        SetupFlags::All => {
            overwrite_dir(Dirs::Data)?;
            overwrite_config()?;
        }
        SetupFlags::Data => overwrite_dir(Dirs::Data)?,
        SetupFlags::Obj => overwrite_dir(Dirs::Obj)?,
        SetupFlags::Projects => overwrite_dir(Dirs::Projects)?,
        SetupFlags::Tags => overwrite_dir(Dirs::Tags)?,
        SetupFlags::Templates => overwrite_dir(Dirs::Templates)?,
        SetupFlags::Config => overwrite_config()?,
    })
}

/// Initializes all prerequisites for `wrut` to function
pub fn setup(flags: Vec<SetupFlags>) -> Result<()> {
    Ok(for flag in flags {
        overwrite(flag)?;
    })
}
