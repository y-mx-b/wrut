use clap::{builder::PossibleValuesParser, Args, ValueEnum};
use crate::cli::util::get_values;
use crate::cli::Type;

/// Contains args for `init` subcommand.
#[derive(Args, Debug)]
pub struct InitArgs {
    /// The type of directory to initialize.
    #[clap(value_enum, value_name = "TYPE", default_value_t = InitType::Project)]
    pub type_: InitType,
    /// The name of the project/template to register.
    ///
    /// By default, the name of the current directory will be used.
    #[clap(long, short)]
    pub name: Option<String>,
    /// The template to initialize a project from.
    ///
    /// A template is required when initialized a project. Will be ignored when initializing a
    /// template.
    #[clap(long, short, hide_possible_values = true, value_parser = PossibleValuesParser::new(get_values(Type::Template)))]
    pub template: Option<String>,
}

// TODO figure out how to remove this, it's redundant and annoying
/// The types allowed to be initialized from directories
#[derive(ValueEnum, Debug, PartialEq, Eq, Clone, Copy)]
pub enum InitType {
    #[clap(alias = "p")]
    Project,
    #[clap(alias = "t")]
    Template,
}
