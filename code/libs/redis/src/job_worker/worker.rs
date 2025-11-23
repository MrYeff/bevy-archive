use crate::job_worker::JobWorkerRx;
use futures::{StreamExt, stream::FuturesUnordered};
use serde::{Serialize, de::DeserializeOwned};
use shared::{prelude::WorkerArgs, redis::RedisAccess};
use std::{future::Future, sync::Arc};

pub struct JobWorker<Jid, JResult, ProcessFn, State> {
    rx: JobWorkerRx<Jid, JResult>,
    cfg: WorkerArgs,
    process_job: ProcessFn,
    _marker: std::marker::PhantomData<State>,
}

impl<Jid, JResult, ProcessFn, State, Fut> JobWorker<Jid, JResult, ProcessFn, State>
where
    // Jid is owned (we pull it from Redis), but we also need to clone it once per job
    Jid: ToString + DeserializeOwned + Send + Clone + 'static,
    JResult: Serialize + Send + 'static,
    State: Send + Sync + 'static,
    // process_job owns Jid and gets an Arc<State>, returns a statically-typed future
    ProcessFn: Fn(Jid, Arc<State>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = JResult> + Send + 'static,
{
    pub async fn new(
        redis_access: impl Into<RedisAccess>,
        cfg: WorkerArgs,
        process_job: ProcessFn,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            rx: JobWorkerRx::new(redis_access).await?,
            cfg,
            process_job,
            _marker: std::marker::PhantomData,
        })
    }

    pub async fn run(&self, state: Arc<State>) -> anyhow::Result<()> {
        let mut in_flight = FuturesUnordered::new();

        loop {
            tokio::select! {
                Some(res) = in_flight.next(), if !in_flight.is_empty() => {
                    res?;
                },
                job_res = self.rx.fetch_next_job(), if in_flight.len() < self.cfg.workers as usize => {
                    let job = job_res?;
                    let state_for_job = Arc::clone(&state);
                    in_flight.push(self.process_one(job, state_for_job));
                }
            }
        }
    }

    async fn process_one(&self, job: Jid, state: Arc<State>) -> anyhow::Result<()> {
        let result = (self.process_job)(job.clone(), state).await;
        self.rx.store_result(job, result).await?;
        Ok(())
    }
}
