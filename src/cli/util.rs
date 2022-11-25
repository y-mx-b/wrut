use crate::backend::list::list;
use crate::cli::Type;
use clap::builder::PossibleValue;

pub fn get_values(type_: Type) -> Vec<PossibleValue> {
    list(type_)
        .expect("")
        .iter()
        .map(|project| PossibleValue::new(project))
        .collect()
}
