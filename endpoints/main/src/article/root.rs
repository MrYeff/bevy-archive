use std::time::Duration;

use askama::Template;
use axum::extract::Path;
use axum::response::{Html, IntoResponse};

pub(super) async fn handler(Path(id): Path<u32>) -> impl IntoResponse {
    let template = ArticleTemplate {
        title: "Sample Article",
        tags: vec!["rust", "web", "axum"].into_boxed_slice(),
        id,
        load_delay: Duration::ZERO,
    };
    Html(template.render().unwrap())
}

#[derive(Template)]
#[template(path = "article/root.html")]
struct ArticleTemplate<'a> {
    title: &'a str,
    tags: Box<[&'a str]>,
    id: u32,
    load_delay: Duration,
}
