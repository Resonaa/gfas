use std::io;

use clap::{CommandFactory, ValueEnum};

use crate::cli::Cli;

/// The thing to generate via the generate subcommand
#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum GenerateMode {
    /// Raw roff used for the man page
    Man,

    /// Completions for bash
    CompleteBash,

    /// Completions for zsh
    CompleteZsh,

    /// Completions for fish
    CompleteFish,

    /// Completions for PowerShell
    CompletePowerShell
}

/// Generates man page or shell completions.
pub fn generate(mode: GenerateMode) -> anyhow::Result<()> {
    match mode {
        GenerateMode::Man => {
            clap_mangen::Man::new(Cli::command()).render(&mut io::stdout())?;
            Ok(())
        }
        _ => todo!()
    }
}
