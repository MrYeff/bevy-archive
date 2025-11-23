use crate::utils::init_redis_client;
use fred::prelude::*;
use serde::{Serialize, de::DeserializeOwned};
use shared::redis::RedisAccess;
use std::{marker::PhantomData, sync::Arc};

#[derive(Clone)]
pub struct JobWorkerRx<JID, JResult> {
    _phantom: PhantomData<fn() -> (JID, JResult)>,
    base_key: Arc<str>,
    redis: Client,
}

impl<JID, JResult> JobWorkerRx<JID, JResult>
where
    JID: ToString + DeserializeOwned,
    JResult: Serialize,
{
    pub async fn new(redis_access: impl Into<RedisAccess>) -> Result<Self, Error> {
        let cfg = redis_access.into();
        Ok(Self {
            _phantom: PhantomData,
            redis: init_redis_client(&cfg).await?,
            base_key: cfg.base_key.clone(),
        })
    }

    /// Block for the next job; returns the deserialized JID.
    pub async fn fetch_next_job(&self) -> Result<JID, Error> {
        let queue_key = format!("{}:{}", self.base_key, super::JOB_QUEUE_SUFFIX);
        let (_key, payload): (String, Vec<u8>) = self.redis.brpop(&queue_key, 0.0).await?;
        let job_id = serde_json::from_slice::<JID>(&payload).expect("deserialize job id");
        Ok(job_id)
    }

    /// Store result and clear dedupe mark so the id can be re-queued later.
    pub async fn store_result(&self, job_id: JID, result: JResult) -> Result<(), Error> {
        let result_key = format!("{}:{}", self.base_key, super::RESULT_HASH_SUFFIX);
        let dedup_key = format!("{}:{}", self.base_key, super::JOB_DEDUP_SET_SUFFIX);

        let field = job_id.to_string();
        let payload = serde_json::to_vec(&result).expect("serialize result");

        let _: i64 = self.redis.hset(&result_key, (field, payload)).await?;
        let _: i64 = self.redis.srem(&dedup_key, job_id.to_string()).await?;
        Ok(())
    }
}
