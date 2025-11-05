use std::time::Duration;

use askama::Template;
use axum::{
    extract::Query,
    response::{Html, IntoResponse},
};
use serde::Deserialize;

pub(super) async fn handler(params: Query<LoadMetaParams>) -> impl IntoResponse {
    // TODO schedule load -> return temp id for loading preview

    let template = LoadPreviewTemplate {
        load_delay: Duration::from_millis(0),
        temp_id: 0,
    };
    Html(template.render().unwrap())
}

#[derive(Deserialize)]
pub(super) struct LoadMetaParams {
    url: Box<str>,
}

#[derive(Template)]
#[template(path = "article/new/load_preview.html")]
struct LoadPreviewTemplate {
    load_delay: std::time::Duration,
    temp_id: u32,
}
