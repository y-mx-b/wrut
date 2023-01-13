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
    /// List templates.
    #[clap(alias = "ls")]
    List,

    /// Initialize and register a template in the current directory.
    #[clap(alias = "i")]
    Init(InitArgs),

    /// Create and register a new template with the given name.
    #[clap(alias = "n")]
    New(NewArgs),

    /// Unregister and/or delete the given template.
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
