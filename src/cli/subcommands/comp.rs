use clap::Args;
use clap_complete::Shell;

/// Contains args for `comp` subcommand
#[derive(Args, Debug)]
pub struct CompArgs {
    /// The shell to generate completions for
    #[clap(default_value_t = Shell::Bash)]
    pub shell: Shell,
}
