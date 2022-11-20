mod backend;
mod cli;

use crate::backend::{comp, config, init, list, setup};
use crate::cli::{Cli, Commands};
use anyhow::Result;
use clap::Parser;
use log::info;
use std::env::current_dir;

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
            let list = list::list(args.type_)?;
            println!("{}", list.join("\t"));
            Ok(())
        }
        Commands::Init(args) => {
            info!("Running subcommand `init`.");
            info!("{:?}", args);
            init::init(current_dir()?, &args, config::get_config(cli.config)?)?;
            Ok(())
        }
    }
}
