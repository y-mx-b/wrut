use crate::setup::overwrite;
use anyhow::Result;
use clap::ValueEnum;

/// A set of flags to determine what directory to overwrite.
#[derive(ValueEnum, Debug, PartialEq, Eq, Clone)]
pub enum SetupFlag {
    /// Restore everything to default (alias: 'a').
    #[clap(alias = "a")]
    All,

    /// Set `~/.wrut` to default (alias: 'd').
    #[clap(alias = "d")]
    Data,
    /// Set `~/.wrut/.obj` to default (alias: 'o').
    #[clap(alias = "o")]
    Obj,
    /// Set `~/.wrut/projects` to default (alias: 'p').
    #[clap(alias = "p")]
    Projects,
    /// Set `~/.wrut/templates` to default (alias: 't').
    #[clap(alias = "t")]
    Templates,
    /// Set `~/.wrut/tags` to default (alias: 's').
    #[clap(alias = "s")]
    Tags,

    /// Set `~/.config/wrut` to default (alias: 'c').
    #[clap(alias = "c")]
    Config,
}

/// Given a set of flags, overwrite and/or initialize the directories associated with thsoe flags.
pub fn setup(flags: Vec<SetupFlag>) -> Result<()> {
    Ok(for flag in flags {
        overwrite(flag)?;
    })
}
