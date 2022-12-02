use crate::setup::{dir, Dirs, SetupFlag};
use anyhow::Result;
use std::fs;

/// Overwrite the directory associated with given a `Dirs` variant.
///
/// If the directory does not exist, it will be created.
fn overwrite_dir(d: Dirs) -> Result<()> {
    let dir_path = dir(d)?;
    if dir_path.is_dir() {
        fs::remove_dir_all(&dir_path)?;
    }

    fs::create_dir(&dir_path)?;

    // Special case for overwriting data dir
    if d == Dirs::Data {
        fs::create_dir(dir(Dirs::Obj)?)?;
        fs::create_dir(dir(Dirs::Projects)?)?;
        fs::create_dir(dir(Dirs::Tags)?)?;
        fs::create_dir(dir(Dirs::Templates)?)?;
    }

    Ok(())
}

/// Given a `SetupFlag` variant, overwrite and/or initialize the directories and files associated
/// with the given flag.
pub fn overwrite(flag: SetupFlag) -> Result<()> {
    Ok(match flag {
        SetupFlag::All => {
            overwrite_dir(Dirs::Data)?;
            overwrite_dir(Dirs::Config)?;
        }
        SetupFlag::Data => overwrite_dir(Dirs::Data)?,
        SetupFlag::Obj => overwrite_dir(Dirs::Obj)?,
        SetupFlag::Projects => overwrite_dir(Dirs::Projects)?,
        SetupFlag::Tags => overwrite_dir(Dirs::Tags)?,
        SetupFlag::Templates => overwrite_dir(Dirs::Templates)?,
    })
}
