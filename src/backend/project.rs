use crate::setup::{dir, Dirs};
use crate::{Type, WrutError, config::Config};
use anyhow::Result;
use std::env::current_dir;
use std::path::PathBuf;
use walkdir::WalkDir;
use crate::backend::utils::{unregister, get_name, register, ignore};
use crate::list::list;

/// A struct representing a `wrut` project.
pub struct Project {
    name: String,
    path: PathBuf,
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

    /// Get a `Vec<String>` of containing a list of all currently registered projects.
    pub fn list() -> Result<Vec<String>> {
        list(Type::Project)
    }

    /// Initialize a project.
    ///
    /// This function will generate a project from a given template and register a symlink to
    /// the current directory in `~/.wrut/projects`.
    ///
    /// # Arguments
    ///
    /// * `template` - The template to generate the project from
    /// * `config` - The path to the configuration file to use
    pub fn init(&self, template: &String, config: PathBuf) -> Result<()> {
        // register project
        register(Type::Project, &self.path, &self.name)?;

        // get full template directory, initialize directory walker
        let template_dir = dir(Dirs::Templates)?
            .join(template)
            .canonicalize()?;
        let walker = WalkDir::new(&template_dir)
            .min_depth(1)
            .follow_links(true)
            .into_iter();

        // get configs
        let config = Config::from_file(config.to_path_buf())?;
        let template_config = Config::from_file(template_dir.join(".wrut.toml"))?;

        for entry in walker.filter_entry(|e| !ignore(e, &config, &template_config)) {
            let source = entry?.path().canonicalize()?;
            let dest = self.path.join(&source.strip_prefix(&template_dir)?);

            // check if source is file or dir
            if source.is_dir() {
                std::fs::create_dir(&dest)?;
            } else if source.is_file() {
                std::fs::copy(&source, &dest)?;
            }
        }
        Ok(())
    }

    /// Create a new project directory and then initialize the project.
    ///
    /// This function will create a new directory, generate a project from a given template in that
    /// directory, and then register a symlink to that directory in `~/.wrut/projects`.
    ///
    /// # Arguments
    ///
    /// * `template` - The template to generate the project from
    /// * `config` - The path to the configuration file to use
    pub fn new_init(&self, template: &String, config: PathBuf) -> Result<()> {
        // Create new project directory
        let project_dir = current_dir()?.join(&self.name);
        std::fs::create_dir(&project_dir)?;

        // call normal init
        self.init(template, config)
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

        unregister(Type::Project, &self.name)
    }
}
