use serde::Deserialize;
use shared::jobs::article_preloading::{ArticleMeta, ImportError, TempArticleId};
use url::Host;

pub(super) async fn process_job(job: TempArticleId) -> anyhow::Result<ArticleMeta> {
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
                get_latest_commit(user, repo, &b).await?
            }
        }
        (None, None) => {
            let default_branch = get_default_branch(user, repo).await?;
            get_latest_commit(user, repo, &default_branch).await?
        }
        _ => todo!(),
    };

    let archive_toml = fetch_archive_toml(user, repo, &commit).await?;
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

async fn get_default_branch(user: &str, repo: &str) -> anyhow::Result<Box<str>> {
    let info: GitHubRepoInfo = reqwest::get(format!("https://api.github.com/repos/{user}/{repo}"))
        .await?
        .json()
        .await?;
    Ok(info.default_branch)
}

#[derive(Deserialize)]
struct GitHubRepoInfo {
    default_branch: Box<str>,
}

async fn get_latest_commit(user: &str, repo: &str, branch: &str) -> anyhow::Result<Box<str>> {
    let info: GitHubBranchInfo = reqwest::get(format!(
        "https://api.github.com/repos/{user}/{repo}/branches/{branch}"
    ))
    .await?
    .json()
    .await?;
    Ok(info.commit.sha)
}

#[derive(Deserialize)]
struct GitHubBranchInfo {
    commit: GitHubCommitInfo,
}

#[derive(Deserialize)]
struct GitHubCommitInfo {
    sha: Box<str>,
}

async fn fetch_archive_toml(user: &str, repo: &str, commit: &str) -> anyhow::Result<Box<str>> {
    let url = format!("https://raw.githubusercontent.com/{user}/{repo}/{commit}/archive.toml");
    let rsp = reqwest::get(url).await?;
    let content = rsp.text().await?;
    Ok(content.into())
}

#[derive(Deserialize, Clone)]
struct ArchiveMeta {
    title: Box<str>,
    tags: Box<[Box<str>]>,
    path_in_repo: Box<str>,
}
