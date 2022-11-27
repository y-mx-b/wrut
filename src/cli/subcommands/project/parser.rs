use super::{InitArgs, NewArgs, RemoveArgs};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct CommandParser {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[clap(alias = "ls")]
    List,
    #[clap(alias = "i")]
    Init(InitArgs),
    #[clap(alias = "n")]
    New(NewArgs),
    #[clap(alias = "rm")]
    Remove(RemoveArgs),
}
