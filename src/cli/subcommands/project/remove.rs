use crate::cli::util::get_values;
use clap::{builder::PossibleValuesParser, Args};
use wrut::Type;

#[derive(Args, Debug)]
pub struct RemoveArgs {
    /// The project to delete.
    #[clap(long, short, hide_possible_values = true)]
    #[clap(value_parser = PossibleValuesParser::new(get_values(Type::Project)))]
    pub project: String,
}
