use crate::cli::subcommands::{InitArgs, ListArgs};
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
    #[clap(alias = "ls")]
    /// List all items of the given type
    List(ListArgs),
    /// Initialize data/config directories
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
