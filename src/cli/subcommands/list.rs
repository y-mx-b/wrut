use crate::cli::Type;
use clap::Args;

/// Contains args for `list` subcommand
#[derive(Args, Debug)]
pub struct ListArgs {
    /// Type to operate on
    #[clap(value_enum, default_value_t = Type::Project)]
    type_: Type,
}
