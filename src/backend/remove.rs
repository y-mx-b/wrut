use crate::{Type, setup};
use anyhow::Result;
use std::fs;

pub fn remove_project(name: &String) -> Result<()> {
    unregister(Type::Project, name)
}
pub fn remove_template(name: &String) -> Result<()> {
    unregister(Type::Template, name)
}

/// Delete symlink to a project/template or delete a tag directory.
fn unregister(type_: Type, name: &String) -> Result<()> {
    let target = setup::dir(type_.into())?.join(name);

    if target.is_symlink() {
        fs::remove_file(&target)?;
    }

    if target.is_dir() {
        fs::remove_dir_all(target)?;
    }

    Ok(())
}
