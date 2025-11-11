use std::{marker::PhantomData, sync::Arc};

use fred::prelude::*;
use serde::{Serialize, de::DeserializeOwned};

#[derive(Clone)]
pub struct JobWorkerTx<JID, JResult> {
    _phantom: PhantomData<fn() -> (JID, JResult)>,
    base_key: Arc<str>,
    redis: Client,
}

const JOB_QUEUE_SUFFIX: &str = "jobs"; // LIST (queue) of serialized JID
const JOB_DEDUP_SET_SUFFIX: &str = "jobs:seen"; // SET  (enqueued ids)
const RESULT_HASH_SUFFIX: &str = "results"; // HASH (JID -> result)

// ---------------- TX (producer / API) ----------------

impl<JID, JResult> JobWorkerTx<JID, JResult>
where
    JID: ToString + Serialize,
    JResult: DeserializeOwned,
{
    pub fn new(redis: Client, base_key: impl Into<Arc<str>>) -> Self {
        Self {
            _phantom: PhantomData,
            base_key: base_key.into(),
            redis,
        }
    }

    /// Deduped enqueue of just the job_id:
    /// - SADD {base}:jobs:seen {job_id} (if new)
    /// - LPUSH {base}:jobs <serde_json(JID)>
    pub async fn schedule_job(&self, job_id: JID) -> Result<(), Error> {
        let id_str = job_id.to_string();
        let dedup_key = format!("{}:{}", self.base_key, JOB_DEDUP_SET_SUFFIX);
        let queue_key = format!("{}:{}", self.base_key, JOB_QUEUE_SUFFIX);

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
        let result_key = format!("{}:{}", self.base_key, RESULT_HASH_SUFFIX);
        let field = job_id.to_string();

        let raw: Option<Vec<u8>> = self.redis.hget(&result_key, field).await?;
        Ok(raw.map(|bytes| serde_json::from_slice(&bytes).expect("deserialize result")))
    }
}

// ---------------- RX (worker side) ----------------

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
    pub fn new(redis: Client, base_key: impl Into<Arc<str>>) -> Self {
        Self {
            _phantom: PhantomData,
            base_key: base_key.into(),
            redis,
        }
    }

    /// Block for the next job; returns the deserialized JID.
    pub async fn fetch_next_job(&self) -> Result<JID, Error> {
        let queue_key = format!("{}:{}", self.base_key, JOB_QUEUE_SUFFIX);
        let (_key, payload): (String, Vec<u8>) = self.redis.brpop(&queue_key, 0.0).await?;
        let job_id = serde_json::from_slice::<JID>(&payload).expect("deserialize job id");
        Ok(job_id)
    }

    /// Store result and clear dedupe mark so the id can be re-queued later.
    pub async fn store_result(&self, job_id: JID, result: JResult) -> Result<(), Error> {
        let result_key = format!("{}:{}", self.base_key, RESULT_HASH_SUFFIX);
        let dedup_key = format!("{}:{}", self.base_key, JOB_DEDUP_SET_SUFFIX);

        let field = job_id.to_string();
        let payload = serde_json::to_vec(&result).expect("serialize result");

        let _: i64 = self.redis.hset(&result_key, (field, payload)).await?;
        let _: i64 = self.redis.srem(&dedup_key, job_id.to_string()).await?;
        Ok(())
    }
}
