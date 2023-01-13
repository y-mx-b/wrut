use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct CommandParser {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    #[clap(alias = "ls")]
    List,
    Init,
    Add,
    #[clap(alias = "rm")]
    Remove,
}
