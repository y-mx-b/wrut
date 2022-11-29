use crate::setup;
use anyhow::Result;
use std::fs;
use std::os::unix::fs::symlink;

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
