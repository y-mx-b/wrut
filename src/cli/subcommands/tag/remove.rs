use crate::cli::util::get_values;
use clap::{builder::PossibleValuesParser, Args};
use wrut::Type;

#[derive(Args, Debug)]
pub struct RemoveArgs {
    /// The tag to delete.
    #[clap(value_parser = PossibleValuesParser::new(get_values(Type::Tag)))]
    pub name: String,
}
