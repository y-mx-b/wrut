use crate::cli::subcommands::{CompArgs, InitArgs, InitType, ListArgs, SetupArgs};
use clap::{Parser, Subcommand, ValueEnum};
use clap_verbosity_flag::Verbosity;

/// Main cli struct
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    #[command(flatten)]
    pub verbose: Verbosity,
}

/// Available subcommands
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Generate shell completions
    Comp(CompArgs),
    /// Initialize data/config directories
    Setup(SetupArgs),
    #[clap(alias = "ls")]
    /// List all items of the given type
    List(ListArgs),
    /// Initialize a new template or project directory
    Init(InitArgs),
}

/// Types to operate on
#[derive(ValueEnum, Clone, Debug, PartialEq, Eq)]
pub enum Type {
    #[clap(alias = "p")]
    Project,
    Tag,
    #[clap(alias = "t")]
    Template,
}

impl From<InitType> for Type {
    fn from(item: InitType) -> Self {
        match item {
            InitType::Project => Type::Project,
            InitType::Template => Type::Template,
        }
    }
}
