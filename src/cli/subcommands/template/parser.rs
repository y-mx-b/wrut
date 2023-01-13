use super::{InitArgs, RemoveArgs};
use anyhow::Result;
use clap::{Parser, Subcommand};
use std::env::current_dir;
use wrut::Template;

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
    pub fn run(&self) -> Result<()> {
        Ok(match self {
            Command::List => println!("{}", Template::list()?.join("\n")),
            Command::Init(args) => { let _ = Template::from(current_dir()?, args.name.as_deref())?.init()?.add_tags(&args.tags); },
            Command::Remove(args) => Template::get(&args.template)?.remove(args.delete)?,
        })
    }
}
