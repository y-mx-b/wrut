use crate::cli::util::get_values;
use clap::{builder::PossibleValuesParser, Args};
use wrut::Type;

#[derive(Args, Debug)]
pub struct NewArgs {
    /// The template to initialize a project from.
    #[clap(hide_possible_values = true, value_parser = PossibleValuesParser::new(get_values(Type::Template)))]
    pub template: String,
    /// The name of the project to initialize.
    pub name: String,

    /// Tags to add to the project
    #[clap(long, short, value_delimiter = ',')]
    #[clap(hide_possible_values = true, value_parser = PossibleValuesParser::new(get_values(Type::Tag)))]
    pub tags: Vec<String>,
}
