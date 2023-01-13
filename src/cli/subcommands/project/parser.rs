use super::{InitArgs, NewArgs, RemoveArgs};
use anyhow::Result;
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
    #[clap(alias = "i")]
    Init(InitArgs),
    #[clap(alias = "n")]
    New(NewArgs),
    #[clap(alias = "rm")]
    Remove(RemoveArgs),
}

impl Command {
    pub fn run(&self) -> Result<()> {
        Ok(match self {
            Command::List => {}
            Command::Init(_args) => {}
            Command::New(_args) => {}
            Command::Remove(_args) => {}
        })
    }
}
