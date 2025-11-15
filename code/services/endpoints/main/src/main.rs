mod article;
mod root;

use axum::{Router, routing::*};
use clap::Parser;
use config::RedisDesignation;
use core::{
    article::{ArticleId, rendering::RenderArticleResult},
    article_preloading::{PreloadArticleResult, TempArticleId},
    common_args::EndpointArgs,
    setup::load_config,
};
use redis::job_worker::JobWorkerTx;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = EndpointArgs::try_parse()?;
    let config = load_config(args.common.cfg)?;

    let app = Router::new()
        .route("/", get(root::handler))
        .nest("/article", article::route())
        .with_state(Ctx {
            render_worker_tx: JobWorkerTx::new(
                config
                    .redis
                    .get_access(&RedisDesignation::RenderWorkerChache)?,
            )
            .await?,
            preload_article_worker_tx: JobWorkerTx::new(
                config
                    .redis
                    .get_access(&RedisDesignation::PreloadArticleWorkerChache)?,
            )
            .await?,
        });

    let listener = TcpListener::bind(format!("127.0.0.1:{}", args.port))
        .await
        .unwrap();
    println!("Listening on http://127.0.0.1:{}", args.port);
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

#[derive(Clone)]
struct Ctx {
    render_worker_tx: JobWorkerTx<ArticleId, RenderArticleResult>,
    preload_article_worker_tx: JobWorkerTx<TempArticleId, PreloadArticleResult>,
}
