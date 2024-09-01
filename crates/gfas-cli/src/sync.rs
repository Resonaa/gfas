use clap::Args;
use gfas_api::GitHub;
use tokio_util::task::TaskTracker;

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
    #[arg(long, value_name = "URL", default_value = "https://api.github.com")]
    endpoint: String
}

async fn run(SyncFlags { user, token, endpoint }: SyncFlags) -> anyhow::Result<()> {
    let mut github = GitHub::new(token)?;
    github.with_host_override(endpoint);

    let (following, followers) =
        tokio::try_join!(github.explore(&user, true), github.explore(&user, false))?;

    let tracker = TaskTracker::new();

    following.difference(&followers).cloned().for_each(|user| {
        let users = github.users();
        tracker.spawn(async move { users.unfollow(&user).await });
    });

    followers.difference(&following).cloned().for_each(|user| {
        let users = github.users();
        tracker.spawn(async move { users.follow(&user).await });
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
