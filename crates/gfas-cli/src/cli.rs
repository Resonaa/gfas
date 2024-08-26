use clap::{Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};

use crate::generate::GenerateMode;
use crate::sync::SyncFlags;

/// CLI arguments
#[derive(Parser, Debug)]
#[command(about, version)]
pub struct Cli {
    /// Disable color printing
    #[arg(long, default_value_t = false)]
    pub no_color: bool,

    #[command(flatten)]
    pub verbose: Verbosity<InfoLevel>,

    #[command(subcommand)]
    pub command: Commands
}

/// CLI subcommands
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Sync followings
    Sync {
        #[command(flatten)]
        flags: SyncFlags
    },

    /// Generate man page or shell completions
    Generate {
        /// The thing to generate
        mode: GenerateMode
    }
}
