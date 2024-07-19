use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Level, Verbosity};
use gfas::{GitHub, Result};
use tracing::level_filters::LevelFilter;

/// CLI arguments
#[derive(Parser, Debug)]
#[command(about)]
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
async fn main() -> Result<()> {
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

    let [task_1, task_2] = ["following", "followers"].map(|role| {
        let github = github.clone();
        let user = user.clone();
        let role = role.to_owned();

        tokio::spawn(async move { github.explore(&user, &role).await })
    });
    let following = task_1.await??;
    let followers = task_2.await??;

    let unfollow_tasks = following.difference(&followers).map(|user| {
        let github = github.clone();
        let user = user.clone();

        tokio::spawn(async move { github.unfollow(&user).await })
    });

    let follow_tasks = followers.difference(&following).map(|user| {
        let github = github.clone();
        let user = user.clone();

        tokio::spawn(async move { github.follow(&user).await })
    });

    for task in unfollow_tasks.chain(follow_tasks) {
        task.await??;
    }

    Ok(())
}
