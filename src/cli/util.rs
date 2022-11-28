use clap::builder::PossibleValue;
use wrut::{list::list, Type};

/// Returns a vector of possible values for the given type.
///
/// If `wrut` has not been setup, then resulting vector will be empty.
pub fn get_values(type_: Type) -> Vec<PossibleValue> {
    list(type_)
        .unwrap_or(Vec::new())
        .iter()
        .map(|project| PossibleValue::new(project))
        .collect()
}
