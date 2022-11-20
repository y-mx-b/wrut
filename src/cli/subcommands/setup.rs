use clap::{Args, ValueEnum};

/// Contains args for `setup` subcommand.
#[derive(Args, Debug)]
pub struct SetupArgs {
    #[clap(short, long)]
    pub force: bool,
    #[clap(short, long, value_delimiter = ',')] 
    pub overwrite: Vec<SetupOverwrite>
}

#[derive(ValueEnum, Debug, PartialEq, Eq, Clone)]
pub enum SetupOverwrite {
    #[clap(alias = "a")]
    All,

    // Directories
    #[clap(alias = "d")]
    Data,
    #[clap(alias = "o")]
    Obj,
    #[clap(alias = "p")]
    Projects,
    #[clap(alias = "t")]
    Templates,
    #[clap(alias = "s")]
    Tags,

    // Files
    #[clap(alias = "c")]
    Config,
}
