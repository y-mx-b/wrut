use crate::{config::Config, setup, Type};
use anyhow::{Context, Result};
use std::collections::HashSet;
use std::fs;
use std::os::unix::fs::symlink;
use std::path::PathBuf;
use walkdir::DirEntry;

/// Register a new tag and/or add projects/templates to it.
///
/// If the provided tag does not exist, this function will create a new tag directory under `~/.wrut/tags`.
/// All entries in `templates` and `projects` will be added to their respective directories.
pub fn init_tag(name: &String, templates: &Vec<String>, projects: &Vec<String>) -> Result<()> {
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
    // check if already exists, don't try to create if it does
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
pub fn ignore(entry: &DirEntry, global_config: &Config, template_config: &Config) -> bool {
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

/// Register a symlink to `dir` given a name and a type.
pub fn register(type_: Type, dir: &PathBuf, name: &String) -> Result<()> {
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
