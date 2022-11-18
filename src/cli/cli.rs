use crate::cli::subcommands::list::List;
use clap::{Parser, Subcommand, ValueEnum};
use clap_verbosity_flag::Verbosity;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    #[command(flatten)]
    pub verbose: Verbosity,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[clap(alias = "ls")]
    List(List),
}

#[derive(ValueEnum, Clone, Debug, PartialEq, Eq)]
pub enum Type {
    #[clap(alias = "p")]
    Project,
    Tag,
    #[clap(alias = "t")]
    Template,
}
