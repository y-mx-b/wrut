use crate::cli::subcommands::{project, tag, template};
use anyhow::Result;
use clap::{Parser, Subcommand};
use clap_complete::Shell;
use clap_verbosity_flag::Verbosity;
use wrut::{SetupFlag, Type};

///A utility to manage project templates.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub type_: Option<CommandType>,

    #[command(flatten)]
    pub verbose: Verbosity,

    /// Generate shell completions.
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
    pub setup: Vec<SetupFlag>,
}

#[derive(Subcommand, Debug)]
pub enum CommandType {
    /// Project related commands (alias: 'p').
    #[clap(alias = "p")]
    Project(project::CommandParser),

    /// Tag related commands (alias: 's').
    #[clap(alias = "s")]
    Tag(tag::CommandParser),

    /// Template related commands (alias: 't').
    #[clap(alias = "t")]
    Template(template::CommandParser),
}

impl CommandType {
    pub fn run(&self) -> Result<()> {
        match self {
            CommandType::Project(cmd) => cmd.command.run(),
            CommandType::Tag(cmd) => cmd.command.run(),
            CommandType::Template(cmd) => cmd.command.run(),
        }
    }
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
