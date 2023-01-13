use crate::cli::util::get_values;
use wrut::Type;
use clap::{builder::PossibleValuesParser, Args};

#[derive(Args, Debug)]
pub struct ListArgs {
    /// Get information regarding the provided tag.
    #[clap(value_parser = PossibleValuesParser::new(get_values(Type::Tag)))]
    pub name: Option<String>,
}
