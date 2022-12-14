use crate::backend::setup::{dir, Dirs};
use anyhow::Result;
use std::path::PathBuf;
use crate::{Project, Template};
use std::os::unix::fs::symlink;

pub struct Tag {
    pub name: String,
}

impl Tag {
    pub fn from(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    pub fn path(&self) -> Result<PathBuf> {
        Ok(dir(Dirs::Tags)?.join(&self.name))
    }
}

impl Project {
    /// Add tags to a project.
    pub fn add_tags(self, tags: &Vec<String>) -> Result<Self> {
        let project_tags_dir = self.tag_dir()?;
        for tag in tags {
            let tag_dir = dir(Dirs::Tags)?.join(&tag);
            symlink(&tag_dir, project_tags_dir.join(&tag))?;
            Tag::from(&tag).init(&vec![], &vec![&self.name()])?;
        }

        Ok(self)
    }
}

impl Template {
    /// Add tags to a template.
    pub fn add_tags(self, tags: &Vec<String>) -> Result<Self> {
        let template_tags_dir = self.tag_dir()?;
        for tag in tags {
            let tag_dir = Tag::from(&tag).path()?;
            symlink(&tag_dir, template_tags_dir.join(&tag))?;
            Tag::from(&tag).init(&vec![], &vec![&self.name()])?;
        }

        Ok(self)
    }
}
