use crate::redis::{RedisAccess, RedisAuth, RedisDesignation};
use clap::Parser;
use std::sync::Arc;

#[derive(Debug, Parser, Clone)]
pub struct EndpointArgs {
    #[arg(long, short = 'p', default_value_t = 8080)]
    pub port: u16,
}

#[derive(Debug, Parser, Clone)]
pub struct WorkerArgs {
    #[arg(long, short = 'w', default_value_t = 4)]
    pub workers: u16,
}

#[derive(Debug, Parser, Clone)]
pub struct PgAccesArgs {
    #[arg(long = "pg-host", default_value = "localhost", env = "PG_HOST")]
    pub host: String,
    #[arg(long = "pg-port", env = "PG_PORT")]
    pub port: u16,
    #[arg(long = "pg-dbname", env = "PG_DBNAME")]
    pub dbname: String,
    #[arg(long = "pg-user", env = "PG_USER")]
    pub user: String,
    #[arg(long = "pg-password", env = "PG_PASSWORD")]
    pub password: String,
}

#[derive(Debug, Parser, Clone)]
pub struct RedisAccessArgs<const DESIGNATION: u32> {
    /// REDIS_<DESIGNATION>_HOST
    #[arg(env = rd_as_env(DESIGNATION, "host"))]
    pub host: String,
    /// REDIS_<DESIGNATION>_PORT
    #[arg(env = rd_as_env(DESIGNATION, "port"))]
    pub port: u16,
    /// REDIS_<DESIGNATION>_BASE_KEY
    #[arg(env = rd_as_env(DESIGNATION, "base_key"))]
    pub base_key: String,
    #[command(flatten)]
    pub auth: Option<RedisAuthArgs<DESIGNATION>>,
}

#[derive(Debug, Parser, Clone)]
pub struct RedisAuthArgs<const DESIGNATION: u32> {
    /// REDIS_<DESIGNATION>_USER
    #[arg(env = rd_as_env(DESIGNATION, "user"))]
    pub user: String,
    /// REDIS_<DESIGNATION>_PASSWORD
    #[arg(env = rd_as_env(DESIGNATION, "password"))]
    pub password: String,
}

fn rd_as_env(value: u32, param: &str) -> String {
    let Some(designation) = RedisDesignation::from_u32(value) else {
        panic!("invalid redis designation for env var");
    };

    format!(
        "REDIS_{}_{}",
        designation.to_string().to_uppercase(),
        param.to_uppercase()
    )
}

impl<const DESIGNATION: u32> From<&RedisAccessArgs<DESIGNATION>> for RedisAccess {
    fn from(args: &RedisAccessArgs<DESIGNATION>) -> Self {
        let url = format!("redis://{}:{}", args.host, args.port);

        Self {
            url: Arc::from(url),
            base_key: Arc::from(args.base_key.clone()),
            auth: args.auth.as_ref().map(|a| a.into()),
        }
    }
}

impl<const DESIGNATION: u32> From<&RedisAuthArgs<DESIGNATION>> for RedisAuth {
    fn from(args: &RedisAuthArgs<DESIGNATION>) -> Self {
        Self {
            user: Arc::from(args.user.clone()),
            password: Arc::from(args.password.clone()),
        }
    }
}
