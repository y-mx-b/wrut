use crate::cli::util::get_values;
use clap::{builder::PossibleValuesParser, Args};
use wrut::Type;

#[derive(Args, Debug)]
pub struct NewArgs {
    /// The name of the tag to register.
    ///
    /// If the tag already exists, the provided templates and projects will be unregistered from
    /// this tag.
    pub name: String,

    /// The templates to register with this tag.
    #[clap(long, short, hide_possible_values = true)]
    #[clap(value_parser = PossibleValuesParser::new(get_values(Type::Template)))]
    pub templates: Vec<String>,

    /// The projects to register with this tag.
    #[clap(long, short, hide_possible_values = true, value_delimiter = ',')]
    #[clap(value_parser = PossibleValuesParser::new(get_values(Type::Project)))]
    pub projects: Vec<String>,
}
