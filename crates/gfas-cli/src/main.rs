use clap::Parser;
use cli::{Cli, Commands};

mod cli;
mod generate;
mod logging;
mod sync;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let Cli { verbose, no_color, command } = Cli::parse();

    logging::setup(verbose, no_color)?;

    match command {
        Commands::Generate { mode } => generate::generate(mode),
        Commands::Sync { user, token } => sync::sync(&user, &token).await
    }
}
