use super::{InitArgs, NewArgs, RemoveArgs};
use anyhow::Result;
use clap::{Parser, Subcommand};
use std::env::current_dir;
use wrut::*;

#[derive(Parser, Debug)]
pub struct CommandParser {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// List projects.
    #[clap(alias = "ls")]
    List,

    /// Initialize and register a new project in the current directory.
    #[clap(alias = "i")]
    Init(InitArgs),

    /// Create and register a new project with a given name.
    #[clap(alias = "n")]
    New(NewArgs),

    /// Unregister and/or delete the given project.
    #[clap(alias = "rm")]
    Remove(RemoveArgs),
}

impl Command {
    pub fn run(&self) -> Result<()> {
        Ok(match self {
            Command::List => println!("{}", Project::list()?.join("\n")),
            Command::Init(args) => {
                Project::from(current_dir()?, args.name.as_deref())?
                    .init(&Template::get(&args.template)?)?
                    .add_tags(&args.tags)?;
            }
            Command::New(args) => {
                Project::from(current_dir()?.join(&args.name), Some(&args.name))?
                    .new_init(&Template::get(&args.template)?)?
                    .add_tags(&args.tags)?;
            }
            Command::Remove(args) => Project::get(&args.project)?.remove(args.delete)?,
        })
    }
}
