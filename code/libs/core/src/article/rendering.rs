use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone)]
pub enum RenderArticleResult {
    Success(HtmlPage),
    Failure(String),
}
