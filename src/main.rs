mod backend;
mod cli;

use crate::cli::Cli;
use crate::backend::{comp, setup};
use anyhow::{Result, Error};
use clap::{Parser, CommandFactory};
use log::info;

fn main() -> Result<()> {
    let cli = Cli::parse();

    env_logger::builder()
        .format_timestamp(None)
        .filter_level(cli.verbose.log_level_filter())
        .init();

    info!("Parsing command arguments.");
    if let Some(type_) = &cli.type_ {
        match &type_ {
                _ => Ok(())
        }
    } else {
        if !&cli.setup.is_empty() { 
            setup::setup(&cli.setup.into())?;
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
