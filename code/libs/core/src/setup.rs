use config::SystemConfig;
use std::path::PathBuf;
use thiserror::Error;

pub fn load_config(cfg: PathBuf) -> Result<SystemConfig, SetupError> {
    let cfg_contents = std::fs::read_to_string(&cfg)?;
    let config: SystemConfig = serde_json::from_str(&cfg_contents)?;

    Ok(config)
}

#[derive(Error, Debug)]
pub enum SetupError {
    #[error("File read error: {0}")]
    FileReadError(std::io::Error),
    #[error("Config parse error: {0}")]
    ConfigParseError(serde_json::Error),
}

impl From<std::io::Error> for SetupError {
    fn from(value: std::io::Error) -> Self {
        SetupError::FileReadError(value)
    }
}

impl From<serde_json::Error> for SetupError {
    fn from(value: serde_json::Error) -> Self {
        SetupError::ConfigParseError(value)
    }
}
