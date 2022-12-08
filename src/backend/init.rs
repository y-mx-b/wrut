use super::utils::{register, ignore};
use super::{Project, Template, Type, config::TemplateConfig};
use anyhow::Result;
use walkdir::WalkDir;
use std::env::current_dir;

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