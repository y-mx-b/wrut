use clap::{Args, ValueEnum};

/// Contains args for `init` subcommand
#[derive(Args, Debug)]
pub struct InitArgs {
    /// The type of directory to initialize
    #[clap(value_enum, value_name = "TYPE", default_value_t = InitType::Project)]
    pub type_: InitType,
    /// The name of the project/template to register
    #[clap(long, short)]
    pub name: Option<String>,
}

/// The types allowed to be initialized from directories
#[derive(ValueEnum, Debug, PartialEq, Eq, Clone, Copy)]
pub enum InitType {
    Project,
    Template,
}
