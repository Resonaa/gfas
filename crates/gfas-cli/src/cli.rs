use clap::{Parser, Subcommand, ValueEnum};
use clap_verbosity_flag::{InfoLevel, Verbosity};

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
        /// Current user
        #[arg(short, long)]
        user: String,

        /// Access token
        #[arg(short, long)]
        token: String
    },

    /// Generate man page or shell completions
    Generate {
        /// The thing to generate
        mode: GenerateMode
    }
}

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
