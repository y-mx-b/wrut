use thiserror::Error;

#[derive(Error, Debug)]
pub enum WutError {
    #[error("Could not find home directory")]
    HomeDirectoryNotFound,
}
