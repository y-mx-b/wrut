pub mod subcommand;

use crate::cli::subcommand::{template::TemplateCommand, SubcommandParser};
use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;
use wrut::WrutError;

#[derive(Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub command_type: CommandType,
    #[command(flatten)]
    pub verbose: Verbosity,
}

impl Cli {
    pub fn run(self) -> Result<(), WrutError> {
        self.command_type.run()
    }
}

#[derive(Subcommand)]
pub enum CommandType {
    Template(SubcommandParser<TemplateCommand>),
}

impl CommandType {
    pub fn run(self) -> Result<(), WrutError> {
        match self {
            CommandType::Template(parser) => parser.command.run(),
        }
    }
}
