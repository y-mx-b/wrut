use crate::setup::dir;
use crate::{config::Config, Type, WrutError};
use anyhow::{Context, Result};
use std::collections::HashSet;
use std::os::unix::fs::symlink;
use std::path::PathBuf;
use walkdir::DirEntry;

/// Acquire the name to use. If `name` is `None`, the name of the directory provided by `dir` will
/// be used.
pub fn get_name(name: &Option<&str>, dir: &PathBuf) -> Result<String> {
    Ok(match name {
        Some(val) => val.to_string(),
        None => dir
            .file_name()
            .ok_or(WrutError::FailedToAcquireDirectoryName(dir.clone()))?
            .to_str()
            .ok_or(WrutError::FailedToAcquireDirectoryName(dir.clone()))?
            .to_string(),
    })
}

pub fn register(type_: Type, path: &PathBuf, name: &String) -> Result<()> {
    let registry = dir(type_.into())?;

    let entry = registry.join(name);
    let link = entry.join("path");
    let entry_tags_dir = entry.join("tags");

    // if a file by this name already exists, delete it
    if entry.is_dir() {
        std::fs::remove_dir_all(&entry)?;
    }

    // create the entry dir
    std::fs::create_dir(entry)?;
    // create the symlink
    symlink(&path, &link)
        .with_context(|| format!("Failed to create symlink to {:?} at {:?}", &path, &link))?;
    std::fs::create_dir(&entry_tags_dir)?;

    Ok(())
}

pub fn unregister(type_: Type, name: &String) -> Result<()> {
    let target = dir(type_.into())?.join(name);
    let template_config = target.canonicalize()?.join(".wrut.toml");

    if template_config.is_file() {
        std::fs::remove_file(&template_config)?;
    }

    if target.is_dir() {
        std::fs::remove_dir_all(target)?;
    }

    Ok(())
}

/// Determine whether to ignore a file/directory given the global and template configuration files.
pub fn ignore(entry: &DirEntry, global_config: &Config, template_config: &Config) -> bool {
    fn ignore_dir(entry: &DirEntry, dirs: impl Iterator<Item = String>) -> bool {
        let mut b = false;
        for dir in dirs {
            b = entry.path().is_dir()
                && entry
                    .file_name()
                    .to_str()
                    .map(|s| s.starts_with(&dir))
                    .unwrap_or(false);
            if b == true {
                break;
            }
        }
        b
    }

    fn ignore_file(entry: &DirEntry, files: impl Iterator<Item = String>) -> bool {
        let mut b = false;
        for file in files {
            b = entry.path().is_file()
                && entry
                    .file_name()
                    .to_str()
                    .map(|s| s.starts_with(&file))
                    .unwrap_or(false);
            if b == true {
                break;
            }
        }
        b
    }

    // merge ignore lists to reduce the number of comparisons
    let ignore_dirs: HashSet<String> = {
        let mut ignore_dirs = global_config.template.ignore_dirs.clone();
        ignore_dirs.append(&mut template_config.template.ignore_dirs.clone());

        ignore_dirs.into_iter().collect()
    };
    let ignore_files: HashSet<String> = {
        let mut ignore_files = global_config.template.ignore_files.clone();
        ignore_files.append(&mut template_config.template.ignore_files.clone());

        ignore_files.into_iter().collect()
    };

    ignore_dir(entry, ignore_dirs.into_iter()) || ignore_file(entry, ignore_files.into_iter())
}
