use fred::prelude::*;
use shared::prelude::RedisAccess;
use std::time::Duration;

pub async fn init_redis_client(cfg: &RedisAccess) -> Result<Client, Error> {
    let mut config = Config::from_url(&cfg.url)?;

    if let Some(auth) = &cfg.auth {
        config.username = Some(auth.user.to_string());
        config.password = Some(auth.password.to_string());
    }

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
