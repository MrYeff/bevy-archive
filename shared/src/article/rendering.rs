use serde::{Deserialize, Serialize};

use crate::HtmlPage;

#[derive(Serialize, Deserialize, Clone)]
pub enum RenderArticleResult {
    Success(HtmlPage),
    Failure(String),
}
