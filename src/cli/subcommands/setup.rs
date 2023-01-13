use clap::Args;

#[derive(Args, Debug)]
pub struct SetupArgs {
    #[clap(long, short)]
    all: bool
}

