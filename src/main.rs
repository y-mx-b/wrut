mod cli;

use crate::cli::{Cli, Commands};
use clap::Parser;
use log::{info, warn};

fn main() {
    let cli = Cli::parse();

    env_logger::builder()
        .format_timestamp(None)
        .filter_level(cli.verbose.log_level_filter())
        .init();

    match &cli.command {
        Commands::List => {
            info!("Running subcommand `list`.");
            println!("List");
        }
    }
}
