use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WutError {
    // IO related errors
    #[error("Could not find home directory")]
    HomeDirectoryNotFound,
    #[error("{0:?}")]
    Io(#[from] std::io::Error),

    // Init errors
    #[error("{0:?} already exist(s) but the `force` option was not set")]
    InitDirAlreadyExists(Vec<PathBuf>),
}
