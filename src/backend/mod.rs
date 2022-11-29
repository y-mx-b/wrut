pub mod config;
mod error;
pub mod init;
pub mod list;
pub mod remove;
pub mod setup;

pub use error::*;

mod project;
mod template;
mod utils;

pub use project::*;
pub use template::*;

/// Types to operate on
#[derive(Debug)]
pub enum Type {
    Project,
    Tag,
    Template,
}
