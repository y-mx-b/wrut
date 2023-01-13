use home::home_dir;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum Dirs {
    Wut,
    Config,
    Data,
}

impl Dirs {
    pub fn dirs() -> HashMap<Dirs, PathBuf> {
        // TODO use actual errors instead of expect
        HashMap::from([
            (Dirs::Wut, home_dir().expect("Failed to get home directory").join(".wut")),
            (Dirs::Config, home_dir().expect("Failed to get home directory").join(".config/wut")),
            (Dirs::Data, home_dir().expect("Failed to get home directory").join(".wut/data"))
        ])
    }
}
