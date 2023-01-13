use crate::cli::util::get_values;
use clap::{builder::PossibleValuesParser, Args};
use wrut::Type;

// TODO add option to delete the project directory
#[derive(Args, Debug)]
pub struct RemoveArgs {
    /// The project to delete.
    #[clap(hide_possible_values = true,
        value_parser = PossibleValuesParser::new(get_values(Type::Project)))]
    pub project: String,
}
