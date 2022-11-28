use crate::cli::util::get_values;
use clap::{builder::PossibleValuesParser, Args};
use wrut::Type;

#[derive(Args, Debug)]
pub struct ListArgs {
    /// Get information regarding the provided tag.
    #[clap(value_parser = PossibleValuesParser::new(get_values(Type::Tag)))]
    pub name: Option<String>,
}
