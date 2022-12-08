use super::utils::{ignore, register};
use super::{config::TemplateConfig, Project, Tag, Template, Type};
use anyhow::Result;
use std::env::current_dir;
use std::io::Write;
use std::os::unix::fs::symlink;
use walkdir::WalkDir;

impl Project {
    /// Initialize a project.
    ///
    /// This function will generate a project from a given template and register a symlink to
    /// the current directory in `~/.wrut/projects`.
    ///
    /// # Arguments
    ///
    /// * `template` - The template to generate the project from
    /// * `config` - The path to the configuration file to use
    pub fn init(self, template: &Template) -> Result<Self> {
        // register project
        register(Type::Project, &self.path, &self.name)?;

        // get full template directory, initialize directory walker
        let template_dir = &template.path;
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
    pub fn new_init(self, template: &Template) -> Result<Self> {
        // Create new project directory
        let project_dir = current_dir()?.join(&self.name);
        std::fs::create_dir(&project_dir)?;

        // call normal init
        self.init(template)
    }
}

impl Template {
    /// Initialize a template.
    ///
    /// This function will create a `.wrut.toml` file in the provided directory and register a symlink
    /// to `dir` in `~/.wrut/templates`.
    pub fn init(self) -> Result<Self> {
        // register template
        register(Type::Template, &self.path, &self.name)?;

        // create template config
        let mut template_config = std::fs::File::create(&self.path.join(".wrut.toml"))?;
        write!(template_config, "{}", TemplateConfig::default().to_string())?;

        Ok(self)
    }
}

impl Tag {
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
}
