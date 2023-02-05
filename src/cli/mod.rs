pub mod subcommand;

use crate::cli::subcommand::CommandType;
use clap::Parser;
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
