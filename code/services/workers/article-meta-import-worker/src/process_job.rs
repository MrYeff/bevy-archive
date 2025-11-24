use serde::Deserialize;
use shared::jobs::article_preloading::{ArticleMeta, ImportError, TempArticleId};
use url::Host;

use crate::GitHubClient;

pub async fn process_job(
    job: &TempArticleId,
    client: &GitHubClient,
) -> anyhow::Result<ArticleMeta> {
    let url = &job.0;

    if url.host() != Some(Host::Domain("github.com")) {
        return Err(ImportError("Only github.com URLs are supported".into()).into());
    };

    let (user, repo, mut tail) = url
        .path_segments()
        .and_then(|mut segments| {
            let owner = segments.next()?;
            let repo = segments.next()?;
            Some((owner, repo, segments))
        })
        .ok_or_else(|| ImportError("Invalid GitHub repository URL".into()))?;

    let commit = match (tail.next(), tail.next()) {
        (Some("commit"), Some(c)) => c.to_string().into_boxed_str(),
        (Some("tree"), Some(b)) => {
            if check_is_commit_sha(b) {
                b.to_string().into_boxed_str()
            } else {
                client.get_latest_commit(user, repo, &b).await?
            }
        }
        (None, None) => {
            let default_branch = client.get_default_branch(user, repo).await?;
            client
                .get_latest_commit(user, repo, &default_branch)
                .await?
        }
        _ => todo!(),
    };

    let archive_toml = client.fetch_archive_toml(user, repo, &commit).await?;
    let archive_meta: ArchiveMeta = toml::from_str(&archive_toml)?;
    Ok(ArticleMeta {
        title: archive_meta.title,
        tags: archive_meta.tags,
        path_in_repo: archive_meta.path_in_repo,
        repo_url: url.clone(),
        commit,
    })
}

fn check_is_commit_sha(s: &str) -> bool {
    s.len() == 40 && s.chars().all(|c| c.is_ascii_hexdigit())
}

#[derive(Deserialize, Clone)]
pub(crate) struct ArchiveMeta {
    pub title: Box<str>,
    pub tags: Box<[Box<str>]>,
    pub path_in_repo: Box<str>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;
    use url::Url;

    #[test]
    fn test_check_is_commit_sha() {
        assert!(check_is_commit_sha(
            "a3c2565e5f4b6c7d8e9f0a1b2c3d4e5f6a7b8c9d"
        ));
        assert!(!check_is_commit_sha("not-a-sha"));
        assert!(!check_is_commit_sha("12345"));
    }

    const ARCHIVE_COMMIT_TOML: &str = include_str!("../test-data/archive_commit.toml");
    const ARCHIVE_DEFAULT_BRANCH: &str = include_str!("../test-data/archive_default_branch.toml");

    #[tokio::test]
    async fn test_process_job_commit_url() {
        let server = MockServer::start();

        let commit_sha = "a3c2565e5f4b6c7d8e9f0a1b2c3d4e5f6a7b8c9d";

        let _archive_mock = server.mock(|when, then| {
            when.method(GET)
                .path("/repos/some-user/some-repo/contents/archive.toml")
                .query_param("ref", "a3c2565e5f4b6c7d8e9f0a1b2c3d4e5f6a7b8c9d");
            then.status(200)
                .header("content-type", "text/plain")
                .body(ARCHIVE_COMMIT_TOML);
        });

        let base_url = Url::parse(&format!("{}/", server.base_url())).unwrap();
        let http_client = reqwest::Client::new();
        let github_client = GitHubClient::with_base_url(http_client, base_url); // no function or associated item named `with_base_url` found for struct `GitHubClient` in the current scope

        let job_url = Url::parse(&format!(
            "https://github.com/{}/{}/commit/{}",
            "some-user", "some-repo", commit_sha
        ))
        .unwrap();
        let job = TempArticleId(job_url);

        let meta = process_job(&job, &github_client).await.unwrap();

        assert_eq!(&*meta.title, "My test article");
        assert_eq!(meta.tags.as_ref(), &["rust".into(), "github".into()]);
        assert_eq!(&*meta.path_in_repo, "articles/test.md");
        assert_eq!(&*meta.commit, commit_sha);
    }

    #[tokio::test]
    async fn test_process_job_default_branch() {
        let server = MockServer::start();

        let _repo_mock = server.mock(|when, then| {
            when.method(GET).path("/repos/some-user/some-repo");
            then.status(200)
                .header("content-type", "application/json")
                .body(r#"{ "default_branch": "main" }"#);
        });

        let _branch_mock = server.mock(|when, then| {
            when.method(GET)
                .path("/repos/some-user/some-repo/branches/main");
            then.status(200)
                .header("content-type", "application/json")
                .body(r#"{ "commit": { "sha": "commit-for-main" } }"#);
        });

        let _archive_mock = server.mock(|when, then| {
            when.method(GET)
                .path("/repos/some-user/some-repo/contents/archive.toml")
                .query_param("ref", "commit-for-main");
            then.status(200)
                .header("content-type", "text/plain")
                .body(ARCHIVE_DEFAULT_BRANCH);
        });

        let base_url = Url::parse(&format!("{}/", server.base_url())).unwrap();
        let http_client = reqwest::Client::new();
        let github_client = GitHubClient::with_base_url(http_client, base_url);

        let job_url = Url::parse("https://github.com/some-user/some-repo").unwrap();
        let job = TempArticleId(job_url);

        let meta = process_job(&job, &github_client).await.unwrap();

        assert_eq!(&*meta.title, "Main branch article");
        assert_eq!(meta.tags.as_ref(), &["rust".into()]);
        assert_eq!(&*meta.path_in_repo, "articles/main.md");
        assert_eq!(&*meta.commit, "commit-for-main");
    }
}
