use clap::{Args, ValueEnum};

#[derive(Args, Debug, Clone, Copy, Default)]
pub struct SetupArgs {
    pub all: bool,

    // Directories
    pub data: bool,
    pub obj: bool,
    pub projects: bool,
    pub templates: bool,
    pub tags: bool,

    // Files
    pub config: bool,
}

#[derive(ValueEnum, Debug, PartialEq, Eq, Clone)]
pub enum SetupFlags {
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

impl From<Vec<SetupFlags>> for SetupArgs {
    fn from(item: Vec<SetupFlags>) -> Self {
        let mut args = SetupArgs::default();

        for flag in item {
            match flag {
                SetupFlags::All => args.all = true,

                SetupFlags::Data => args.data = true,
                SetupFlags::Obj => args.obj = true,
                SetupFlags::Projects => args.projects = true,
                SetupFlags::Templates => args.templates = true,
                SetupFlags::Tags => args.tags = true,

                SetupFlags::Config => args.config = true,
            }
        }

        args
    }
}
