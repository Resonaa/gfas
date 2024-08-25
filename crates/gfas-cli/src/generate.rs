use std::io;

use clap::CommandFactory;

use crate::cli::{Cli, GenerateMode};

/// Generates man page or shell completions.
pub fn generate(mode: GenerateMode) -> anyhow::Result<()> {
    match mode {
        GenerateMode::Man => {
            clap_mangen::Man::new(Cli::command()).render(&mut io::stdout())?;
            Ok(())
        }
        _ => unimplemented!()
    }
}
