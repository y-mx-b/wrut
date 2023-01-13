mod backend;
mod cli;

use crate::cli::{Cli, Commands};
use crate::backend::r#const::Dirs;
use anyhow::{Context, Result};
use clap::Parser;
use log::{info, warn};

fn main() -> Result<()> {
    let cli = Cli::parse();

    env_logger::builder()
        .format_timestamp(None)
        .filter_level(cli.verbose.log_level_filter())
        .init();

    match &cli.command {
        Commands::List(args) => {
            info!("Running subcommand `list`.");
            info!("{:?}", args);
            println!("List");
            Ok(())
        }
        Commands::Init(args) => {
            info!("Running subcommand `init`.");
            info!("{:?}", args);
            println!("Init");
            println!("{:?}", Dirs::dirs());
            Ok(())
        }
    }
}
