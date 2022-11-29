use crate::{setup, Type};
use anyhow::Result;
use std::fs;

/// Given the name of a template, unregister it.
pub fn remove_template(name: &String) -> Result<()> {
    unregister(Type::Template, name)
    // TODO remove `.wrut.toml` file as well
}

pub fn remove_tag(name: &String) -> Result<()> {
    unregister(Type::Tag, name)
}

/// Delete the symlink to a project/template or delete a tag directory.
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
