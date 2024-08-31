use clap::{Args, ValueHint};
use gfas_api::{BuilderExt, GitHub};
use tokio_util::task::TaskTracker;
use url::Url;

/// Flags used in the sync subcommand
#[derive(Args, Debug)]
pub struct SyncFlags {
    /// Current user
    #[arg(short, long)]
    user: String,

    /// Access token
    #[arg(short, long)]
    token: String,

    /// GitHub API endpoint
    #[arg(long, value_name = "URL", default_value = "https://api.github.com",
    value_hint = ValueHint::Url)]
    endpoint: Url
}

async fn run(SyncFlags { user, token, endpoint }: SyncFlags) -> anyhow::Result<()> {
    let github = GitHub::builder().token(&token).endpoint(endpoint)?;

    let (following, followers) =
        tokio::try_join!(github.explore(&user, "following"), github.explore(&user, "followers"))?;

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

/// Synchronizes followings.
pub async fn sync(flags: SyncFlags) -> anyhow::Result<()> {
    tokio::select! {
        res = tokio::signal::ctrl_c() => Ok(res?),
        res = run(flags) => res
    }
}
