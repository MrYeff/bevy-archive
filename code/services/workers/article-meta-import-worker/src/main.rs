use std::convert::Infallible;

use article_meta_import_worker::{GitHubClient, process_job};
use clap::Parser;
use futures::{StreamExt, never::Never, stream::FuturesUnordered};
use redis::job_worker::JobWorkerRx;
use shared::{
    jobs::article_preloading::{ImportError, PreloadArticleResult, TempArticleId},
    prelude::*,
};
#[derive(Debug, Parser, Clone)]
struct Args {
    #[command(flatten)]
    worker: WorkerArgs,
    #[command(flatten)]
    redis_preload_worker: RedisAccessArgs<{ RedisDesignation::PreloadArticleWorkerCache as u32 }>,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<Never> {
    let args = Args::parse();

    let rx = JobWorkerRx::new(args.redis_preload_worker).await?;
    let client = GitHubClient::new();

    let mut tasks = FuturesUnordered::new();

    for _ in 0..args.worker.workers {
        tasks.push(worker_loop(&rx, &client));
    }

    tasks.next().await.unwrap()?;
    unreachable!("tasks only terminate on error");
}

async fn worker_loop(
    rx: &JobWorkerRx<TempArticleId, PreloadArticleResult>,
    client: &GitHubClient,
) -> anyhow::Result<()> {
    loop {
        let jid = rx.fetch_next_job().await?;
        let result = process_job(&jid, &client)
            .await
            .map_err(|e| ImportError(e.to_string().into()));
        rx.store_result(jid, result).await?;
    }
}
