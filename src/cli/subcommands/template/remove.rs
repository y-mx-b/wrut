use clap::Args;

#[derive(Args, Debug)]
pub struct RemoveArgs {
    /// The template to unregister and/or delete.
    pub name: String,
}
