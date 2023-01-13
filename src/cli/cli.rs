use clap::{Parser, Subcommand};
use clap_verbosity_flag::{Verbosity};
use crate::cli::subcommands::list::List;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    #[command(flatten)]
    pub verbose: Verbosity
}

#[derive(Subcommand)]
pub enum Commands {
    List,
}
