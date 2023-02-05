use crate::utils;
use clap::{Args, Subcommand};
use std::env::current_dir;
use wrut::WrutError;

#[derive(Subcommand)]
pub enum TemplateCommand {
    Init(InitArgs),
}

#[derive(Args)]
pub struct InitArgs {
    name: Option<String>,
}

impl TemplateCommand {
    pub fn run(self) -> Result<(), WrutError> {
        match self {
            TemplateCommand::Init(args) => {
                let path = current_dir()?;
                let name = utils::check_name(args.name, &path)?;

                log::info!("Path: {}", path.display());
                log::info!("Name: {}", name);

                todo!();
            }
        }
    }
}
