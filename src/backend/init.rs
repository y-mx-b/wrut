use crate::backend::setup;
use crate::cli::subcommands::InitType;
use anyhow::{Context, Result};
// use std::collections::HashMap;
use std::os::unix::fs;
use std::path::PathBuf;

// #[derive(Hash, PartialEq, Eq, Debug)]
// pub enum Files {
//     Wut,
// }
//
// pub fn files(root: PathBuf) -> Result<HashMap<Files, PathBuf>> {
//     Ok(HashMap::from([(Files::Wut, root.join(".wut.toml"))]))
// }

/// Register the current working directory under the appropriate `wut` directory as a symlink
pub fn init(root: PathBuf, name: &Option<String>, type_: InitType) -> Result<()> {
    let symlink_name: String = {
        match name {
            Some(val) => val.to_string(),
            None => root
                .file_name()
                .expect("Current directory should have a file name")
                .to_str()
                .expect("Current directory should have a file name")
                .to_string(),
        }
    };
    let dirs = setup::dirs()?;
    let dir = dirs
        .get(&type_.into())
        .expect("InitType should map to setup::Dirs");
    let file = dir.join(&symlink_name);
    if file.try_exists()? {
        std::fs::remove_file(file)?;
    }
    fs::symlink(&root, dir.join(&symlink_name)).with_context(|| {
        format!(
            "Failed to create symlink from {:?} at {:?}",
            &root,
            dir.join(&symlink_name)
        )
    })?;
    Ok(())
}
