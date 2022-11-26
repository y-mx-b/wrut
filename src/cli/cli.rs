use crate::backend::setup;
use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;
use std::path::PathBuf;
use crate::cli::subcommands::{project, template, tag};
use crate::cli::subcommands::SetupArgs;

/// Main cli struct
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub type_: CommandType,

    /// A configuration file [default: ~/.config/wut/config.toml]
    #[clap(short, long, hide_default_value = true)]
    // TODO figure out how to make this safer
    #[clap(default_value = setup::file(setup::Files::Config).unwrap().into_os_string())]
    pub config: PathBuf,
    #[command(flatten)]
    pub verbose: Verbosity,
    #[clap(flatten)]
    pub setup: SetupArgs
}

#[derive(Subcommand, Debug)]
pub enum CommandType {
    #[clap(alias = "p")]
    Project(project::CommandParser),
    #[clap(alias = "s")]
    Tag(tag::CommandParser),
    #[clap(alias = "t")]
    Template(template::CommandParser),
}

/// Types to operate on
#[derive(Debug)]
pub enum Type {
    Project,
    Tag,
    Template,
}

impl From<CommandType> for Type {
    fn from(item: CommandType) -> Self {
        match item {
            CommandType::Project(_) => Self::Project,
            CommandType::Tag(_) => Self::Tag,
            CommandType::Template(_) => Self::Template,
        }
    }
}
