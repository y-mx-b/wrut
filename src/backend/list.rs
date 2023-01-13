use crate::{setup, Type};
use anyhow::{Context, Result};

/// List the entries of a given type.
///
/// If no such entries exist, or the required data directory itself does not exist, then an empty
/// vector will be returned.
pub fn list(type_: Type) -> Result<Vec<String>> {
    let dir = setup::dir(type_.into())?;
    let mut list: Vec<String> = Vec::new();

    // TODO better error handling
    for entry in dir.read_dir().with_context(|| {
        format!(
            "Directory {:?} should exist after running `wrut --setup`",
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
