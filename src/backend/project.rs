use crate::backend::setup::{dir, Dirs};
use crate::backend::utils::get_name;
use crate::WrutError;
use anyhow::Result;
use std::path::PathBuf;

/// A struct representing a `wrut` project.
pub struct Project {
    name: String,
    pub path: PathBuf,
}

impl Project {
    /// Create a new `Project` struct given a path and an optional name.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to a directory
    /// * `name` - The name of the project
    ///     * If `name` is `None`, then the directory name of path provided will be used.
    pub fn from(path: impl Into<PathBuf>, name: Option<&str>) -> Result<Self> {
        let path = path.into();
        let name = get_name(&name, &path)?;

        Ok(Project { name, path })
    }

    /// Get an existing `Project` struct given its name.
    ///
    /// If no such project exists, it will return an error.
    pub fn get(name: &str) -> Result<Self> {
        let project = dir(Dirs::Projects)?.join(name);
        let project_path = project.join("path").canonicalize()?;
        let name = get_name(&None, &project)?;

        if project.is_dir() {
            Ok(Project {
                name,
                path: project_path,
            })
        } else {
            Err(WrutError::NoSuchProject(project_path, name))?
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
