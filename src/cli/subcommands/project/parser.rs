use super::{InitArgs, NewArgs, RemoveArgs};
use anyhow::Result;
use clap::{Parser, Subcommand};
use std::env::current_dir;
use std::fs;
use std::path::PathBuf;
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
    pub fn run(&self, config: PathBuf) -> Result<()> {
        Ok(match self {
            Command::List => println!("{}", list::list(Type::Project)?.join("\n")),
            Command::Init(args) => {
                init::init_project(&args.template, &current_dir()?, &args.name, config)?
            }
            Command::New(args) => {
                let project_dir = current_dir()?.join(&args.name);
                fs::create_dir(&project_dir)?;
                // TODO remove this clone
                // guess i should replace String with &str and some lifetimes?
                init::init_project(
                    &args.template,
                    &project_dir,
                    &Some(args.name.clone()),
                    config,
                )?
            }
            Command::Remove(args) => remove::remove_project(&args.project)?,
        })
    }
}
