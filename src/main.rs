mod cli;

use crate::cli::{Cli, Commands};
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
    }
}
