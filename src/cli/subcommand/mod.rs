pub mod template;

use clap::Parser;

#[derive(Parser)]
pub struct SubcommandParser<Command: clap::Subcommand> {
    #[clap(subcommand)]
    pub command: Command,
}
