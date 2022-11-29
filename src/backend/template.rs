use std::path::PathBuf;
use anyhow::Result;

pub struct Template {
    name: String,
    path: PathBuf,
}

impl Template {
    pub fn from(path: impl Into<PathBuf>, name: Option<&str>) -> Result<Self> {

    }
}
