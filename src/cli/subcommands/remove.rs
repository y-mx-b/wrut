use crate::cli::Type;
use clap::Args;

// TODO literally everything here
#[derive(Args, Debug)]
pub struct RemoveArgs {
    #[clap(long, short, value_enum, default_value_t = Type::Project)]
    type_: Type,
}
