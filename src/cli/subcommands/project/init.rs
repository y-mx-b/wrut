use crate::cli::util::get_values;
use clap::{builder::PossibleValuesParser, Args};
use crate::cli::Type;

#[derive(Args, Debug)]
pub struct InitArgs {
    /// The template to initialize a project from.
    #[clap(long, short, hide_possible_values = true)]
    #[clap(value_parser = PossibleValuesParser::new(get_values(Type::Template)))]
    pub template: String,
    /// The name of the project to initialize.
    ///
    /// By default, the name of the current directory will be used.
    pub name: Option<String>
}
