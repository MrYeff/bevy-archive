use std::{error::Error, time::Duration};

use askama::Template;
use axum::{
    extract::Path,
    response::{Html, IntoResponse},
};
use derive_more::Display;

pub(super) async fn handler(Path(temp_id): Path<u32>) -> impl IntoResponse {
    // TODO load preview -> return preview after load is done

    let template = PreviewTemplate {
        load_delay: Duration::from_millis(500),
        temp_id,
        state: PreviewLoadState::Loaded(Ok(ArticleMeta {
            name: "Example Article".into(),
            tags: vec!["tag1".into(), "tag2".into(), "tag3".into()].into_boxed_slice(),
        })),
    };
    Html(template.render().unwrap())
}

#[derive(Template)]
#[template(path = "article/new/preview.html")]
struct PreviewTemplate {
    load_delay: std::time::Duration,
    temp_id: u32,
    state: PreviewLoadState,
}

#[derive(Display)]
enum LoadingError {
    MissingManifest,
    AlreadyImported,
}

enum PreviewLoadState {
    Loading,
    Loaded(Result<ArticleMeta, LoadingError>),
}

struct ArticleMeta {
    name: Box<str>,
    tags: Box<[Box<str>]>,
}
