use clap::Args;

#[derive(Args, Debug)]
pub struct InitArgs {
    /// Overwrite existing configuration and data
    #[clap(short, long)]
    force: bool,
}
