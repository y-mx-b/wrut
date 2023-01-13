use crate::{config::Config, setup, Type, WrutError};
use anyhow::{Context, Result};
use std::collections::HashSet;
use std::fs;
use std::io::Write;
use std::os::unix::fs::symlink;
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

/// Initialize a template.
///
/// This function will create a `.wrut.toml` file in the provided directory and register a symlink
/// to `dir` in `~/.wrut/templates`.
///
/// # Arguments
/// * `dir` - The directory to initialize a template in
///     * This directory must already exist.
/// * `name` - The name of the template to initialize
///     * If `name` is `None`, the name will be the name of the directory provided
pub fn init_template(dir: PathBuf, name: &Option<String>) -> Result<()> {
    // register template
    let template_name = get_name(name, &dir)?;
    register(Type::Template, &dir, &template_name)?;

    // create template config
    let mut template_config = fs::File::create(dir.join(".wrut.toml"))?;
    write!(template_config, "{}", Config::default().to_string())?;

    Ok(())
}

/// Initialize a project.
///
/// This function will generate a project from a given template and register a symlink to
/// `project_dir` in `~/.wrut/projects`.
///
/// # Arguments
/// * `template` - The template to generate the project from
/// * `project_dir` - The directory to initialize a project in
///     * This directory must already exist
/// * `name` - The name of the project to initialize
///     * If `name` is `None`, the name will be the name of the directory provided
/// * `config` - The path to the configuration file to use
pub fn init_project(
    template: &String,
    project_dir: &PathBuf,
    name: &Option<String>,
    config: PathBuf,
) -> Result<()> {
    // register project
    let project_name = get_name(name, &project_dir)?;
    register(Type::Project, &project_dir, &project_name)?;

    // get config
    let config = Config::from_file(config.to_path_buf())?;

    // get full template directory, initialize directory walker
    let template_dir = setup::dir(setup::Dirs::Templates)?
        .join(template)
        .canonicalize()?;
    let walker = WalkDir::new(&template_dir)
        .min_depth(1)
        .follow_links(true)
        .into_iter();
    let template_config = Config::from_file(template_dir.join(".wrut.toml"))?;
    // traverse template directory
    for entry in walker.filter_entry(|e| !ignore(e, &config, &template_config)) {
        // source file/directory
        let source = entry?.path().canonicalize()?;
        // path to copy source to
        let dest = project_dir.join(&source.strip_prefix(&template_dir)?);

        if source.is_dir() {
            fs::create_dir(&dest)?;
        }

        if source.is_file() {
            fs::copy(&source, &dest)?;
        }
    }

    Ok(())
}

/// Register a new tag and/or add projects/templates to it.
///
/// If the provided tag does not exist, this function will create a new tag directory under `~/.wrut/tags`.
/// All entries in `templates` and `projects` will be added to their respective directories.
pub fn init_tag(name: &String, templates: &Vec<String>, projects: &Vec<String>) -> Result<()> {
    // TODO create new directory + subdirectories
    let tag_data_dir = setup::dir(setup::Dirs::Tags)?;
    let tag_dir = tag_data_dir.join(name);
    let tag_templates_dir = &tag_dir.join("templates");
    let tag_projects_dir = &tag_dir.join("projects");

    // create tag_dir and projects/templates subdirs if they don't exist
    if !tag_dir.is_dir() {
        fs::create_dir(&tag_dir)?;
        fs::create_dir(&tag_templates_dir)?;
        fs::create_dir(&tag_projects_dir)?;
    }

    // add templates/projects to appropriate dirs
    // TODO check if already exists, don't try to create if it does
    let templates_dir = setup::dir(setup::Dirs::Templates)?;
    for template in templates {
        let template_path = &templates_dir.join(&template).canonicalize()?;
        let tag_template_symlink = &tag_templates_dir.join(&template);
        symlink(template_path, tag_template_symlink)?;
    }

    let projects_dir = setup::dir(setup::Dirs::Projects)?;
    for project in projects {
        let project_path = &projects_dir.join(&project).canonicalize()?;
        let tag_project_symlink = &tag_projects_dir.join(&project);
        symlink(project_path, tag_project_symlink)?;
    }

    Ok(())
}

/// Determine whether to ignore a file/directory given the global and template configuration files.
fn ignore(entry: &DirEntry, global_config: &Config, template_config: &Config) -> bool {
    fn ignore_dir(entry: &DirEntry, dirs: impl Iterator<Item = String>) -> bool {
        let mut b = false;
        for dir in dirs {
            b = entry.path().is_dir()
                && entry
                    .file_name()
                    .to_str()
                    .map(|s| s.starts_with(&dir))
                    .unwrap_or(false);
            if b == true {
                break;
            }
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
            if b == true {
                break;
            }
        }
        b
    }

    // merge ignore lists to reduce the number of comparisons
    let ignore_dirs: HashSet<String> = {
        let mut ignore_dirs = global_config.template.ignore_dirs.clone();
        ignore_dirs.append(&mut template_config.template.ignore_dirs.clone());

        ignore_dirs.into_iter().collect()
    };
    let ignore_files: HashSet<String> = {
        let mut ignore_files = global_config.template.ignore_files.clone();
        ignore_files.append(&mut template_config.template.ignore_files.clone());

        ignore_files.into_iter().collect()
    };

    ignore_dir(entry, ignore_dirs.into_iter()) || ignore_file(entry, ignore_files.into_iter())
}

/// Acquire the name to use. If `name` is `None`, the name of the directory provided by `dir` will
/// be used.
fn get_name(name: &Option<String>, dir: &PathBuf) -> Result<String> {
    Ok(match name {
        Some(val) => val.to_string(),
        None => dir
            .file_name()
            .ok_or(WrutError::FailedToAcquireDirectoryName(dir.clone()))?
            .to_str()
            .ok_or(WrutError::FailedToAcquireDirectoryName(dir.clone()))?
            .to_string(),
    })
}

/// Register a symlink to `dir` given a name and a type.
fn register(type_: Type, dir: &PathBuf, name: &String) -> Result<()> {
    let registry = setup::dir(type_.into())?;
    let file = registry.join(name);

    // if a file by this name already exists, delete it
    if file.try_exists()? {
        std::fs::remove_file(&file)?;
    }

    // create the symlink
    // TODO make cross-platform (someday)
    symlink(&dir, &file)
        .with_context(|| format!("Failed to create symlink to {:?} at {:?}", &dir, &file))?;

    Ok(())
}
