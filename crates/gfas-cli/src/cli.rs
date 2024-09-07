use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};

/// CLI arguments.
#[derive(Parser, Debug)]
#[command(about, version)]
pub struct Cli {
    /// Access token
    #[arg(env = "GITHUB_TOKEN")]
    pub token: String,

    /// GitHub API endpoint
    #[arg(long, value_name = "URL", default_value = "https://api.github.com")]
    pub endpoint: String,

    /// Disable color printing
    #[arg(long, default_value_t = false)]
    pub no_color: bool,

    #[command(flatten)]
    pub verbose: Verbosity<InfoLevel>
}
