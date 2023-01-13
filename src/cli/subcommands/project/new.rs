use crate::cli::util::get_values;
use wrut::Type;
use clap::{builder::PossibleValuesParser, Args};

#[derive(Args, Debug)]
pub struct NewArgs {
    /// The template to initialize a project from.
    #[clap(long, short, hide_possible_values = true)]
    #[clap(value_parser = PossibleValuesParser::new(get_values(Type::Template)))]
    pub template: String,
    /// The name of the project to initialize.
    pub name: String,
}
