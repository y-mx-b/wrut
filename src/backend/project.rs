use crate::backend::utils::{get_name, ignore, register, unregister};
use crate::list::list;
use crate::setup::{dir, Dirs};
use crate::{config::TemplateConfig, Tag, Type, WrutError};
use anyhow::Result;
use std::env::current_dir;
use std::os::unix::fs::symlink;
use std::path::PathBuf;
use walkdir::WalkDir;

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
    pub fn init(self, template: &String) -> Result<Self> {
        // register project
        register(Type::Project, &self.path, &self.name)?;

        // get full template directory, initialize directory walker
        let template_dir = dir(Dirs::Templates)?
            .join(template)
            .join("path")
            .canonicalize()?;
        let walker = WalkDir::new(&template_dir)
            .min_depth(1)
            .follow_links(true)
            .into_iter();

        // get configs
        let template_config = TemplateConfig::from_file(template_dir.join(".wrut.toml"))?;

        for entry in walker.filter_entry(|e| !ignore(e, &template_config)) {
            let source = entry?.path().canonicalize()?;
            let dest = self.path.join(&source.strip_prefix(&template_dir)?);

            // check if source is file or dir
            if source.is_dir() {
                std::fs::create_dir(&dest)?;
            } else if source.is_file() {
                std::fs::copy(&source, &dest)?;
            }
        }

        Ok(self)
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
    pub fn new_init(self, template: &String) -> Result<Self> {
        // Create new project directory
        let project_dir = current_dir()?.join(&self.name);
        std::fs::create_dir(&project_dir)?;

        // call normal init
        self.init(template)
    }

    /// Add tags to a project.
    pub fn add_tags(self, tags: &Vec<String>) -> Result<Self> {
        let project_tags_dir = dir(Dirs::Projects)?.join(&self.name).join("tags");
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
        let project_tags_dir = dir(Dirs::Projects)?.join(&self.name).join("tags");
        for tag in project_tags_dir.read_dir()? {
            let tag = tag?;
            // TODO make safer
            let tag = Tag::from(tag.file_name().to_str().unwrap());
            tag.remove(&vec![], &vec![&self.name])?;
        }

        unregister(Type::Project, &self.name)
    }
}
