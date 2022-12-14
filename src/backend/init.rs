use super::{config::TemplateConfig, Project, Tag, Template, Type};
use anyhow::{Result, Context};
use std::collections::HashSet;
use std::env::current_dir;
use std::io::Write;
use std::os::unix::fs::symlink;
use walkdir::{WalkDir, DirEntry};
use std::path::PathBuf;
use crate::backend::setup::dir;

fn register(type_: Type, path: &PathBuf, name: &str) -> Result<()> {
    let registry = dir(type_.into())?;

    let entry = registry.join(name);
    let link = entry.join("path");
    let entry_tags_dir = entry.join("tags");

    // if a file by this name already exists, delete it
    if entry.is_dir() {
        std::fs::remove_dir_all(&entry)?;
    }

    // create the entry dir
    std::fs::create_dir(entry)?;
    // create the symlink
    symlink(path, &link)
        .with_context(|| format!("Failed to create symlink to {:?} at {:?}", &path, &link))?;
    std::fs::create_dir(&entry_tags_dir)?;

    Ok(())
}

/// Determine whether to ignore a file/directory given the global and template configuration files.
fn ignore(entry: &DirEntry, template_config: &TemplateConfig) -> bool {
    fn ignore_dir(entry: &DirEntry, dirs: impl Iterator<Item = String>) -> bool {
        let mut b = false;
        for dir in dirs {
            b = entry.path().is_dir()
                && entry
                    .file_name()
                    .to_str()
                    .map(|s| s.starts_with(&dir))
                    .unwrap_or(false);
            if b { break; }
        }
        b
    }

    fn ignore_file(entry: &DirEntry, files: impl Iterator<Item = String>) -> bool {
        let mut b = false;
        for file in files {
            b = entry.path().is_file()
                && entry
                    .file_name()
                    .to_str()
                    .map(|s| s.starts_with(&file))
                    .unwrap_or(false);
            if b { break; }
        }
        b
    }

    // merge ignore lists to reduce the number of comparisons if there are duplicates
    let ignore_dirs: HashSet<String> = template_config.ignore_dirs.iter().cloned().collect();
    let ignore_files: HashSet<String> = template_config.ignore_files.iter().cloned().collect();

    ignore_dir(entry, ignore_dirs.into_iter()) || ignore_file(entry, ignore_files.into_iter())
}

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
        register(Type::Project, self.path(), self.name())?;

        // get full template directory, initialize directory walker
        let template_dir = &template.path();
        let walker = WalkDir::new(template_dir)
            .min_depth(1)
            .follow_links(true)
            .into_iter();

        // get configs
        let template_config = TemplateConfig::from_file(template_dir.join(".wrut.toml"))?;

        for entry in walker.filter_entry(|e| !ignore(e, &template_config)) {
            let source = entry?.path().canonicalize()?;
            let dest = self.path().join(source.strip_prefix(template_dir)?);

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
        let project_dir = current_dir()?.join(self.name());
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
        register(Type::Template, self.path(), self.name())?;

        // create template config
        let mut template_config = std::fs::File::create(self.path().join(".wrut.toml"))?;
        write!(template_config, "{}", TemplateConfig::default())?;

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
            let template_path = &templates_dir.join(template).canonicalize()?;
            let tag_template_symlink = &tag_templates_dir.join(template);
            if !tag_template_symlink.is_symlink() {
                symlink(template_path, tag_template_symlink)?;
            }
        }

        for project in projects {
            let project = Project::get(project)?;
            let project_path = project.path();
            let tag_project_symlink = &tag_projects_dir.join(project.name());
            if !tag_project_symlink.is_symlink() {
                symlink(project_path, tag_project_symlink)?;
            }
        }

        Ok(())
    }
}
