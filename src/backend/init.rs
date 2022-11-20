use crate::backend::setup;
use crate::cli::subcommands::{InitArgs, InitType};
use crate::cli::Type;
use anyhow::{Context, Result};
// use std::collections::HashMap;
use std::fs;
use std::os::unix::fs::symlink;
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

// #[derive(Hash, PartialEq, Eq, Debug)]
// pub enum Files {
//     Wut,
// }
//
// pub fn files(root: PathBuf) -> Result<HashMap<Files, PathBuf>> {
//     Ok(HashMap::from([(Files::Wut, root.join(".wut.toml"))]))
// }

/// Register the current working directory under the appropriate `wut` directory as a symlink
pub fn init(root: PathBuf, args: &InitArgs) -> Result<()> {
    let symlink_name: String = {
        match &args.name {
            Some(val) => val.to_string(),
            None => root
                .file_name()
                .expect("Current directory should have a file name")
                .to_str()
                .expect("Current directory should have a file name")
                .to_string(),
        }
    };

    // register symlink in the appropriate directory
    register(args.type_, &root, &symlink_name)?;

    match args.type_ {
        InitType::Template => init_template(root, &symlink_name),
        InitType::Project => {
            let dir: PathBuf = setup::dirs()?
                .get(&setup::Dirs::Templates)
                .expect("Directory should exist after `setup`.")
                .to_path_buf();
            init_project(
                dir.join(args.template.as_ref().expect("Should be provided.")),
                root,
                &symlink_name,
            )
        }
    }
}

fn register(type_: InitType, root: &PathBuf, name: &String) -> Result<()> {
    let dirs = setup::dirs()?;
    let dir = dirs
        .get(&type_.into())
        .expect("Type should map to setup::Dirs");
    let file = dir.join(name);

    // if a file by this name already exists, delete it
    if file.try_exists()? {
        std::fs::remove_file(file)?;
    }

    // create the symlink
    symlink(&root, dir.join(name)).with_context(|| {
        format!(
            "Failed to create symlink from {:?} at {:?}",
            &root,
            dir.join(name)
        )
    })?;

    Ok(())
}

fn init_template(root: PathBuf, name: &String) -> Result<()> {
    // TODO create .wut.toml file for macros and whatnot

    Ok(())
}

fn init_project(origin: PathBuf, root: PathBuf, name: &String) -> Result<()> {
    // TODO filter out specific files
    let walker = WalkDir::new(&origin)
        .min_depth(1)
        .follow_links(true)
        .into_iter();
    for entry in walker.filter_entry(|e| !ignore_dir(e, &[".git", "target"])) {
        let source_dir = &origin.canonicalize()?;
        let source = entry?.path().canonicalize()?;
        let dest = root.join(&source.strip_prefix(source_dir)?);

        if source.is_dir() {
            println!("directory: {}", source_dir.display());
            fs::create_dir(&dest)?;
        }

        if source.is_file() {
            println!("{:?}", source);
            println!("{:?}", &dest);
            fs::copy(&source, &dest)?;
        }
    }

    Ok(())
}

// TODO use config file in the future
fn ignore_dir(entry: &DirEntry, dirs: &[&'static str]) -> bool {
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
