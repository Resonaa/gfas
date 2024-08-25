use gfas_api::GitHub;
use tokio_util::task::TaskTracker;

async fn run(user: &str, token: &str) -> anyhow::Result<()> {
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

/// Synchronizes followings.
pub async fn sync(user: &str, token: &str) -> anyhow::Result<()> {
    tokio::select! {
        res = tokio::signal::ctrl_c() => Ok(res?),
        res = run(user, token) => res
    }
}
