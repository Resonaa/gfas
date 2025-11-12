use gfas_api::GitHub;
use tokio_util::task::TaskTracker;
use tracing::info;

async fn run(token: String, endpoint: String) -> anyhow::Result<()> {
	let mut github = GitHub::new(token)?;
	github.with_host_override(endpoint);

	let user = github.users().get_authenticated_public_user().await?.body.login;

	info!("current user: {user}");

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
pub async fn sync(token: String, endpoint: String) -> anyhow::Result<()> {
	tokio::select! {
			res = tokio::signal::ctrl_c() => Ok(res?),
			res = run(token, endpoint) => res
	}
}
