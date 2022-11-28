use crate::{setup, Type};
use anyhow::{Context, Result};

pub fn list(type_: Type) -> Result<Vec<String>> {
    let dirs = setup::dirs()?;
    let dir = dirs
        .get(&type_.into())
        .expect("Type should map to setup::Dirs");
    let mut list: Vec<String> = Vec::new();

    // TODO better error handling
    for entry in dir.read_dir().with_context(|| {
        format!(
            "Directory {:?} should exist after running `wut setup`",
            &dir
        )
    })? {
        if let Ok(entry) = entry {
            list.push(
                entry
                    .file_name()
                    .into_string()
                    .expect("File name should exist"),
            );
        }
    }

    Ok(list)
}
