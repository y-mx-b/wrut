use crate::{setup, Type};
use anyhow::{Context, Result};
use std::process::Command;
use std::str;

/// List the entries of a given type.
///
/// If no such entries exist, or the required data directory itself does not exist, then an empty
/// vector will be returned.
pub fn list(type_: Type) -> Result<Vec<String>> {
    let dirs = setup::dirs()?;
    let dir = dirs
        .get(&type_.into())
        .expect("Type should map to setup::Dirs");
    let mut list: Vec<String> = Vec::new();

    // TODO better error handling
    for entry in dir.read_dir().with_context(|| {
        format!(
            "Directory {:?} should exist after running `wrut --setup`",
            &dir
        )
    })? {
        if let Ok(entry) = entry {
            list.push(
                entry
                    .file_name()
                    .into_string()
                    .expect("File name should exist"),
            );
        }
    }

    Ok(list)
}

// TODO use termtree instead of this hack
/// List the projects/templates of a given tag. If `tag` is `None`, list all tags and their
/// projects/templates.
pub fn list_tags(tag: &Option<String>) -> Result<String> {
    let tag_dir = if let Some(tag) = tag {
        setup::dir(setup::Dirs::Tags)?.join(tag)
    } else {
        setup::dir(setup::Dirs::Tags)?
    };

    let output = Command::new("tree")
        .arg(tag_dir.display().to_string())
        .output()?;
    Ok(str::from_utf8(&output.stdout)?.to_string())
}
