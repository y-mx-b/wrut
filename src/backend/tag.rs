use anyhow::Result;
use std::path::PathBuf;
use crate::{Project, Template};
use std::os::unix::fs::symlink;

pub struct Tag {
    name: String,
}

impl Tag {
    pub fn from(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    // TODO: make reference?
    pub fn path(&self) -> Result<PathBuf> {
        Ok(Tag::global_store()?.join(self.name()))
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Project {
    /// Add tags to a project.
    pub fn add_tags(self, tags: &Vec<String>) -> Result<Self> {
        let project_tags_dir = self.tag_dir()?;
        for tag in tags {
            let tag = Tag::from(tag);
            symlink(tag.path()?, project_tags_dir.join(tag.name()))?;
            tag.init(&vec![], &vec![self.name()])?;
        }

        Ok(self)
    }
}

impl Template {
    /// Add tags to a template.
    pub fn add_tags(self, tags: &Vec<String>) -> Result<Self> {
        let template_tags_dir = self.tag_dir()?;
        for tag in tags {
            let tag_dir = Tag::from(tag).path()?;
            symlink(&tag_dir, template_tags_dir.join(tag))?;
            Tag::from(tag).init(&vec![], &vec![self.name()])?;
        }

        Ok(self)
    }
}
