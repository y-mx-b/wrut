mod cli;

use crate::cli::{subcommands::comp, Cli};
use anyhow::Result;
use clap::{CommandFactory, Parser};
use log::info;
use wrut::setup;

fn main() -> Result<()> {
    let cli = Cli::parse();

    env_logger::builder()
        .format_timestamp(None)
        .filter_level(cli.verbose.log_level_filter())
        .init();

    info!("Parsing command arguments.");
    // check for setup, then completions, then commands
    if !&cli.setup.is_empty() {
        setup::setup(cli.setup)
    } else if let Some(sh) = cli.sh {
        Ok(comp::print_completions(sh))
    } else if let Some(type_) = &cli.type_ {
        type_.run()
    } else {
        // print help message if nothing else is given
        Ok(println!("{}", Cli::command().render_help()))
    }
}
