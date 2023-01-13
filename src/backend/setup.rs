use crate::backend::{Dirs, WutError};
use crate::cli::subcommands::SetupArgs;
use anyhow::{Context, Result};
use log::info;
use std::fs;

/// Initializes all prerequisites for `wut` to function
pub fn setup(args: &SetupArgs) -> Result<()> {
    let dirs = Dirs::dirs()?;

    // delete and create all dirs if `force` is set
    if args.force {
        for dir in dirs.values() {
            if dir.is_dir() {
                info!("Removing directory {:?}", dir);
                fs::remove_dir_all(dir)
                    .with_context(|| format!("Attempted to recursively remove {:?}", dir))?;
            }

            info!("Creating directory {:?}", dir);
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
            info!("Creating directory: {:?}", dir);
            fs::create_dir_all(dir)?;
        }

        Ok(())
    }
}
