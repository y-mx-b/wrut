use crate::cli::Cli;
use clap::CommandFactory;
use clap_complete::{generate, Generator};
use std::io;

/// Print shell completions to stdout
pub fn print_completions<G: Generator>(gen: G) {
    let cmd = &mut Cli::command();
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}
