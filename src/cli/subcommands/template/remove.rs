use clap::Args;

// TODO add option to delete the template directory
#[derive(Args, Debug)]
pub struct RemoveArgs {
    /// The template to unregister and/or delete.
    pub template: String,
}
