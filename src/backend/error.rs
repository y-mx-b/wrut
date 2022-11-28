use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WrutError {
    // IO related errors
    #[error("Could not find home directory.")]
    HomeDirectoryNotFound,

    // Init errors
    #[error("Failed to acquire the name of the directory at {0:?}.")]
    FailedToAcquireDirectoryName(PathBuf),

    // Config errors
    #[error("Failed to read configuration file at {0:?}.")]
    FailedToReadConfigFile(PathBuf),
}
