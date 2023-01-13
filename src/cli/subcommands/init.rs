use clap::{Args, ValueEnum};

#[derive(Args, Debug)]
pub struct InitArgs {
    /// The type of directory to initialize
    #[clap(value_enum, value_name = "TYPE", default_value_t = InitType::Project)]
    pub type_: InitType,
    #[clap(long, short)]
    pub name: Option<String>,
}

#[derive(ValueEnum, Debug, PartialEq, Eq, Clone, Copy)]
pub enum InitType {
    Project,
    Template,
}
