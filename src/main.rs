mod backend;
mod cli;

use crate::backend::{comp, setup};
use crate::cli::subcommands::{project, tag, template};
use crate::cli::{Cli, CommandType};
use anyhow::{Error, Result};
use clap::{CommandFactory, Parser};
use log::info;

fn main() -> Result<()> {
    let cli = Cli::parse();

    env_logger::builder()
        .format_timestamp(None)
        .filter_level(cli.verbose.log_level_filter())
        .init();

    info!("Parsing command arguments.");
    if let Some(type_) = &cli.type_ {
        Ok(match &type_ {
            // TODO implement literally all of this
            CommandType::Project(cmd) => match &cmd.command {
                project::Command::List => {}
                project::Command::Init(_args) => {}
                project::Command::New(_args) => {}
                project::Command::Remove(_args) => {}
            },
            CommandType::Tag(cmd) => match &cmd.command {
                tag::Command::List => {}
                tag::Command::Add => {}
                tag::Command::Remove => {}
            },
            CommandType::Template(cmd) => match &cmd.command {
                template::Command::List => {}
                template::Command::Init => {}
                template::Command::Add => {}
                template::Command::Remove => {}
            },
        })
    } else {
        if !&cli.setup.is_empty() {
            setup::setup(cli.setup)?;
            Ok(())
        } else if let Some(sh) = cli.sh {
            comp::print_completions(sh);
            Ok(())
        } else {
            println!("{}", Cli::command().render_help());
            Ok(())
        }
    }
}
