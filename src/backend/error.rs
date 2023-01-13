use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WutError {
    // IO related errors
    #[error("Could not find home directory.")]
    HomeDirectoryNotFound,

    // Setup errors
    #[error("{0:?} already exist(s) but the `force` option was not set.")]
    SetupDirAlreadyExists(Vec<PathBuf>),

    // Init errors
    #[error("Failed to acquire the name of the directory at {0:?}.")]
    FailedToAcquireDirectoryName(PathBuf),

    // Config errors
    #[error("Failed to read configuration file at {0:?}.")]
    FailedToReadConfigFile(PathBuf),
}
