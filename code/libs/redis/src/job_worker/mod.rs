mod rx;
mod tx;
mod worker;

pub use rx::JobWorkerRx;
pub use tx::JobWorkerTx;
pub use worker::JobWorker;

const JOB_QUEUE_SUFFIX: &str = "jobs"; // LIST (queue) of serialized JID
const JOB_DEDUP_SET_SUFFIX: &str = "jobs:seen"; // SET  (enqueued ids)
const RESULT_HASH_SUFFIX: &str = "results"; // HASH (JID -> result)
