use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WutError {
    // Missing directories/files
    #[error("Could not find home directory")]
    HomeDirectoryNotFound,
    #[error("{0:?}")]
    Io(#[from] std::io::Error),

    // Init
    #[error("{0:?} already exist(s) but the `force` option was not set")]
    InitDirAlreadyExists(Vec<PathBuf>),
}
