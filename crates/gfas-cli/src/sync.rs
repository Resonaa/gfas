use std::collections::HashSet;

use gfas_api::GitHub;
use tokio_util::task::TaskTracker;
use tracing::{error, info};

async fn run(token: String, endpoint: String, dry_run: bool) -> anyhow::Result<()> {
	let mut github = GitHub::new(token)?;
	github.with_host_override(endpoint);

	let user = github.users().get_authenticated_public_user().await?.body.login;

	info!("current user: {user}");

	let (following, followers) =
		tokio::try_join!(github.list_followings(&user), github.list_followers(&user))?;

	if dry_run {
		let to_unfollow = following.difference(&followers).cloned().collect::<Vec<_>>();
		let to_follow = followers.difference(&following).cloned().collect::<Vec<_>>();

		info!("=== dry run starting ===");
		info!("{} user(s) to unfollow: {}", to_unfollow.len(), to_unfollow.join(", "));
		info!("{} user(s) to follow: {}", to_follow.len(), to_follow.join(", "));
		info!("=== dry run completed ===");

		return Ok(());
	}

	let tracker = TaskTracker::new();

	following.difference(&followers).cloned().for_each(|user| {
		let users = github.users();
		tracker.spawn(async move {
			match users.unfollow(&user).await {
				Ok(_) => info!("unfollowed {user}"),
				Err(e) => error!("error unfollowing {user}: {e}")
			}
		});
	});

	followers.difference(&following).cloned().for_each(|user| {
		let users = github.users();
		tracker.spawn(async move {
			match users.follow(&user).await {
				Ok(_) => info!("followed {user}"),
				Err(e) => error!("error following {user}: {e}")
			}
		});
	});

	tracker.close();
	tracker.wait().await;

	// check if we failed to follow some users, probably because they've blocked us

	let tracker = TaskTracker::new();

	let new_following = github.list_followings(&user).await?;

	followers
		.difference(&following)
		.cloned()
		.collect::<HashSet<_>>()
		.difference(&new_following)
		.cloned()
		.for_each(|user| {
			let users = github.users();
			tracker.spawn(async move {
				// block and unblock to remove their follow
				match users.block(&user).await {
					Ok(_) => info!("blocked {user}"),
					Err(e) => error!("error blocking {user}: {e}")
				}
				match users.unblock(&user).await {
					Ok(_) => info!("unblocked {user}"),
					Err(e) => error!("error unblocking {user}: {e}")
				}
			});
		});

	tracker.close();
	tracker.wait().await;

	Ok(())
}

/// Synchronizes followings.
pub async fn sync(token: String, endpoint: String, dry_run: bool) -> anyhow::Result<()> {
	tokio::select! {
			res = tokio::signal::ctrl_c() => Ok(res?),
			res = run(token, endpoint, dry_run) => res
	}
}
