use clap::Args;

/// Contains args for `init` subcommand
#[derive(Args, Debug)]
pub struct InitArgs {
    /// Overwrite existing configuration and data
    #[clap(short, long)]
    pub force: bool,
}
