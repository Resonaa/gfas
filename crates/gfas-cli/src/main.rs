use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Level, Verbosity};
use futures::future;
use gfas_api::GitHub;
use tracing::level_filters::LevelFilter;

/// CLI arguments
#[derive(Parser, Debug)]
#[command(about, version)]
struct Args {
    /// Current user
    #[arg(short, long)]
    user: String,

    /// Access token
    #[arg(short, long)]
    token: String,

    #[command(flatten)]
    verbose: Verbosity<InfoLevel>
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Args { token, user, verbose } = Args::parse();

    let filter = match verbose.log_level() {
        None => LevelFilter::OFF,
        Some(Level::Error) => LevelFilter::ERROR,
        Some(Level::Warn) => LevelFilter::WARN,
        Some(Level::Info) => LevelFilter::INFO,
        Some(Level::Debug) => LevelFilter::DEBUG,
        _ => LevelFilter::TRACE
    };

    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_max_level(filter)
        .without_time()
        .with_target(false)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    let github = GitHub::with_token(&token)?;

    let (following, followers) =
        tokio::try_join!(github.explore(&user, "following"), github.explore(&user, "followers"))?;

    tokio::join!(
        future::join_all(following.difference(&followers).map(|user| github.unfollow(user))),
        future::join_all(followers.difference(&following).map(|user| github.follow(user)))
    );

    Ok(())
}
