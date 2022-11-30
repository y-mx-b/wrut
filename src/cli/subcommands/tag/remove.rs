use crate::cli::util::get_values;
use clap::{builder::PossibleValuesParser, Args};
use wrut::Type;

#[derive(Args, Debug)]
pub struct RemoveArgs {
    /// The relevant tag.
    ///
    /// If `--templates` and `--projects` aren't set, the tag itself will be removed.
    #[clap(value_parser = PossibleValuesParser::new(get_values(Type::Tag)))]
    pub name: String,

    /// The templates to remove from this tag.
    #[clap(long, short, hide_possible_values = true)]
    #[clap(value_parser = PossibleValuesParser::new(get_values(Type::Template)))]
    pub templates: Vec<String>,

    /// The projects to remove from this tag.
    #[clap(long, short, hide_possible_values = true, value_delimiter = ',')]
    #[clap(value_parser = PossibleValuesParser::new(get_values(Type::Project)))]
    pub projects: Vec<String>,
}
