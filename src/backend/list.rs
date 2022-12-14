use crate::{backend::setup, Type, Project, Tag, Template};
use std::process::Command;
use anyhow::{Context, Result};

/// List the entries of a given type.
///
/// If no such entries exist, or the required data directory itself does not exist, then an empty
/// vector will be returned.
pub fn list(type_: Type) -> Result<Vec<String>> {
    let dir = setup::dir(type_.into())?;
    let mut list: Vec<String> = Vec::new();

    // TODO: better error handling
    for entry in dir.read_dir().with_context(|| {
        format!(
            "Directory {:?} should exist after running `wrut --setup`",
            &dir
        )
    })?.flatten() {
        list.push(
            entry
                .file_name()
                .into_string()
                .expect("File name should exist"),
        );
    }

    Ok(list)
}

impl Project {
    /// Get a `Vec<String>` of containing a list of all currently registered projects.
    pub fn list() -> Result<Vec<String>> {
        list(Type::Project)
    }
}

impl Template {
    /// Get a `Vec<String>` of containing a list of all currently registered projects.
    pub fn list() -> Result<Vec<String>> {
        list(Type::Template)
    }
}

impl Tag {
    // TODO: use termtree instead of this hack
    /// List the projects/templates of a given tag. If `tag` is `None`, list all tags and their
    /// projects/templates.
    pub fn list(tag: &Option<String>) -> Result<String> {
        let tag_dir = if let Some(tag) = tag {
            Tag::from(tag).path()?
        } else {
            Tag::global_store()?
        };

        let output = Command::new("tree")
            .arg(tag_dir.display().to_string())
            .output()?;
        Ok(std::str::from_utf8(&output.stdout)?.to_string())
    }
}
