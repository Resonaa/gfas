use clap::Parser;
use cli::Cli;

mod cli;
mod logging;
mod sync;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let Cli { verbose, no_color, token, endpoint, dry_run } = Cli::parse();

	logging::setup(verbose, no_color)?;

	sync::sync(token, endpoint, dry_run).await
}
