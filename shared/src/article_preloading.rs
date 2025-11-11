use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::Url;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TempArticleId(pub Url);

impl From<Url> for TempArticleId {
    fn from(value: Url) -> Self {
        Self(value)
    }
}

impl Display for TempArticleId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.0)
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ArticleMeta {
    pub title: Box<str>,
    pub tags: Box<[Box<str>]>,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum PreloadArticleResult {
    Success(ArticleMeta),
    Failure(String),
}
