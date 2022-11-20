use crate::backend::setup;
use crate::cli::subcommands::{CompArgs, InitArgs, InitType, ListArgs, RemoveArgs, SetupArgs};
use clap::{Parser, Subcommand, ValueEnum};
use clap_verbosity_flag::Verbosity;
use std::path::PathBuf;

/// Main cli struct
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    #[command(flatten)]
    pub verbose: Verbosity,
    /// A configuration file [default: ~/.config/wut/config.toml]
    #[clap(short, long, hide_default_value = true)]
    // TODO figure out how to make this safer
    #[clap(default_value = setup::file(setup::Files::Config).unwrap().into_os_string())]
    pub config: PathBuf,
}

/// Available subcommands
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Generate shell completions.
    #[clap(alias = "sh")]
    Comp(CompArgs),
    /// Initialize data/config directories.
    ///
    /// Will fail if setup directories exist and `--force` is not set.
    Setup(SetupArgs),
    /// List all items of the given type.
    #[clap(alias = "ls")]
    List(ListArgs),
    /// Initialize a new template or project directory.
    ///
    /// If an entry under the provided name already exists, then it will be overwritten.
    #[clap(alias = "i")]
    Init(InitArgs),
    /// Remove a template or project
    #[clap(alias = "rm")]
    Remove(RemoveArgs),
}

/// Types to operate on
#[derive(ValueEnum, Clone, Debug, PartialEq, Eq, Copy)]
pub enum Type {
    #[clap(alias = "p")]
    Project,
    #[clap(alias = "f")]
    Tag,
    #[clap(alias = "t")]
    Template,
}

impl From<InitType> for Type {
    fn from(item: InitType) -> Self {
        match item {
            InitType::Project => Type::Project,
            InitType::Template => Type::Template,
        }
    }
}
