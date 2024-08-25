use std::env;
use std::io::{stderr, stdout, IsTerminal};

use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Level, Verbosity};
use gfas_api::GitHub;
use tokio_util::task::TaskTracker;
use tracing::level_filters::LevelFilter;

/// CLI flags
#[derive(Parser, Debug)]
#[command(about, version)]
struct Flags {
    /// Current user
    #[arg(short, long)]
    user: String,

    /// Access token
    #[arg(short, long)]
    token: String,

    /// Disable color printing
    #[arg(long, default_value_t = false)]
    no_color: bool,

    #[command(flatten)]
    verbose: Verbosity<InfoLevel>
}

async fn run(user: &str, token: &str) -> Result<(), Box<dyn std::error::Error>> {
    let github = GitHub::with_token(token)?;

    let (following, followers) =
        tokio::try_join!(github.explore(user, "following"), github.explore(user, "followers"))?;

    let tracker = TaskTracker::new();

    following.difference(&followers).cloned().for_each(|user| {
        let github = github.clone();
        tracker.spawn(async move { github.unfollow(&user).await });
    });

    followers.difference(&following).cloned().for_each(|user| {
        let github = github.clone();
        tracker.spawn(async move { github.follow(&user).await });
    });

    tracker.close();
    tracker.wait().await;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Flags { token, user, verbose, no_color } = Flags::parse();

    {
        let filter = match verbose.log_level() {
            None => LevelFilter::OFF,
            Some(Level::Error) => LevelFilter::ERROR,
            Some(Level::Warn) => LevelFilter::WARN,
            Some(Level::Info) => LevelFilter::INFO,
            Some(Level::Debug) => LevelFilter::DEBUG,
            _ => LevelFilter::TRACE
        };

        // Whether to use ANSI terminal escape codes
        let ansi = stdout().is_terminal()
            && stderr().is_terminal()
            && !no_color
            && env::var("NO_COLOR").is_err()
            && !matches!(env::var("TERM"), Ok(v) if v == "dumb");

        // Log to stderr
        let subscriber = tracing_subscriber::fmt()
            .compact()
            .with_max_level(filter)
            .with_target(false)
            .with_ansi(ansi)
            .with_writer(stderr)
            .finish();

        tracing::subscriber::set_global_default(subscriber)?;
    }

    tokio::select! {
        res = tokio::signal::ctrl_c() => Ok(res?),
        res = run(&user, &token) => res
    }
}
