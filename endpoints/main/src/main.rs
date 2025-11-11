mod article;
mod root;

use std::{sync::Arc, time::Duration};

use axum::{Router, routing::*};
use constants::{PRELOAD_ARTICLE_WORKER_KEY, RENDER_WORKER_KEY};
use fred::prelude::*;
use redis::{init_redis_client, job_worker::JobWorkerTx};
use shared::{
    SystemConfig,
    article::{ArticleId, rendering::RenderArticleResult},
    article_preloading::{PreloadArticleResult, TempArticleId},
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = SystemConfig::load_me_instead();

    let app = Router::new()
        .route("/", get(root::handler))
        .nest("/article", article::route())
        .with_state(Ctx {
            render_worker_tx: JobWorkerTx::new(
                init_redis_client(config.render_worker_redis_url.as_ref()).await?,
                RENDER_WORKER_KEY,
            ),
            preload_article_worker_tx: JobWorkerTx::new(
                init_redis_client(config.preload_article_worker_redis_url.as_ref()).await?,
                PRELOAD_ARTICLE_WORKER_KEY,
            ),
        });

    let listener = TcpListener::bind(config.main_endpoint_url.as_ref())
        .await
        .unwrap();
    println!("Listening on http://{}", config.main_endpoint_url);
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

#[derive(Clone)]
struct Ctx {
    render_worker_tx: JobWorkerTx<ArticleId, RenderArticleResult>,
    preload_article_worker_tx: JobWorkerTx<TempArticleId, PreloadArticleResult>,
}
