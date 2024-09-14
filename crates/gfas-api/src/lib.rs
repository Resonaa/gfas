//! This crate exports some GitHub API bindings through [`GitHub`].

use std::collections::HashSet;
use std::ops::{Deref, DerefMut};

use octorust::auth::Credentials;
use octorust::Client;
use tracing::{info, instrument, Level};

type Result<T> = std::result::Result<T, octorust::ClientError>;

/// Asynchronous GitHub API bindings that wraps [`octorust::Client`] internally.
///
/// # Examples
///
/// ```rust
/// use gfas_api::GitHub;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let github = GitHub::new(String::from("<TOKEN>"))?;
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
#[repr(transparent)]
pub struct GitHub(Client);

impl Deref for GitHub {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for GitHub {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl GitHub {
    const USER_AGENT: &'static str = "gfas";

    /// Create a new GitHub API client.
    pub fn new(token: String) -> Result<Self> {
        Ok(Self(Client::new(Self::USER_AGENT, Credentials::Token(token))?))
    }

    /// Paginates through the given user profile link and returns
    /// discovered followings/followers collected in [`HashSet`].
    ///
    /// # Errors
    ///
    /// Fails if an error occurs during sending requests.
    #[instrument(skip(self), ret(level = Level::TRACE), err)]
    pub async fn explore(&self, user: &str, following: bool) -> Result<HashSet<String>> {
        let mut res = HashSet::new();

        const PER_PAGE: i64 = 100;

        let users = self.users();

        for page in 1.. {
            let response = if following {
                users.list_following_for_user(user, PER_PAGE, page).await
            } else {
                users.list_followers_for_user(user, PER_PAGE, page).await
            }?;

            let explored = response.body.into_iter().map(|u| u.login);

            let len = explored.len() as i64;

            res.extend(explored);

            info!("{}(+{len})", res.len());

            if len < PER_PAGE {
                break;
            }
        }

        Ok(res)
    }
}
