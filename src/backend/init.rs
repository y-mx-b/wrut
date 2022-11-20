use crate::backend::config;
use crate::backend::{setup, WutError};
use crate::cli::subcommands::{InitArgs, InitType};
use anyhow::{Context, Result};
use std::fs;
use std::os::unix::fs::symlink;
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

// TODO add some info logs

/// Register the current working directory under the appropriate `wut` directory as a symlink
pub fn init(dir: PathBuf, args: &InitArgs, config: config::Config) -> Result<()> {
    let symlink_name: String = {
        match &args.name {
            Some(val) => val.to_string(),
            None => dir
                .file_name()
                // TODO remove clone() call
                .ok_or(WutError::FailedToAcquireDirectoryName(dir.clone()))?
                .to_str()
                .ok_or(WutError::FailedToAcquireDirectoryName(dir.clone()))?
                .to_string(),
        }
    };

    // register symlink in the appropriate directory
    register(args.type_, &dir, &symlink_name)?;

    match args.type_ {
        InitType::Template => init_template(dir),
        InitType::Project => init_project(
            setup::dir(setup::Dirs::Templates)?
                .join(args.template.as_ref().expect("Should be provided.")),
            dir,
            config,
        ),
    }
}

fn register(type_: InitType, dir: &PathBuf, name: &String) -> Result<()> {
    let registry = setup::dir(type_.into())?;
    let file = registry.join(name);

    // if a file by this name already exists, delete it
    if file.try_exists()? {
        std::fs::remove_file(&file)?;
    }

    // create the symlink
    symlink(&dir, &file)
        .with_context(|| format!("Failed to create symlink to {:?} at {:?}", &dir, &file))?;

    Ok(())
}

fn init_template(dir: PathBuf) -> Result<()> {
    // TODO create .wut.toml file for macros and whatnot

    Ok(())
}

fn init_project(template: PathBuf, dir: PathBuf, config: config::Config) -> Result<()> {
    let source_dir = &template.canonicalize()?;
    let walker = WalkDir::new(source_dir)
        .min_depth(1)
        .follow_links(true)
        .into_iter();

    // TODO use config file to get ignore dirs
    // TODO use config file to get ignore files
    for entry in walker.filter_entry(|e| !ignore_dir(e, &config.template.ignore_dirs)) {
        let source = entry?.path().canonicalize()?;
        let dest = dir.join(&source.strip_prefix(source_dir)?);

        if source.is_dir() {
            fs::create_dir(&dest)?;
        }

        if source.is_file() {
            fs::copy(&source, &dest)?;
        }
    }

    Ok(())
}

fn ignore_dir(entry: &DirEntry, dirs: &Vec<String>) -> bool {
    let mut b = false;
    for dir in dirs.iter() {
        b = entry.path().is_dir()
            && entry
                .file_name()
                .to_str()
                .map(|s| s.starts_with(dir))
                .unwrap_or(false);
        if b == true {
            break;
        }
    }
    b
}

// TODO add ignore_file function
