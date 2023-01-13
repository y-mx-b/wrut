use crate::cli::util::get_values;
use wrut::Type;
use clap::{builder::PossibleValuesParser, Args};

#[derive(Args, Debug)]
pub struct RemoveArgs {
    /// The tag to delete.
    #[clap(value_parser = PossibleValuesParser::new(get_values(Type::Tag)))]
    pub name: String,
}
