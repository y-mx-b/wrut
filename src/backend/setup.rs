use crate::backend::{config::default_config, WrutError};
use super::Type;
use anyhow::Result;
use home::home_dir;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use clap::ValueEnum;

#[derive(ValueEnum, Debug, PartialEq, Eq, Clone)]
pub enum SetupFlag {
    /// Restore everything to default (alias: 'a').
    #[clap(alias = "a")]
    All,

    /// Set `~/.wrut` to default (alias: 'd').
    #[clap(alias = "d")]
    Data,
    /// Set `~/.wrut/.obj` to default (alias: 'o').
    #[clap(alias = "o")]
    Obj,
    /// Set `~/.wrut/projects` to default (alias: 'p').
    #[clap(alias = "p")]
    Projects,
    /// Set `~/.wrut/templates` to default (alias: 't').
    #[clap(alias = "t")]
    Templates,
    /// Set `~/.wrut/tags` to default (alias: 's').
    #[clap(alias = "s")]
    Tags,

    /// Set `~/.config/wrut` to default (alias: 'c').
    #[clap(alias = "c")]
    Config,
}

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

fn overwrite(flag: SetupFlag) -> Result<()> {
    Ok(match flag {
        SetupFlag::All => {
            overwrite_dir(Dirs::Data)?;
            overwrite_dir(Dirs::Config)?;
            overwrite_config()?;
        }
        SetupFlag::Data => overwrite_dir(Dirs::Data)?,
        SetupFlag::Obj => overwrite_dir(Dirs::Obj)?,
        SetupFlag::Projects => overwrite_dir(Dirs::Projects)?,
        SetupFlag::Tags => overwrite_dir(Dirs::Tags)?,
        SetupFlag::Templates => overwrite_dir(Dirs::Templates)?,
        SetupFlag::Config => {
            overwrite_dir(Dirs::Config)?;
            overwrite_config()?;
        }
    })
}

/// Initializes all prerequisites for `wrut` to function
pub fn setup(flags: Vec<SetupFlag>) -> Result<()> {
    Ok(for flag in flags {
        overwrite(flag)?;
    })
}
