use crate::{config::Config, setup, Type, WrutError};
use anyhow::{Context, Result};
use std::fs;
use std::io::Write;
use std::os::unix::fs::symlink;
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};
use std::collections::HashSet;
use core::option::Iter;

pub fn init_template(dir: PathBuf, name: &Option<String>) -> Result<()> {
    // register template
    let template_name = get_name(name, &dir)?;
    register(Type::Template, &dir, &template_name)?;

    // create template config
    let mut template_config = fs::File::create(dir.join(".wut.toml"))?;
    write!(template_config, "{}", Config::empty().to_string())?;

    Ok(())
}

pub fn init_project(
    template: &String,
    project_dir: &PathBuf,
    name: &Option<String>,
    config: PathBuf,
) -> Result<()> {
    // register project
    let project_name = get_name(name, &project_dir)?;
    register(Type::Project, &project_dir, &project_name)?;

    // get config
    let config = Config::from_file(config.to_path_buf())?;

    // get full template directory, initialize directory walker
    let template_dir = setup::dir(setup::Dirs::Templates)?
        .join(template)
        .canonicalize()?;
    let walker = WalkDir::new(&template_dir)
        .min_depth(1)
        .follow_links(true)
        .into_iter();
    let template_config = Config::from_file(template_dir.join(".wut.toml"))?;
    // traverse template directory
    for entry in walker.filter_entry(|e| !ignore(e, &config, &template_config)) {
        // source file/directory
        let source = entry?.path().canonicalize()?;
        // path to copy source to
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

fn ignore(entry: &DirEntry, global_config: &Config, template_config: &Config) -> bool {
    fn ignore_dir(entry: &DirEntry, dirs: impl Iterator<Item=String>) -> bool {
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

    fn ignore_file(entry: &DirEntry, files: impl Iterator<Item=String>) -> bool {
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

    ignore_dir(entry, ignore_dirs.into_iter())
    || ignore_file(entry, ignore_files.into_iter())
}

fn get_name(name: &Option<String>, dir: &PathBuf) -> Result<String> {
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

fn register(type_: Type, dir: &PathBuf, name: &String) -> Result<()> {
    let registry = setup::dir(type_.into())?;
    let file = registry.join(name);

    // if a file by this name already exists, delete it
    if file.try_exists()? {
        std::fs::remove_file(&file)?;
    }

    // create the symlink
    // TODO make cross-platform (someday)
    symlink(&dir, &file)
        .with_context(|| format!("Failed to create symlink to {:?} at {:?}", &dir, &file))?;

    Ok(())
}
