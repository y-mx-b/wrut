pub mod config;
mod error;
pub mod list;
pub mod setup;

mod project;
mod tag;
mod template;
mod utils;

pub use error::WrutError;
pub use project::Project;
pub use tag::Tag;
pub use template::Template;

/// Types to operate on
#[derive(Debug)]
pub enum Type {
    Project,
    Tag,
    Template,
}
