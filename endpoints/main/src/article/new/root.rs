use askama::Template;
use axum::response::{Html, IntoResponse};

pub(super) async fn handler() -> impl IntoResponse {
    Html(NewArticle.render().unwrap())
}

#[derive(Template)]
#[template(path = "article/new/root.html")]
struct NewArticle;
