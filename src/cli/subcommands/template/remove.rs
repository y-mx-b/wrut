use clap::Args;

// TODO add option to delete the template directory
#[derive(Args, Debug)]
pub struct RemoveArgs {
    /// The template to unregister and/or delete.
    pub template: String,

    /// If set, it will recursively delete the template directory as well
    #[clap(long, short)]
    pub delete: bool,
}
