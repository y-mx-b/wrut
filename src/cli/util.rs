use crate::backend::list::list;
use crate::cli::Type;
use clap::builder::PossibleValue;

// TODO currently requires directly to already be setup, do so upon first running this command
pub fn get_values(type_: Type) -> Vec<PossibleValue> {
    list(type_)
        .expect("")
        .iter()
        .map(|project| PossibleValue::new(project))
        .collect()
}
