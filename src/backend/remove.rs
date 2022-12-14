use crate::{Project, Tag, Type, Template};
use anyhow::Result;
use crate::backend::dirs::dir;

fn unregister(type_: Type, name: &str) -> Result<()> {
    let target = dir(type_.into())?.join(name);
    let template_config = target.canonicalize()?.join(".wrut.toml");

    if template_config.is_file() {
        std::fs::remove_file(&template_config)?;
    }

    if target.is_dir() {
        std::fs::remove_dir_all(target)?;
    }

    Ok(())
}

impl Project {
    /// Remove the given project.
    ///
    /// # Arguments
    ///
    /// * `delete` - If `delete` is `true`, the project directory will be deleted. If `false`, then
    /// the project will only be unregistered from `~/.wrut/projects`.
    pub fn remove(&self, delete: bool) -> Result<()> {
        if delete {
            std::fs::remove_dir_all(self.path())?;
        }

        // delete projects in tags dir
        let project_tags_dir = self.tag_dir()?;
        for tag in project_tags_dir.read_dir()? {
            let tag = tag?;
            // TODO: make safer
            let tag = Tag::from(tag.file_name().to_str().unwrap());
            tag.remove(&vec![], &vec![self.name()])?;
        }

        unregister(Type::Project, self.name())
    }
}

impl Template {
    /// Remove the given project.
    ///
    /// # Arguments
    ///
    /// * `delete` - If `delete` is `true`, the project directory will be deleted. If `false`, then
    /// the project will only be unregistered from `~/.wrut/projects`.
    pub fn remove(&self, delete: bool) -> Result<()> {
        if delete {
            std::fs::remove_dir_all(self.path())?;
        }

        // delete templates in tags dir
        let template_tags_dir = self.tag_dir()?;
        for tag in template_tags_dir.read_dir()? {
            let tag = tag?;
            // TODO: make safer
            let tag = Tag::from(tag.file_name().to_str().unwrap());
            tag.remove(&vec![], &vec![self.name()])?;
        }

        unregister(Type::Template, self.name())
    }
}

impl Tag {
    pub fn remove(&self, templates: &Vec<&str>, projects: &Vec<&str>) -> Result<()> {
        if templates.is_empty() && projects.is_empty() {
            unregister(Type::Tag, self.name())
        } else {
            let tag_templates_dir = self.templates_dir()?;
            let tag_projects_dir = self.projects_dir()?;

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
