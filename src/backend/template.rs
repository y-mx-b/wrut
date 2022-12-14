use crate::backend::utils::get_name;
use crate::{Tag, WrutError};
use anyhow::Result;
use std::os::unix::fs::symlink;
use std::path::PathBuf;

/// A struct representing a `wrut` template.
pub struct Template {
    pub name: String,
    pub path: PathBuf,
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
        let template = Template::global_store()?.join(name);
        let template_path = template.join("path").canonicalize()?;
        let name = get_name(&None, &template)?;

        if template.is_dir() {
            Ok(Self {
                name,
                path: template_path,
            })
        } else {
            Err(WrutError::NoSuchTemplate(template_path, name))?
        }
    }

    /// Add tags to a template.
    pub fn add_tags(self, tags: &Vec<String>) -> Result<Self> {
        let template_tags_dir = self.tag_dir()?;
        for tag in tags {
            let tag_dir = Tag::from(&tag).path()?;
            symlink(&tag_dir, template_tags_dir.join(&tag))?;
            Tag::from(&tag).init(&vec![], &vec![&self.name])?;
        }

        Ok(self)
    }
}
