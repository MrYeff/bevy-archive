use derive_more::Display;
use std::sync::Arc;

#[derive(Clone, Copy, Debug, Display, Hash, Eq, PartialEq)]
#[repr(u32)]
pub enum RedisDesignation {
    RenderWorkerCache,
    PreloadArticleWorkerCache,
}

impl RedisDesignation {
    pub const fn as_u32(&self) -> u32 {
        *self as u32
    }

    pub const fn from_u32(value: u32) -> Option<Self> {
        match value {
            0 => Some(RedisDesignation::RenderWorkerCache),
            1 => Some(RedisDesignation::PreloadArticleWorkerCache),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct RedisAccess {
    pub url: Arc<str>,
    pub base_key: Arc<str>,
    pub auth: Option<RedisAuth>,
}

#[derive(Clone, Debug)]
pub struct RedisAuth {
    pub user: Arc<str>,
    pub password: Arc<str>,
}
