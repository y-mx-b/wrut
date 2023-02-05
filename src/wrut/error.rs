use std::ffi::OsString;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WrutError {
    // TODO: better from impl
    #[error("Error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Expected directory, found file: {0:?}")]
    ExpectedDirectory(PathBuf),

    #[error("Expected UTF-8 name, found: {0:?}")]
    ExpectedUtf8(OsString),
}
