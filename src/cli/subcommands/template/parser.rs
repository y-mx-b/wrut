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
    Init,
    #[clap(alias = "n")]
    New,
    #[clap(alias = "rm")]
    Remove,
}

impl Command {
    pub fn run(&self) -> Result<()> {
        Ok(match self {
            Command::List => {}
            Command::Init => {}
            Command::New => {}
            Command::Remove => {}
        })
    }
}
