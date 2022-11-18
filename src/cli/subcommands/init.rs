use clap::Args;

#[derive(Args, Debug)]
pub struct Init {
    /// Overwrite existing configuration and data
    #[clap(short, long)]
    force: bool,
}
