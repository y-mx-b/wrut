pub mod template;

use crate::cli::subcommand::template::TemplateCommand;
use clap::{Parser, Subcommand};
use wrut::WrutError;

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

#[derive(Parser)]
pub struct SubcommandParser<Command: clap::Subcommand> {
    #[clap(subcommand)]
    pub command: Command,
}
