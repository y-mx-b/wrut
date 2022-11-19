use clap::{Args, ValueEnum};

#[derive(Args, Debug)]
pub struct InitArgs {
    /// The type of directory to initialize
    #[clap(value_enum, value_name = "TYPE", default_value_t = InitType::Project)]
    type_: InitType,
}

#[derive(ValueEnum, Debug, PartialEq, Eq, Clone)]
pub enum InitType {
    Project,
    Template,
}
