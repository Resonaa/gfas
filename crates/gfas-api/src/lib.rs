//! This crate exports some GitHub API bindings through [`GitHub`].

use std::collections::HashSet;

use derive_builder::Builder;
use futures::TryFutureExt;
use reqwest::{header, Client, Response, Result};
use serde::Deserialize;
use tracing::{debug, info, instrument, warn, Level};
use url::Url;

/// Asynchronous GitHub API bindings that wraps a [`reqwest::Client`] internally.
#[derive(Debug, Clone, Builder)]
pub struct GitHub {
    #[builder(
        setter(name = "token", into),
        field(
            ty = "String",
            build = r#"
                let mut headers = header::HeaderMap::new();
                headers.insert("User-Agent", header::HeaderValue::from_static("gfas"));
                headers.insert("Authorization", format!("token {}", self.client).parse().unwrap());
                Client::builder().default_headers(headers).build().unwrap()
            "#
        )
    )]
    client: Client,

    #[builder]
    endpoint: Url
}

impl GitHub {
    /// Alias for [`GitHubBuilder::create_empty()`].
    pub fn builder() -> GitHubBuilder {
        GitHubBuilder::create_empty()
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

        #[derive(Deserialize)]
        struct User {
            login: String
        }

        const PER_PAGE: usize = 100;

        for page in 1.. {
            debug!("page {page}");

            let users: Vec<_> = self
                .client
                .get(url.clone())
                .query(&[("page", page), ("per_page", PER_PAGE)])
                .send()
                .and_then(|r| r.json::<Vec<User>>())
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
        self.client.put(url).send().await
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
        self.client.delete(url).send().await
    }
}
