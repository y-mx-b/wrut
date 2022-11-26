mod backend;
mod cli;

use crate::cli::Cli;
use anyhow::Result;
use clap::Parser;
use log::info;

fn main() -> Result<()> {
    let cli = Cli::parse();

    env_logger::builder()
        .format_timestamp(None)
        .filter_level(cli.verbose.log_level_filter())
        .init();

    info!("Parsing command arguments.");
    match &cli.type_ {
        _ => Ok(())
    }
}
