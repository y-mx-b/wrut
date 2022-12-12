use crate::backend::setup::{dir, Dirs};
use crate::backend::utils::{get_name, unregister};
use crate::{Tag, Type, WrutError};
use anyhow::Result;
use std::os::unix::fs::symlink;
use std::path::PathBuf;

/// A struct representing a `wrut` project.
pub struct Project {
    pub name: String,
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

    /// Add tags to a project.
    pub fn add_tags(self, tags: &Vec<String>) -> Result<Self> {
        let project_tags_dir = self.tag_dir()?;
        for tag in tags {
            let tag_dir = dir(Dirs::Tags)?.join(&tag);
            symlink(&tag_dir, project_tags_dir.join(&tag))?;
            Tag::from(&tag).init(&vec![], &vec![&self.name])?;
        }

        Ok(self)
    }

    /// Remove the given project.
    ///
    /// # Arguments
    ///
    /// * `delete` - If `delete` is `true`, the project directory will be deleted. If `false`, then
    /// the project will only be unregistered from `~/.wrut/projects`.
    pub fn remove(&self, delete: bool) -> Result<()> {
        if delete {
            std::fs::remove_dir_all(&self.path)?;
        }

        // delete projects in tags dir
        let project_tags_dir = self.tag_dir()?;
        for tag in project_tags_dir.read_dir()? {
            let tag = tag?;
            // TODO make safer
            let tag = Tag::from(tag.file_name().to_str().unwrap());
            tag.remove(&vec![], &vec![&self.name])?;
        }

        unregister(Type::Project, &self.name)
    }
}
