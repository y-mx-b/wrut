use clap::{Parser, Subcommand};
use crate::cli::subcommands::project::InitArgs;

#[derive(Parser, Debug)]
pub struct CommandParser {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[clap(alias = "ls")]
    List,
    Init(InitArgs),
    Add,
    #[clap(alias = "rm")]
    Remove,
}

