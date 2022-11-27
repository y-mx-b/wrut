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
    Add,
    #[clap(alias = "rm")]
    Remove,
}
