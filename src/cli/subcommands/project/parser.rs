use clap::{Parser, Subcommand};
use crate::cli::subcommands::project::InitArgs;

#[derive(Parser, Debug)]
pub struct ProjectCommandParser {
    #[clap(subcommand)]
    pub command: ProjectCommands,
}

#[derive(Subcommand, Debug)]
pub enum ProjectCommands {
    #[clap(alias = "ls")]
    List,
    #[clap(alias = "i")]
    Init(InitArgs),
    #[clap(alias = "a")]
    Add,
    #[clap(alias = "rm")]
    Remove,
}

