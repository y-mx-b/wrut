use crate::backend::setup;
use crate::cli::subcommands::SetupFlags;
use crate::cli::subcommands::{project, tag, template};
use clap::{Parser, Subcommand};
use clap_complete::Shell;
use clap_verbosity_flag::Verbosity;
use std::path::PathBuf;

/// Main cli struct
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub type_: Option<CommandType>,

    /// A configuration file to use for configurations [default: ~/.config/wrut/config.toml]
    #[clap(short, long, hide_default_value = true)]
    // TODO figure out how to make this safer
    #[clap(default_value = setup::file(setup::Files::Config)
            .expect("Could not get home directory.")
            .to_str()
            .expect("Could not get string")
            .to_string())]
    pub config: PathBuf,
    #[command(flatten)]
    pub verbose: Verbosity,

    /// Generate shell completions
    #[clap(exclusive = true, long, short = 'z')]
    pub sh: Option<Shell>,

    /// Setup and/or restore directories to their defaults.
    ///
    /// Arguments are provided in the following format:
    ///
    /// `--setup projects,templates,config`
    ///
    /// Alternatively, you can use the shorter aliases:
    ///
    /// `--setup p,t,c`
    #[clap(
        exclusive = true,
        long,
        short,
        value_delimiter = ',',
        value_name = "DIRECTORIES"
    )]
    pub setup: Vec<SetupFlags>,
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
