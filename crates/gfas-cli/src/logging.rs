#![allow(unexpected_cfgs)]

use std::env;
use std::io::{self, IsTerminal};

use clap_verbosity_flag::{InfoLevel, Verbosity};
use log::Level;
use tracing::level_filters::LevelFilter;
use tracing::subscriber::{set_global_default, SetGlobalDefaultError};

/// Sets up tracing filter and subscriber.
#[cfg(not(tarpaulin_include))]
pub fn setup(verbose: Verbosity<InfoLevel>, no_color: bool) -> Result<(), SetGlobalDefaultError> {
	let filter = match verbose.log_level() {
		None => LevelFilter::OFF,
		Some(Level::Error) => LevelFilter::ERROR,
		Some(Level::Warn) => LevelFilter::WARN,
		Some(Level::Info) => LevelFilter::INFO,
		Some(Level::Debug) => LevelFilter::DEBUG,
		_ => LevelFilter::TRACE
	};

	// Whether to use ANSI terminal escape codes
	let ansi = io::stdout().is_terminal()
		&& io::stderr().is_terminal()
		&& !no_color
		&& env::var("NO_COLOR").is_err()
		&& !matches!(env::var("TERM"), Ok(v) if v == "dumb");

	// Log to stderr
	let subscriber = tracing_subscriber::fmt()
		.compact()
		.with_max_level(filter)
		.with_target(false)
		.with_ansi(ansi)
		.with_writer(io::stderr)
		.finish();

	set_global_default(subscriber)
}
