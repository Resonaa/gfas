use std::io;

use clap::{Command, CommandFactory, ValueEnum};
use clap_complete::aot::{generate as generate_completions, Generator, Shell};

use crate::cli::Cli;

/// The thing to generate via the generate subcommand
#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum GenerateMode {
    /// Raw roff used for the man page
    Man,

    /// Completions for bash
    Bash,

    /// Completions for zsh
    Zsh,

    /// Completions for fish
    Fish,

    /// Completions for Elvish
    Elvish,

    /// Completions for PowerShell
    #[value(name = "ps")]
    PowerShell
}

fn complete<G: Generator>(gen: G, mut cmd: Command) {
    let bin_name = cmd.get_name().to_string();
    generate_completions(gen, &mut cmd, bin_name, &mut io::stdout());
}

/// Generates man page or shell completions.
pub fn generate(mode: GenerateMode) -> anyhow::Result<()> {
    let cmd = Cli::command();

    match mode {
        GenerateMode::Man => {
            clap_mangen::Man::new(Cli::command()).render(&mut io::stdout())?;
        }
        GenerateMode::Bash => {
            complete(Shell::Bash, cmd);
        }
        GenerateMode::Zsh => {
            complete(Shell::Zsh, cmd);
        }
        GenerateMode::Fish => {
            complete(Shell::Fish, cmd);
        }
        GenerateMode::Elvish => {
            complete(Shell::Elvish, cmd);
        }
        GenerateMode::PowerShell => {
            complete(Shell::PowerShell, cmd);
        }
    }

    Ok(())
}
