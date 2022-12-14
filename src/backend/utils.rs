use crate::WrutError;
use anyhow::Result;
use std::path::Path;

/// Acquire the name to use. If `name` is `None`, the name of the directory provided by `dir` will
/// be used.
pub fn get_name(name: &Option<&str>, dir: &Path) -> Result<String> {
    Ok(match name {
        Some(val) => val.to_string(),
        None => dir
            .file_name()
            .ok_or_else(|| WrutError::FailedToAcquireDirectoryName(dir.to_path_buf()))?
            .to_str()
            .ok_or_else(|| WrutError::FailedToAcquireDirectoryName(dir.to_path_buf()))?
            .to_string(),
    })
}
