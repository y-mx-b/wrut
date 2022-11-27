use clap::Args;

#[derive(Args, Debug)]
pub struct NewArgs {
    /// The name of the template to initialize.
    pub name: String,
}
