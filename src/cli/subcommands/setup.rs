use clap::Args;

#[derive(Args, Debug)]
pub struct SetupArgs {
    #[clap(alias = "a")]
    pub all: bool,
   
    // Directories
    #[clap(alias = "d")]
    pub data: bool,
    #[clap(alias = "o")]
    pub obj: bool,
    #[clap(alias = "p")]
    pub projects: bool,
    #[clap(alias = "t")]
    pub templates: bool,
    #[clap(alias = "s")]
    pub tags: bool,

    // Files
    #[clap(alias = "c")]
    pub config: bool,
}

