use clap::Args;

/// Contains args for `setup` subcommand
#[derive(Args, Debug)]
pub struct SetupArgs {
    /// Overwrite existing configuration and data
    #[clap(short, long)]
    pub force: bool,
}
