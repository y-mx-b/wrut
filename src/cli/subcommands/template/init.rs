use clap::Args;

#[derive(Args, Debug)]
pub struct InitArgs {
    /// The name of the template to initialize.
    ///
    /// By default, the name of the current directory will be used.
    pub name: Option<String>,
}
