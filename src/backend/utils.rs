use std::path::PathBuf;
use anyhow::Result;
use crate::{WrutError, Type};
use crate::setup::dir;

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

/// Delete the symlink to a project/template or delete a tag directory.
pub fn unregister(type_: Type, name: &String) -> Result<()> {
    let target = dir(type_.into())?.join(name);
    let template_config = target.canonicalize()?.join(".wrut.toml");

    if template_config.is_file() {
        std::fs::remove_file(&template_config)?;
    }

    if target.is_symlink() {
        std::fs::remove_file(&target)?;
    }

    if target.is_dir() {
        std::fs::remove_dir_all(target)?;
    }

    Ok(())
}
