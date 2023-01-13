use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WutError {
    // IO related errors
    #[error("Could not find home directory")]
    HomeDirectoryNotFound,

    // Setup errors
    #[error("{0:?} already exist(s) but the `force` option was not set")]
    SetupDirAlreadyExists(Vec<PathBuf>),
}
