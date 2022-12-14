use anyhow::Result;
use crate::{Project, Template, Tag, Type, WrutError};
use std::path::PathBuf;
use home::home_dir;

/// Each variant refers to a specific directory required for `wrut` to function.
#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum Dirs {
    Data,
    Obj,
    Projects,
    Tags,
    Templates,
}

impl From<Type> for Dirs {
    fn from(item: Type) -> Self {
        match item {
            Type::Project => Dirs::Projects,
            Type::Template => Dirs::Templates,
            Type::Tag => Dirs::Tags,
        }
    }
}

/// Given a `Dirs` variant, it will return the path to that directory.
pub(crate) fn dir(dir: Dirs) -> Result<PathBuf> {
    let home = home_dir().ok_or(WrutError::HomeDirectoryNotFound)?;
    Ok(match dir {
        Dirs::Data => home.join(".wrut"),
        Dirs::Obj => home.join(".wrut/.obj"),
        Dirs::Projects => home.join(".wrut/projects"),
        Dirs::Tags => home.join(".wrut/tags"),
        Dirs::Templates => home.join(".wrut/templates"),
    })
}

// TODO: change `PathBuf` to reference types?
impl Project {
    /// Return the storage directory containing all projects.
    pub fn global_store() -> Result<PathBuf> {
        dir(Dirs::Projects)
    }

    /// Return the storage directory for a project.
    pub fn store(&self) -> Result<PathBuf> {
        Ok(dir(Dirs::Projects)?.join(self.name()))
    }

    /// Return the tag directory for a project.
    pub fn tag_dir(&self) -> Result<PathBuf> {
        Ok(self.store()?.join("tags"))
    }
}

impl Template {
    /// Return the storage directory containing all projects.
    pub fn global_store() -> Result<PathBuf> {
        dir(Dirs::Templates)
    }

    /// Return the storage directory for a template.
    pub fn store(&self) -> Result<PathBuf> {
        Ok(dir(Dirs::Templates)?.join(self.name()))
    }

    /// Return the tag directory for a template.
    pub fn tag_dir(&self) -> Result<PathBuf> {
        Ok(self.store()?.join("tags"))
    }
}

impl Tag {
    pub fn global_store() -> Result<PathBuf> {
        dir(Dirs::Tags)
    }

    pub fn templates_dir(&self) -> Result<PathBuf> {
        Ok(self.path()?.join("templates"))
    }

    pub fn projects_dir(&self) -> Result<PathBuf> {
        Ok(self.path()?.join("projects"))
    }
}
