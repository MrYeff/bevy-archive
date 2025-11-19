use derive_more::Display;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use url::Url;

#[derive(Serialize, Deserialize, Display, Clone, Debug)]
pub struct TempArticleId(pub Url);

impl From<Url> for TempArticleId {
    fn from(value: Url) -> Self {
        Self(value)
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ArticleMeta {
    pub repo_url: Url,
    pub path_in_repo: Box<str>,
    pub commit: Box<str>,
    pub title: Box<str>,
    pub tags: Box<[Box<str>]>,
}

#[derive(Serialize, Deserialize, Error, Display, Debug, Clone)]
pub struct ImportError(pub Box<str>);

pub type PreloadArticleResult = Result<ArticleMeta, ImportError>;
