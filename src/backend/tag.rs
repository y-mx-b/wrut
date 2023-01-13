use crate::backend::utils::unregister;
use crate::backend::setup::{dir, Dirs};
use crate::{Type, Project, Template};
use anyhow::Result;
use std::os::unix::fs::symlink;
use std::process::Command;
use std::path::PathBuf;

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

    /// Register a new tag and/or add projects/templates to it.
    ///
    /// If the provided tag does not exist, this function will create a new tag directory under
    /// `~/.wrut/tags`. All entries in `templates` and `projects` will be added to their respective
    /// directories.
    pub fn init(&self, templates: &Vec<&str>, projects: &Vec<&str>) -> Result<()> {
        let tag_dir = self.path()?;
        let tag_templates_dir = self.templates_dir()?;
        let tag_projects_dir = self.projects_dir()?;

        // create tag_dir and projects/templates subdirs if they don't exist
        if !tag_dir.is_dir() {
            std::fs::create_dir(&tag_dir)?;
            std::fs::create_dir(&tag_templates_dir)?;
            std::fs::create_dir(&tag_projects_dir)?;
        }

        // add templates/projects to appropriate dirs
        // check if already exists, don't try to create if it does
        let templates_dir = Template::global_store()?;
        for template in templates {
            let template_path = &templates_dir.join(&template).canonicalize()?;
            let tag_template_symlink = &tag_templates_dir.join(&template);
            if !tag_template_symlink.is_symlink() {
                symlink(template_path, tag_template_symlink)?;
            }
        }

        for project in projects {
            let project_path = Project::get(project)?.path;
            let tag_project_symlink = &tag_projects_dir.join(&project);
            if !tag_project_symlink.is_symlink() {
                symlink(project_path, tag_project_symlink)?;
            }
        }

        Ok(())
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
