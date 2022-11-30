use crate::backend::utils::{get_name, register, unregister};
use crate::list::list;
use crate::setup::{dir, Dirs};
use crate::{config::Config, Type, WrutError, Tag};
use anyhow::Result;
use std::io::Write;
use std::path::PathBuf;
use std::os::unix::fs::symlink;

/// A struct representing a `wrut` template.
pub struct Template {
    name: String,
    path: PathBuf,
}

impl Template {
    /// Create a new `Template` struct given a path and an optional name.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to a directory
    /// * `name` - The name of the template
    ///     * If `name` is `None`, then the directory name of path provided will be used.
    pub fn from(path: impl Into<PathBuf>, name: Option<&str>) -> Result<Self> {
        let path = path.into();
        let name = get_name(&name, &path)?;

        Ok(Self { name, path })
    }

    /// Get an existing `Template` struct given its name.
    ///
    /// If no such project exists, it will return an error.
    pub fn get(name: &str) -> Result<Self> {
        let template = dir(Dirs::Templates)?.join(name);
        let template_link = template.join("path").canonicalize()?;
        let name = get_name(&None, &template)?;

        if template.is_symlink() {
            Ok(Self {
                name,
                path: template_link,
            })
        } else {
            Err(WrutError::NoSuchProject(template, name))?
        }
    }

    /// Get a `Vec<String>` of containing a list of all currently registered projects.
    pub fn list() -> Result<Vec<String>> {
        list(Type::Template)
    }

    /// Initialize a template.
    ///
    /// This function will create a `.wrut.toml` file in the provided directory and register a symlink
    /// to `dir` in `~/.wrut/templates`.
    pub fn init(&self, tags: &Vec<String>) -> Result<()> {
        // register template
        register(Type::Template, &self.path, &self.name)?;

        // add tags to the template
        let template_tags_dir = dir(Dirs::Templates)?.join(&self.name).join("tags");
        for tag in tags {
            let tag_dir = dir(Dirs::Tags)?.join(&tag);
            symlink(&tag_dir, template_tags_dir.join(&tag))?;
            Tag::from(&tag).init(&vec![], &vec![&self.name])?;
        }

        // create template config
        let mut template_config = std::fs::File::create(&self.path.join(".wrut.toml"))?;
        write!(template_config, "{}", Config::default().to_string())?;

        Ok(())
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

        unregister(Type::Template, &self.name)
    }
}
