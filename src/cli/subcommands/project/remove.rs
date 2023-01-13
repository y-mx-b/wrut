use crate::cli::util::get_values;
use clap::{builder::PossibleValuesParser, Args};
use wrut::Type;

#[derive(Args, Debug)]
pub struct RemoveArgs {
    /// The project to delete.
    #[clap(hide_possible_values = true,
        value_parser = PossibleValuesParser::new(get_values(Type::Project)))]
    pub project: String,

    /// If set, it will recursively delete the project directory as well
    #[clap(long, short)]
    pub delete: bool,
}
