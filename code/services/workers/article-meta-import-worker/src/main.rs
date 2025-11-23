mod process;

use std::sync::Arc;

use clap::Parser;
use futures::FutureExt;
use redis::job_worker::JobWorker;
use shared::{
    jobs::article_preloading::{ImportError, TempArticleId},
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
    let args = Args::parse();

    let worker = JobWorker::new(
        &args.redis_preload_worker,
        args.worker,
        |jid: TempArticleId, _| {
            process_job(jid).map(|r| r.map_err(|e| ImportError(e.to_string().into())))
        },
    )
    .await?;

    worker.run(Arc::new(())).await?;
    Ok(())
}
