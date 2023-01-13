use crate::backend::setup;
use clap::{Parser, Subcommand, ValueEnum};
use clap_verbosity_flag::Verbosity;
use std::path::PathBuf;
use crate::cli::subcommands::project::ProjectCommandParser;
use crate::cli::subcommands::setup::SetupArgs;

/// Main cli struct
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command_type: CommandType,

    /// A configuration file [default: ~/.config/wut/config.toml]
    #[clap(short, long, hide_default_value = true)]
    // TODO figure out how to make this safer
    #[clap(default_value = setup::file(setup::Files::Config).unwrap().into_os_string())]
    pub config: PathBuf,
    #[command(flatten)]
    pub verbose: Verbosity,
    #[clap(flatten)]
    pub setup: SetupArgs
}

#[derive(Subcommand, Debug)]
pub enum CommandType {
    #[clap(alias = "p")]
    Project(ProjectCommandParser),
    #[clap(alias = "f")]
    Tag,
    #[clap(alias = "t")]
    Template,
}

/// Types to operate on
#[derive(ValueEnum, Clone, Debug, PartialEq, Eq, Copy)]
pub enum Type {
    Project,
    Tag,
    Template,
}
