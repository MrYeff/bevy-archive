use crate::utils::init_redis_client;
use fred::prelude::*;
use serde::{Serialize, de::DeserializeOwned};
use shared::redis::RedisAccess;
use std::{marker::PhantomData, sync::Arc};

#[derive(Clone)]
pub struct JobWorkerTx<JID, JResult> {
    _phantom: PhantomData<fn() -> (JID, JResult)>,
    base_key: Arc<str>,
    redis: Client,
}

impl<JID, JResult> JobWorkerTx<JID, JResult>
where
    JID: ToString + Serialize,
    JResult: DeserializeOwned,
{
    pub async fn new(redis_access: impl Into<RedisAccess>) -> Result<Self, Error> {
        let cfg = redis_access.into();
        Ok(Self {
            _phantom: PhantomData,
            redis: init_redis_client(&cfg).await?,
            base_key: cfg.base_key.clone(),
        })
    }

    /// Deduped enqueue of the job_id
    pub async fn schedule_job(&self, job_id: JID) -> Result<(), Error> {
        let id_str = job_id.to_string();
        let dedup_key = format!("{}:{}", self.base_key, super::JOB_DEDUP_SET_SUFFIX);
        let queue_key = format!("{}:{}", self.base_key, super::JOB_QUEUE_SUFFIX);

        let added: i64 = self.redis.sadd(&dedup_key, id_str).await?;
        if added == 0 {
            // already queued/in-flight
            return Ok(());
        }

        let payload = serde_json::to_vec(&job_id).expect("serialize job id");
        let _: i64 = self.redis.lpush(&queue_key, payload).await?;
        Ok(())
    }

    /// Results live in HASH {base}:results field={job_id} value=serde_json(JResult)
    pub async fn try_fetch_result(&self, job_id: JID) -> Result<Option<JResult>, Error> {
        let result_key = format!("{}:{}", self.base_key, super::RESULT_HASH_SUFFIX);
        let field = job_id.to_string();

        let raw: Option<Vec<u8>> = self.redis.hget(&result_key, field).await?;
        Ok(raw.map(|bytes| serde_json::from_slice(&bytes).expect("deserialize result")))
    }
}
