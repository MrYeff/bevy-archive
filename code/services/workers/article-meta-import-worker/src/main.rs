use article_meta_import_worker::{GitHubClient, process_job};
use clap::Parser;
use redis::job_worker::JobWorker;
use shared::{
    jobs::article_preloading::{ImportError, TempArticleId},
    prelude::*,
};
use std::sync::Arc;
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
        async move |jid: TempArticleId, ctx: Arc<Ctx>| {
            process_job(jid, &ctx.client)
                .await
                .map_err(|e| ImportError(e.to_string().into()))
        },
    )
    .await?;

    worker.run(Arc::new(Ctx::default())).await?;
    Ok(())
}

#[derive(Default)]
struct Ctx {
    client: GitHubClient,
}
