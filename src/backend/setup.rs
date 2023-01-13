use crate::backend::{config::default_config, WutError};
use crate::cli::subcommands::{InitType, SetupArgs, SetupOverwrite};
use crate::cli::Type;
use anyhow::{Context, Result};
use home::home_dir;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
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

impl From<InitType> for Dirs {
    fn from(item: InitType) -> Self {
        match item {
            InitType::Project => Dirs::Projects,
            InitType::Template => Dirs::Templates,
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

pub fn dir(dir: Dirs) -> Result<PathBuf> {
    let home = home_dir().ok_or(WutError::HomeDirectoryNotFound)?;
    Ok(match dir {
        Dirs::Config => home.join(".config/wut"),
        Dirs::Projects => home.join(".wut/projects"),
        Dirs::Tags => home.join(".wut/tags"),
        Dirs::Templates => home.join(".wut/templates"),
        Dirs::Obj => home.join(".wut/.obj"),
    })
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum Files {
    Config,
}

pub fn files() -> Result<HashMap<Files, PathBuf>> {
    let home = home_dir().ok_or(WutError::HomeDirectoryNotFound)?;
    Ok(HashMap::from([(
        Files::Config,
        home.join(".config/wut/config.toml"),
    )]))
}

pub fn file(file: Files) -> Result<PathBuf> {
    let home = home_dir().ok_or(WutError::HomeDirectoryNotFound)?;
    Ok(match file {
        Files::Config => home.join(".config/wut/config.toml"),
    })
}

fn overwrite_dir(d: Dirs) -> Result<()> {
    let dir_path = dir(d)?;
    if dir_path.is_dir() {
        fs::remove_dir_all(&dir_path)?;
    }
    fs::create_dir(&dir_path)?;

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

fn overwrite(list: &Vec<SetupOverwrite>) -> Result<()> {
    Ok(for item in list {
        match item {
            SetupOverwrite::Config => overwrite_config()?,

            SetupOverwrite::Projects => overwrite_dir(Dirs::Projects)?,
            SetupOverwrite::Templates => overwrite_dir(Dirs::Templates)?,
            SetupOverwrite::Tags => overwrite_dir(Dirs::Tags)?,
            SetupOverwrite::Obj => overwrite_dir(Dirs::Obj)?,

            SetupOverwrite::Data => {
                overwrite_dir(Dirs::Projects)?;
                overwrite_dir(Dirs::Templates)?;
                overwrite_dir(Dirs::Tags)?;
                overwrite_dir(Dirs::Obj)?;
            }

            SetupOverwrite::All => {
                overwrite_config()?;

                overwrite_dir(Dirs::Projects)?;
                overwrite_dir(Dirs::Templates)?;
                overwrite_dir(Dirs::Tags)?;
                overwrite_dir(Dirs::Obj)?;
            }
        }
    })
}

/// Initializes all prerequisites for `wut` to function
pub fn setup(args: &SetupArgs) -> Result<()> {
    // overwrite necessary files
    overwrite(&args.overwrite)?;

    // TODO improve the setup, with better error handling

    Ok(())
}
