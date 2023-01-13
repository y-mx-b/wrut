use clap::Args;
use clap_complete::Shell;

#[derive(Args, Debug)]
pub struct CompArgs {
    #[clap(default_value_t = Shell::Bash)]
    pub shell: Shell,
}
