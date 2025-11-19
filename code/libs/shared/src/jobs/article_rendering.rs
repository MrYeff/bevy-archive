use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Display, Clone)]
pub struct ArticleId(u32);

impl From<u32> for ArticleId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

pub type RenderArticleResult = Result<Box<str>, String>;
