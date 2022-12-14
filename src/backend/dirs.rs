use anyhow::Result;
use crate::{Project, Template, Tag};
use crate::backend::setup::{Dirs, dir};
use std::path::PathBuf;

impl Project {
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
