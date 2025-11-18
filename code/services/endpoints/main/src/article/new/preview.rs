use askama::Template;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{Html, IntoResponse},
};
use shared::prelude::*;
use serde::Deserialize;
use std::time::Duration;

use crate::Ctx;

pub(super) async fn handler(
    State(ctx): State<Ctx>,
    params: Query<PreviewParams>,
) -> impl IntoResponse {
    let url = match Url::try_from(params.url.clone()) {
        Ok(url) => url,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid URL").into_response(),
    };
    // TODO schedule load -> return temp id for loading preview
    let rsp = match ctx
        .preload_article_worker_tx
        .try_fetch_result(url.clone().into())
        .await
    {
        Ok(rsp) => rsp,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to preload article",
            )
                .into_response();
        }
    };

    let Some(result) = rsp else {
        if let Err(_) = ctx
            .preload_article_worker_tx
            .schedule_job(url.clone().into())
            .await
        {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to schedule article preload",
            )
                .into_response();
        };

        let template = LoadPreviewTemplate {
            load_delay: Duration::from_millis(0),
            url: url.into(),
        };
        return (
            StatusCode::ACCEPTED,
            Html(template.render().unwrap()).into_response(),
        )
            .into_response();
    };

    let template = PreviewTemplate {
        url: url.into(),
        result,
    };
    Html(template.render().unwrap()).into_response()
}

/// Displayed after preview is loaded
#[derive(Template)]
#[template(path = "article/new/preview.html")]
struct PreviewTemplate {
    url: Box<str>,
    result: PreloadArticleResult,
}

/// Displayed while loading preview
#[derive(Template)]
#[template(path = "article/new/load_preview.html")]
struct LoadPreviewTemplate {
    load_delay: Duration,
    url: Box<str>,
}

#[derive(Deserialize)]
pub struct PreviewParams {
    url: Box<str>,
}
