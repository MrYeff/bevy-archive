use fred::prelude::*;
use std::time::Duration;

pub mod job_worker;

pub async fn init_redis_client(url: &str) -> Result<Client, Error> {
    let config = Config::from_url(url)?;

    let client = Builder::from_config(config)
        .with_connection_config(|config| {
            config.connection_timeout = Duration::from_secs(5);
            config.tcp = TcpConfig {
                nodelay: Some(true),
                ..Default::default()
            };
        })
        .build()?;

    client.init().await?;
    Ok(client)
}
