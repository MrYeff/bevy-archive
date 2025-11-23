use serde::Deserialize;
use url::Url;

pub struct GitHubClient {
    client: reqwest::Client,
    base_url: Url,
}

impl GitHubClient {
    pub async fn get_default_branch(&self, user: &str, repo: &str) -> anyhow::Result<Box<str>> {
        let info: GitHubRepoInfo = self
            .client
            .get(self.base_url.join(&format!("repos/{user}/{repo}"))?)
            .send()
            .await?
            .json()
            .await?;
        Ok(info.default_branch)
    }

    pub async fn get_latest_commit(
        &self,
        user: &str,
        repo: &str,
        branch: &str,
    ) -> anyhow::Result<Box<str>> {
        let info: GitHubBranchInfo = self
            .client
            .get(
                self.base_url
                    .join(&format!("repos/{user}/{repo}/branches/{branch}"))?,
            )
            .send()
            .await?
            .json()
            .await?;
        Ok(info.commit.sha)
    }

    pub(crate) async fn fetch_archive_toml(
        &self,
        user: &str,
        repo: &str,
        commit: &str,
    ) -> anyhow::Result<Box<str>> {
        let content: Box<str> = self
            .client
            .get(self.base_url.join(&format!(
                "repos/{user}/{repo}/contents/archive.toml?ref={commit}"
            ))?)
            .send()
            .await?
            .text()
            .await?
            .into();
        Ok(content)
    }

    pub fn new() -> Self {
        Self::from(reqwest::Client::new())
    }

    #[cfg(test)]
    pub fn with_base_url(client: reqwest::Client, base_url: Url) -> Self {
        GitHubClient { client, base_url }
    }
}

impl From<reqwest::Client> for GitHubClient {
    fn from(client: reqwest::Client) -> Self {
        GitHubClient {
            client,
            base_url: Url::parse("https://api.github.com/").unwrap(),
        }
    }
}

impl Default for GitHubClient {
    fn default() -> Self {
        Self::from(reqwest::Client::new())
    }
}

#[derive(Deserialize)]
pub(crate) struct GitHubRepoInfo {
    pub default_branch: Box<str>,
}

#[derive(Deserialize)]
pub(crate) struct GitHubBranchInfo {
    pub commit: GitHubCommitInfo,
}

#[derive(Deserialize)]
pub(crate) struct GitHubCommitInfo {
    pub sha: Box<str>,
}
