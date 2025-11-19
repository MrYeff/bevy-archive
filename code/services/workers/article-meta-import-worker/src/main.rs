mod process;
use clap::Parser;
use futures::future;
use redis::job_worker::JobWorkerRx;
use shared::{
    jobs::article_preloading::{ImportError, PreloadArticleResult, TempArticleId},
    prelude::*,
};

use crate::process::process_job;

#[derive(Debug, Parser, Clone)]
struct Args {
    #[command(flatten)]
    worker: WorkerArgs,
    #[command(flatten)]
    redis_preload_worker: RedisAccessArgs<{ RedisDesignation::PreloadArticleWorkerCache as u32 }>,
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    let args = Args::try_parse()?;

    // Start the worker
    let ctx = WorkerCtx {
        redis: JobWorkerRx::new(&args.redis_preload_worker).await?,
    };
    let workers: Box<[_]> = (0..args.worker.workers)
        .map(|_| {
            let ctx = ctx.clone();
            tokio::spawn(async move {
                run_worker(ctx).await;
            })
        })
        .collect();

    future::join_all(workers).await;
    Ok(())
}

async fn run_worker(ctx: WorkerCtx) {
    loop {
        let Ok(job) = ctx.redis.fetch_next_job().await else {
            panic!("Failed to fetch job from Redis"); // TODO: handle errors properly
        };

        let result = process_job(&job).await;
        ctx.redis
            .store_result(job, result.map_err(|e| ImportError(e.to_string().into())))
            .await
            .expect("Failed to store job result"); // TODO: handle errors properly
    }
}

#[derive(Clone)]
struct WorkerCtx {
    redis: JobWorkerRx<TempArticleId, PreloadArticleResult>,
}
