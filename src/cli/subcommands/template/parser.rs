use super::{InitArgs, RemoveArgs};
use anyhow::Result;
use clap::{Parser, Subcommand};
use wrut::*;
use std::env::current_dir;

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

    /// Unregister and/or delete the given template.
    #[clap(alias = "rm")]
    Remove(RemoveArgs),
}

impl Command {
    // TODO literall all of this
    pub fn run(&self) -> Result<()> {
        Ok(match self {
            Command::List => println!("{}", list::list(Type::Template)?.join("\n")),
            Command::Init(args) => init::init_template(current_dir()?, &args.name)?,
            Command::Remove(_args) => {}
        })
    }
}
