use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Level, Verbosity};
use gfas::GitHub;
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
    verbose: Verbosity<InfoLevel>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Args {
        token,
        user,
        verbose,
    } = Args::parse();

    let filter = match verbose.log_level() {
        None => LevelFilter::OFF,
        Some(Level::Error) => LevelFilter::ERROR,
        Some(Level::Warn) => LevelFilter::WARN,
        Some(Level::Info) => LevelFilter::INFO,
        Some(Level::Debug) => LevelFilter::DEBUG,
        _ => LevelFilter::TRACE,
    };

    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_max_level(filter)
        .without_time()
        .with_target(false)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    let github = GitHub::with_token(&token)?;

    let (following, followers) = tokio::try_join!(
        github.explore(&user, "following"),
        github.explore(&user, "followers")
    )?;

    let unfollow_tasks = following.difference(&followers).cloned().map(|user| {
        let github = github.clone();
        tokio::spawn(async move { github.unfollow(&user).await })
    });

    let follow_tasks = followers.difference(&following).cloned().map(|user| {
        let github = github.clone();
        tokio::spawn(async move { github.follow(&user).await })
    });

    for task in unfollow_tasks.chain(follow_tasks) {
        task.await??;
    }

    Ok(())
}
