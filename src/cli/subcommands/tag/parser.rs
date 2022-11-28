use super::{ListArgs, NewArgs, RemoveArgs};
use anyhow::Result;
use clap::{Parser, Subcommand};
use wrut::*;

#[derive(Parser, Debug)]
pub struct CommandParser {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// List tags.
    ///
    /// If a tag is provided, then the projects/templates registered under the provided tag will be
    /// printed.
    #[clap(alias = "ls")]
    List(ListArgs),

    /// Register a new tag and/or register projects/templates under the given tag.
    #[clap(alias = "n")]
    New(NewArgs),

    /// Remove the given tag.
    #[clap(alias = "rm")]
    Remove(RemoveArgs),
}

impl Command {
    pub fn run(&self) -> Result<()> {
        Ok(match self {
            Command::List(args) => println!("{}", list::list_tags(&args.name)?),
            Command::New(args) => init::init_tag(&args.name, &args.templates, &args.projects)?,
            Command::Remove(args) => remove::remove_tag(&args.name)?,
        })
    }
}
