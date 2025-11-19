use askama::Template;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{Html, IntoResponse},
};
use serde::Deserialize;
use shared::jobs::article_preloading::PreloadArticleResult;
use std::time::Duration;
use url::Url;

use crate::Ctx;

pub(super) async fn handler(
    State(ctx): State<Ctx>,
    params: Query<PreviewParams>,
) -> impl IntoResponse {
    // TODO schedule load -> return temp id for loading preview
    let rsp = match ctx
        .preload_article_worker_tx
        .try_fetch_result(params.url.clone().into())
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
            .schedule_job(params.url.clone().into())
            .await
        {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to schedule article preload",
            )
                .into_response();
        };

        let template = LoadPreviewTemplate {
            load_delay: &Duration::from_millis(0),
            url: &params.url,
        };
        return (
            StatusCode::ACCEPTED,
            Html(template.render().unwrap()).into_response(),
        )
            .into_response();
    };

    let template = PreviewTemplate {
        url: &params.url,
        result: &result,
    };
    Html(template.render().unwrap()).into_response()
}

/// Displayed after preview is loaded
#[derive(Template)]
#[template(path = "article/new/preview.html")]
struct PreviewTemplate<'a> {
    result: &'a PreloadArticleResult,
    url: &'a Url,
}

/// Displayed while loading preview
#[derive(Template)]
#[template(path = "article/new/load_preview.html")]
struct LoadPreviewTemplate<'a> {
    load_delay: &'a Duration,
    url: &'a Url,
}

#[derive(Deserialize)]
pub struct PreviewParams {
    url: Url,
}
