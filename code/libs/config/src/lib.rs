mod redis;

use serde::Deserialize;

pub use redis::*;
use thiserror::Error;

#[derive(Deserialize)]
pub struct SystemConfig {
    pub redis: RedisConfig,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Redis config error: {0}")]
    RedisConfigError(RedisConfigError),
}
