//! This crate exports some GitHub API bindings through [`GitHub`].

use std::collections::HashSet;
use std::ops::{Deref, DerefMut};

use futures::TryFutureExt;
use reqwest::{header, Client, ClientBuilder, Response, Result};
use tracing::{debug, info, instrument, warn, Level};
use url::Url;

/// Extension for [`reqwest::ClientBuilder`].
pub trait BuilderExt {
    /// Sets the GitHub token for every request.
    fn token(self, token: &str) -> Self;

    /// Builds the GitHub client with API endpoint url.
    fn endpoint(self, endpoint: Url) -> Result<GitHub>;
}

impl BuilderExt for ClientBuilder {
    fn token(self, token: &str) -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert("User-Agent", header::HeaderValue::from_static("gfas"));
        headers.insert("Authorization", format!("token {token}").parse().unwrap());
        self.default_headers(headers)
    }

    fn endpoint(self, endpoint: Url) -> Result<GitHub> {
        Ok(GitHub { client: self.build()?, endpoint })
    }
}

/// Asynchronous GitHub API bindings that wraps a [`reqwest::Client`] internally.
///
/// # Examples
///
/// ```rust
/// use gfas_api::{BuilderExt, GitHub};
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let github = GitHub::builder().token("<token>").endpoint("https://api.github.com".parse()?)?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct GitHub {
    client: Client,
    endpoint: Url
}

impl Deref for GitHub {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

impl DerefMut for GitHub {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.client
    }
}

impl GitHub {
    /// Alias for [`reqwest::ClientBuilder::new()`].
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    /// Paginates through the given user profile link and returns
    /// discovered users collected in [`HashSet`].
    ///
    /// `role` should be either `"following"` or `"followers"`.
    ///
    /// # Errors
    ///
    /// Fails if an error occurs during sending requests.
    #[instrument(skip(self), ret(level = Level::TRACE), err)]
    pub async fn explore(&self, user: &str, role: &str) -> Result<HashSet<String>> {
        let mut res = HashSet::new();

        let url = self.endpoint.join(&format!("users/{user}/{role}")).unwrap();

        const PER_PAGE: usize = 100;

        for page in 1.. {
            debug!("page {page}");

            let users: Vec<_> = self
                .get(url.clone())
                .query(&[("page", page), ("per_page", PER_PAGE)])
                .send()
                .and_then(|r| r.json::<Vec<octokit_rs::webhook::User>>())
                .await?
                .into_iter()
                .map(|u| u.login)
                .collect();

            let len = users.len();

            res.extend(users);

            info!("{}(+{len})", res.len());

            if len < PER_PAGE {
                break;
            }
        }

        Ok(res)
    }

    /// Follows a user.
    ///
    /// # Errors
    ///
    /// Fails if an error occurs during sending the request.
    #[instrument(skip(self), ret(level = Level::TRACE), err)]
    pub async fn follow(&self, user: &str) -> Result<Response> {
        warn!("");

        let url = self.endpoint.join(&format!("/user/following/{user}")).unwrap();
        self.put(url).send().await
    }

    /// Unfollows a user.
    ///
    /// # Errors
    ///
    /// Fails if an error occurs during sending the request.
    #[instrument(skip(self), ret(level = Level::TRACE), err)]
    pub async fn unfollow(&self, user: &str) -> Result<Response> {
        warn!("");

        let url = self.endpoint.join(&format!("/user/following/{user}")).unwrap();
        self.delete(url).send().await
    }
}
