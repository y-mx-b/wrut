use crate::cli::Type;
use clap::Args;

#[derive(Args, Debug)]
pub struct ListArgs {
    #[clap(value_enum, default_value_t = Type::Project)]
    type_: Type,
}
