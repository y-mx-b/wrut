use crate::backend::setup::{dir, Dirs};
use anyhow::Result;
use std::path::PathBuf;

pub struct Tag {
    pub name: String,
}

impl Tag {
    pub fn from(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    pub fn path(&self) -> Result<PathBuf> {
        Ok(dir(Dirs::Tags)?.join(&self.name))
    }
}
