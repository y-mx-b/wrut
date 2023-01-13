mod backend;
mod cli;

use crate::backend::{comp, setup};
use crate::cli::{Cli, Commands};
use anyhow::Result;
use clap::Parser;
use log::info;

fn main() -> Result<()> {
    let cli = Cli::parse();

    env_logger::builder()
        .format_timestamp(None)
        .filter_level(cli.verbose.log_level_filter())
        .init();

    match &cli.command {
        Commands::Comp(args) => {
            info!("Running subcommand `comp`.");
            info!("{:?}", args);
            comp::print_completions(args.shell);
            Ok(())
        }
        Commands::Setup(args) => {
            info!("Running subcommand `setup`.");
            info!("{:?}", args);
            setup::setup(args)?;
            Ok(())
        }
        Commands::List(args) => {
            info!("Running subcommand `list`.");
            info!("{:?}", args);
            // TODO write actual code here
            println!("List");
            Ok(())
        }
        Commands::Init(args) => {
            info!("Running subcommand `init`.");
            info!("{:?}", args);
            // TODO write actual code here
            println!("Init");
            Ok(())
        }
    }
}
