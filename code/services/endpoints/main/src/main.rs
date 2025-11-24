mod article;
mod root;

use axum::{Router, routing::*};
use clap::Parser;
use redis::job_worker::JobWorkerTx;
use shared::{
    jobs::{
        article_preloading::{PreloadArticleResult, TempArticleId},
        article_rendering::{ArticleId, RenderArticleResult},
    },
    prelude::*,
};
use tokio::net::TcpListener;

#[derive(Debug, Parser, Clone)]
struct Args {
    #[command(flatten)]
    endpoint: EndpointArgs,
    #[command(flatten)]
    pg_access: PgAccesArgs,
    #[command(flatten)]
    redis_render_worker: RedisAccessArgs<{ RedisDesignation::RenderWorkerCache as u32 }>,
    #[command(flatten)]
    redis_preload_worker: RedisAccessArgs<{ RedisDesignation::PreloadArticleWorkerCache as u32 }>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let app = Router::new()
        .route("/", get(root::handler))
        .nest("/article", article::route())
        .with_state(Ctx {
            render_worker_tx: JobWorkerTx::new(args.redis_render_worker).await?,
            preload_article_worker_tx: JobWorkerTx::new(args.redis_preload_worker).await?,
        });

    let listener = TcpListener::bind(format!("127.0.0.1:{}", args.endpoint.port))
        .await
        .unwrap();
    println!("Listening on http://127.0.0.1:{}", args.endpoint.port);
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

#[derive(Clone)]
struct Ctx {
    render_worker_tx: JobWorkerTx<ArticleId, RenderArticleResult>,
    preload_article_worker_tx: JobWorkerTx<TempArticleId, PreloadArticleResult>,
}
