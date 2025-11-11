mod types;
use std::sync::Arc;

pub use types::*;

pub mod article;
pub mod article_preloading;

pub struct SystemConfig {
    // --- Endpoints ---
    pub main_endpoint_url: Arc<str>,

    // -- Redis URLs ---
    pub render_worker_redis_url: Arc<str>,
    pub preload_article_worker_redis_url: Arc<str>,
}

impl SystemConfig {
    pub fn load_me_instead() -> Self {
        Self {
            main_endpoint_url: Arc::from("http://localhost:3000"),
            render_worker_redis_url: Arc::from("redis://localhost:5000/"),
            preload_article_worker_redis_url: Arc::from("redis://localhost:5001/"),
        }
    }
}
