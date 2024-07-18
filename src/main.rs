use anyhow::Result;
use clap::Parser;
use futures_util::TryFutureExt;
use reqwest::{header, Client};
use serde::Deserialize;
use std::collections::HashSet;
use tracing::{info, warn, Level};

#[derive(Parser, Debug)]
#[command(about)]
struct Args {
    /// Current user
    #[arg(short, long)]
    user: String,

    /// Access token
    #[arg(short, long)]
    token: String,

    /// Dry run
    #[arg(short, long, default_value_t)]
    dry: bool,
}

#[derive(Deserialize)]
struct User {
    login: String,
}

async fn explore(client: Client, user: String, role: String) -> Result<HashSet<String>> {
    let mut res = HashSet::new();

    const PER_PAGE: usize = 100;

    let url = format!("https://api.github.com/users/{user}/{role}");

    for page in 1.. {
        let users = client
            .get(&url)
            .query(&[("page", page), ("per_page", PER_PAGE)])
            .send()
            .and_then(|r| r.json::<Vec<User>>())
            .await?
            .into_iter()
            .map(|u| u.login)
            .collect::<Vec<_>>();

        let last = users.len() < PER_PAGE;

        res.extend(users);

        if last {
            break;
        }
    }

    info!("explored {} {role}", res.len());

    Ok(res)
}

async fn follow(client: Client, user: String, dry: bool) -> Result<()> {
    if dry {
        info!("following {user}");
    } else {
        client
            .put(format!("https://api.github.com/user/following/{user}"))
            .send()
            .await?;

        warn!("followed {user}");
    }

    Ok(())
}

async fn unfollow(client: Client, user: String, dry: bool) -> Result<()> {
    if dry {
        info!("unfollowing {user}");
    } else {
        client
            .delete(format!("https://api.github.com/user/following/{user}"))
            .send()
            .await?;

        warn!("unfollowed {user}");
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_max_level(Level::INFO)
        .without_time()
        .with_target(false)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    let Args { token, user, dry } = Args::parse();

    let mut headers = header::HeaderMap::new();
    headers.insert("User-Agent", "gfas".parse()?);
    headers.insert("Authorization", format!("token {token}").parse()?);

    let client = Client::builder().default_headers(headers).build()?;

    let [task_1, task_2] = ["following", "followers"].map(|role| {
        let client = client.clone();
        let user = user.clone();
        let role = role.to_owned();

        tokio::spawn(async move { explore(client, user, role).await })
    });
    let following = task_1.await??;
    let followers = task_2.await??;

    let unfollow_tasks = following.difference(&followers).map(|user| {
        let client = client.clone();
        let user = user.clone();

        tokio::spawn(async move { unfollow(client, user, dry).await })
    });

    let follow_tasks = followers.difference(&following).map(|user| {
        let client = client.clone();
        let user = user.clone();

        tokio::spawn(async move { follow(client, user, dry).await })
    });

    for task in unfollow_tasks.chain(follow_tasks) {
        task.await??;
    }

    Ok(())
}
