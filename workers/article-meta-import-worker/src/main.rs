use constants::PRELOAD_ARTICLE_WORKER_KEY;
use futures::future;
use redis::job_worker::JobWorkerRx;
use shared::{
    SystemConfig,
    article_preloading::{ArticleMeta, PreloadArticleResult, TempArticleId},
};

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    // Load configuration
    let system_config = SystemConfig::load_me_instead();
    let service_config = ServiceConfig { worker_count: 4 };

    // Start the worker
    let ctx = WorkerCtx {
        redis: JobWorkerRx::new(
            redis::init_redis_client(system_config.preload_article_worker_redis_url.as_ref())
                .await
                .expect("Failed to initialize Redis client"),
            PRELOAD_ARTICLE_WORKER_KEY,
        ),
    };
    let workers: Box<[_]> = (0..service_config.worker_count)
        .map(|_| {
            let ctx = ctx.clone();
            tokio::spawn(async move {
                run_worker(ctx).await;
            })
        })
        .collect();

    future::join_all(workers).await;
}

async fn run_worker(ctx: WorkerCtx) {
    loop {
        let Ok(job) = ctx.redis.fetch_next_job().await else {
            panic!("Failed to fetch job from Redis"); // TODO: handle errors properly
        };

        // TODO download and parse the article at job.0

        let result = PreloadArticleResult::Success(ArticleMeta {
            title: "pretending to have a backend".into(),
            tags: Box::new(["example".into(), "demo".into()]),
        });

        ctx.redis
            .store_result(job, result)
            .await
            .expect("Failed to store job result"); // TODO: handle errors properly
    }
}

#[derive(Clone)]
struct ServiceConfig {
    worker_count: usize,
}

#[derive(Clone)]
struct WorkerCtx {
    redis: JobWorkerRx<TempArticleId, PreloadArticleResult>,
}
