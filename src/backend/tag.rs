use crate::backend::setup::{dir, Dirs};
use crate::backend::utils::unregister;
use crate::Type;
use anyhow::Result;
use std::path::PathBuf;
use std::process::Command;

pub struct Tag {
    name: String,
}

impl Tag {
    pub fn from(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    pub fn global_store() -> Result<PathBuf> {
        Ok(dir(Dirs::Tags)?)
    }

    pub fn path(&self) -> Result<PathBuf> {
        Ok(dir(Dirs::Tags)?.join(&self.name))
    }

    pub fn templates_dir(&self) -> Result<PathBuf> {
        Ok(self.path()?.join("templates"))
    }

    pub fn projects_dir(&self) -> Result<PathBuf> {
        Ok(self.path()?.join("projects"))
    }

    // TODO use termtree instead of this hack
    /// List the projects/templates of a given tag. If `tag` is `None`, list all tags and their
    /// projects/templates.
    pub fn list(tag: &Option<String>) -> Result<String> {
        let tag_dir = if let Some(tag) = tag {
            Tag::from(tag).path()?
        } else {
            Tag::global_store()?
        };

        let output = Command::new("tree")
            .arg(tag_dir.display().to_string())
            .output()?;
        Ok(std::str::from_utf8(&output.stdout)?.to_string())
    }

    pub fn remove(&self, templates: &Vec<&str>, projects: &Vec<&str>) -> Result<()> {
        if templates.len() == 0 && projects.len() == 0 {
            unregister(Type::Tag, &self.name)
        } else {
            let tag_templates_dir = dir(Dirs::Tags)?.join(&self.name).join("templates");
            let tag_projects_dir = dir(Dirs::Tags)?.join(&self.name).join("projects");

            for template in templates {
                let template_link = tag_templates_dir.join(template);
                if template_link.is_symlink() {
                    std::fs::remove_file(template_link)?;
                }
            }

            for project in projects {
                let project_link = tag_projects_dir.join(project);
                if project_link.is_symlink() {
                    std::fs::remove_file(project_link)?;
                }
            }

            Ok(())
        }
    }
}
