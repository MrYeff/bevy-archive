use askama::Template;
use axum::response::{Html, IntoResponse};

pub(super) async fn handler() -> impl IntoResponse {
    let template = MainTemplate;
    Html(template.render().unwrap())
}

#[derive(Template)]
#[template(path = "main.html")]
struct MainTemplate;
