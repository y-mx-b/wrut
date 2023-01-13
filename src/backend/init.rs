use crate::backend::config;
use crate::backend::{setup, WrutError};
use crate::cli::Type;
use crate::cli::subcommands::{project, template};
use anyhow::{Context, Result};
use std::fs;
use std::os::unix::fs::symlink;
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

// TODO add some info logs

fn get_name(name: &Option<String>, dir: &PathBuf) -> Result<String> {
    Ok(match name {
        Some(val) => val.to_string(),
        None => dir
            .file_name()
            .ok_or(WrutError::FailedToAcquireDirectoryName(dir.clone()))?
            .to_str()
            .ok_or(WrutError::FailedToAcquireDirectoryName(dir.clone()))?
            .to_string()
    })
}

fn register(type_: Type, dir: &PathBuf, name: &String) -> Result<()> {
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

pub fn init_template(dir: PathBuf) -> Result<()> {
    // TODO create .wut.toml file for macros and whatnot

    Ok(())
}

pub fn init_project(args: project::InitArgs, project_dir: PathBuf, config: config::Config) -> Result<()> {
    let template_dir = setup::dir(setup::Dirs::Templates)?.join(args.template).canonicalize()?;
    let walker = WalkDir::new(&template_dir)
        .min_depth(1)
        .follow_links(true)
        .into_iter();

    // TODO use config file to get ignore dirs
    // TODO use config file to get ignore files
    for entry in walker.filter_entry(|e| !ignore_dir(e, &config.template.ignore_dirs)) {
        let source = entry?.path().canonicalize()?;
        let dest = project_dir.join(&source.strip_prefix(&template_dir)?);

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
