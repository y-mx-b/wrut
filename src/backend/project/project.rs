use std::path::PathBuf;
use crate::setup::{dir, Dirs};
use crate::init::*;
use crate::list::list;
use crate::Type;
use anyhow::Result;
use crate::WrutError;
use crate::remove::remove_project;
use std::env::current_dir;

pub struct Project {
    name: String,
    path: PathBuf,
}

impl Project {
    // Create new Project structs

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

        Ok(Project {
            name,
            path,
        })
    }

    /// Get an existing `Project` struct given its name.
    ///
    /// If no such project exists, it will return an error.
    pub fn get(name: &str) -> Result<Self> {
        let projects_dir = dir(Dirs::Projects)?;
        let project = projects_dir.join(name);
        let name = get_name(&None, &project)?;

        if project.is_symlink() {

            Ok(Project {
                name,
                path: project,
            })
        } else {
            Err(WrutError::NoSuchProject(project, name))?
        }
    }

// TODO add list, init, new, remove methods
    
    pub fn list() -> Result<Vec<String>> {
        list(Type::Project)
    }

    // TODO move logic here
    pub fn init(&self, template: &String, config: PathBuf) -> Result<()> {
        init_project(template, &self.path, Some(&self.name), config)
    }

    pub fn new_init(&self, template: &String, config: PathBuf) -> Result<()> {
        // Create new project directory
        let project_dir = current_dir()?.join(&self.name);
        std::fs::create_dir(&project_dir)?;

        // call normal init
        self.init(template, config)
    }

    pub fn remove(&self) -> Result<()> {
        remove_project(&self.name)
    }
}
