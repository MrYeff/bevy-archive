use std::time::Duration;

use askama::Template;
use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse},
};

pub(super) async fn handler(Path(id): Path<u32>) -> impl IntoResponse {
    let template = LoadContentTemplate {
        id,
        load_delay: Duration::from_millis(500),
    };
    (StatusCode::ACCEPTED, Html(template.render().unwrap()))
}

#[derive(Template)]
#[template(path = "article/load_content.html")]
struct LoadContentTemplate {
    id: u32,
    load_delay: Duration,
}
